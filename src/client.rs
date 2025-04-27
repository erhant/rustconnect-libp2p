use crate::{ChatBehaviour, ChatBehaviourEvent};
use futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{PeerId, TransportError, gossipsub, mdns, swarm};
use libp2p::{noise, tcp, yamux};
use std::collections::VecDeque;
use std::io;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub struct ChatClient {
    /// The underlying [`swarm`] instance
    swarm: swarm::Swarm<ChatBehaviour>,
    /// Cancellation token to stop the client.
    cancellation: CancellationToken,
    /// Message queue to store the messages received.
    pub received: VecDeque<(PeerId, String)>,
    /// Channel to send messages to the client.
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
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_behaviour(|key| Ok(ChatBehaviour::new(key.clone()).unwrap()))
            .unwrap()
            .build();

        let (sender, receiver) = mpsc::unbounded_channel();
        Ok((
            Self {
                swarm,
                cancellation,
                received: Default::default(),
                sender_channel: receiver,
            },
            sender,
        ))
    }

    /// Publish a message to the chat topic.
    pub fn publish(&mut self, message: impl Into<Vec<u8>>) -> Result<(), ChatClientError> {
        let topic = gossipsub::IdentTopic::new(Self::CHAT_TOPIC);

        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, message)
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
                        tracing::error!("Error while publishing: {e}");
                    }
                }

                // handle events
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            tracing::info!("mDNS discovered a new peer: {peer_id}");
                            // add explicitly to ALWAYS send to this peer
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            tracing::info!("mDNS discover peer has expired: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message,
                        ..
                    })) => {
                        let message_str = String::from_utf8_lossy(&message.data);
                        tracing::info!("{peer_id}: {message_str}");

                        // store the message in history
                        self.received.push_back((peer_id, message_str.to_string()));
                    },
                    SwarmEvent::NewListenAddr { address, .. } => {
                        tracing::info!("Local node is listening on {address}");
                    },
                    SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        tracing::info!("Connected closed with {peer_id}");
                    },
                    _ => {
                        tracing::trace!("Unhandled event: {event:?}");
                    }
                }
            }
        }

        self.stop();

        Ok(())
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
                format!("/ip4/0.0.0.0/udp/{port}/quic-v1")
                    .parse()
                    .expect("should parse"),
            )
            .map_err(ChatClientError::ListenError)?;
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
    }
}
