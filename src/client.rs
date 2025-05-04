use crate::{ChatBehaviour, ChatBehaviourEvent};
use futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{PeerId, TransportError, gossipsub, identify, mdns, swarm};
use libp2p::{noise, tcp, yamux};
use std::collections::VecDeque;
use std::io;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// The main client struct that handles the chat functionality.
///
/// - Shall be started with [`Self::run`] and will listen for incoming messages.
/// - All received messages will be stored in the [`Self::received`] queue.
/// - Can be stopped gracefully with [`Self::cancel`].
pub struct ChatClient {
    /// The underlying [`swarm`] instance
    swarm: swarm::Swarm<ChatBehaviour>,
    /// Cancellation token to stop the client.
    cancellation: CancellationToken,
    /// Message queue to store the messages received.
    pub received: VecDeque<(PeerId, String)>,
    /// Channel to send messages to this client.
    sender_channel: mpsc::UnboundedReceiver<String>,
}

/// A generic error type for the chat client.
#[derive(Debug, thiserror::Error)]
pub enum ChatClientError {
    #[error("Could not subscribe: {0}")]
    SubscribtionError(gossipsub::SubscriptionError),
    #[error("Could not listen: {0}")]
    ListenError(TransportError<io::Error>),
    #[error("Could not publish: {0}")]
    PublishError(gossipsub::PublishError),
}

impl ChatClient {
    /// Gosipsub topic name for chatting.
    pub const CHAT_TOPIC: &'static str = "rustconnect";

    /// Creates a new client instance listening on `0.0.0.0:{port}`.
    ///
    /// Use `0` to let the OS assign a port.
    ///
    /// Returns a channel to send messages to the client.
    pub fn new(
        cancellation: CancellationToken,
    ) -> eyre::Result<(Self, mpsc::UnboundedSender<String>)> {
        // here we generate a new identity for the client, but we could take it from outside too
        let keypair = libp2p::identity::Keypair::generate_ed25519();

        let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new, // uses existing keypair for Noise protocol
                yamux::Config::default,
            )?
            .with_behaviour(|key| Ok(ChatBehaviour::new(key.clone()).unwrap()))
            .unwrap()
            .build();

        let (sender, receiver) = mpsc::unbounded_channel();
        Ok((
            Self {
                swarm,
                cancellation,
                received: Default::default(),
                // the "receiver" of this channel will be the channel used by "sender"
                sender_channel: receiver,
            },
            sender,
        ))
    }

    /// Publish a message to the chat topic.
    pub fn publish(&mut self, message: impl AsRef<[u8]>) -> Result<(), ChatClientError> {
        let topic = gossipsub::IdentTopic::new(Self::CHAT_TOPIC);

        // append timestamp to message
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut data = Vec::new();
        data.extend_from_slice(&nanos.to_ne_bytes());
        data.extend_from_slice(message.as_ref());

        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, data)
            .map_err(ChatClientError::PublishError)?;

        Ok(())
    }

    pub async fn run(&mut self, port: u16) -> Result<(), ChatClientError> {
        // start the client
        self.start(port)?;

        loop {
            tokio::select! {
                // check for cancellation
                _ = self.cancellation.cancelled() => break,

                // check for messages to send
                Some(message) = self.sender_channel.recv() => {
                    if message.is_empty() {
                        continue;
                    }

                    // publish the message
                    if let Err(e) = self.publish(message) {
                        log::error!("Error while publishing: {e}");
                    }
                }

                // handle events
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(event)) => self.handle_mdns(event),
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Identify(event)) => self.handle_identify(event),
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(event)) => self.handle_gossipsub(event),
                    SwarmEvent::NewListenAddr { address, .. } => {
                        log::info!("Local node is listening on {address}");
                    },
                    SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        log::info!("Connected closed with {peer_id}");
                    },
                    _ => {
                        log::trace!("Unhandled event: {event:?}");
                    }
                }
            }
        }

        self.stop();

        Ok(())
    }

    #[inline]
    fn handle_mdns(&mut self, event: mdns::Event) {
        match event {
            mdns::Event::Discovered(peers) => {
                use libp2p::swarm::DialError;

                for (peer_id, _multiaddr) in peers {
                    log::info!("mDNS discovered a new peer: {peer_id}");
                    // we dont add it yet, we instead wait for the identify event
                    match self.swarm.dial(peer_id) {
                        Ok(_) => { /* do nothing */ }
                        Err(err) => match err {
                            DialError::DialPeerConditionFalse(_) => { /* do nothing */ }
                            err => {
                                log::error!("Could not dial peer {peer_id}: {err}");
                            }
                        },
                    }
                }
            }
            mdns::Event::Expired(peers) => {
                for (peer_id, _multiaddr) in peers {
                    log::info!("mDNS discover peer has expired: {peer_id}");
                    if let Err(_) = self.swarm.disconnect_peer_id(peer_id) {
                        log::error!("Could not disconnect peer {peer_id}");
                    }
                }
            }
        }
    }

    #[inline]
    fn handle_identify(&mut self, event: identify::Event) {
        match event {
            identify::Event::Received { peer_id, info, .. } => {
                log::info!("Identified peer {peer_id}!");
                if info.protocol_version != ChatBehaviour::PROTOCOL_VERSION {
                    log::warn!(
                        "Peer {peer_id} is using a different protocol version: {}, disconnecting.",
                        info.protocol_version
                    );
                    let _ = self.swarm.disconnect_peer_id(peer_id);
                } else {
                    log::debug!("Adding peer {peer_id} to gossipsub explicit peers");
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .add_explicit_peer(&peer_id);
                }
            }
            _ => {
                log::trace!("Unhandled identify event: {event:?}");
            }
        }
    }

    #[inline]
    fn handle_gossipsub(&mut self, event: gossipsub::Event) {
        match event {
            gossipsub::Event::Message {
                message_id,
                message,
                propagation_source: peer_id,
            } => {
                // extract nanoseconds from the message
                let _ =
                    u64::from_ne_bytes(message.data[0..8].try_into().expect("should be 8 bytes"));
                let message_str = String::from_utf8_lossy(&message.data[8..]);
                log::debug!("Gossipsub message received: {message_id:?}");
                log::info!("Message from {peer_id}:\n{message_str}");

                // store the message in history
                self.received.push_back((peer_id, message_str.to_string()));
            }
            _ => {
                log::trace!("Unhandled gossipsub event: {event:?}");
            }
        }
    }

    /// Triggers the cancellation token, which will stop the client and all other tasks
    /// that may be waiting for this cancellation.
    #[inline]
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }

    /// Starts the client by subscribing to the chat topic and listening.
    ///
    /// Can be inlined as its only called once.
    #[inline]
    fn start(&mut self, port: u16) -> Result<(), ChatClientError> {
        log::info!("Starting client on port {port}");
        // subscribe
        let topic = gossipsub::IdentTopic::new(Self::CHAT_TOPIC);
        self.swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&topic)
            .map_err(ChatClientError::SubscribtionError)?;

        // listen on all interfaces and whatever port the OS assigns
        self.swarm
            .listen_on(
                format!("/ip4/0.0.0.0/tcp/{port}")
                    .parse()
                    .expect("should parse"),
            )
            .map_err(ChatClientError::ListenError)?;

        Ok(())
    }

    /// Starts the client by subscribing to the chat topic and listening.
    ///
    /// Can be inlined as its only called once.
    #[inline]
    fn stop(&mut self) {
        // unsubscribe
        let topic = gossipsub::IdentTopic::new(Self::CHAT_TOPIC);
        self.swarm.behaviour_mut().gossipsub.unsubscribe(&topic);

        // close channel
        self.sender_channel.close();

        log::info!("Client stopped");
    }
}
