use libp2p::{gossipsub, identify, identity::Keypair, mdns, swarm::NetworkBehaviour};
use std::time::Duration;

/// This macro will create a `ChatBehaviourEvent` type that swarm will emit in a stream.
#[derive(NetworkBehaviour)]
pub struct ChatBehaviour {
    pub(crate) gossipsub: gossipsub::Behaviour,
    pub(crate) mdns: mdns::tokio::Behaviour,
    pub(crate) identify: identify::Behaviour,
}

/// A generic error type for the chat behaviour.
#[derive(Debug, thiserror::Error)]
pub enum ChatBehaviourError {
    #[error("Could not create MDNS behaviour: {0}")]
    MDNS(std::io::Error),
    #[error("Could not create GossipSub config: {0}")]
    GossipsubConfig(gossipsub::ConfigBuilderError),
    #[error("Could not create GossipSub behaviour: {0}")]
    Gossipsub(&'static str),
}

impl ChatBehaviour {
    /// identify protocol string, looks like `chat/{major}.{minor}`
    pub const PROTOCOL_VERSION: &str = concat!(
        "chat/",
        env!("CARGO_PKG_VERSION_MAJOR"),
        ".",
        env!("CARGO_PKG_VERSION_MINOR")
    );

    pub fn new(key: Keypair) -> Result<Self, ChatBehaviourError> {
        Ok(ChatBehaviour {
            identify: identify_behaviour(&key),
            mdns: mdns_behaviour(&key)?,
            gossipsub: gossipsub_behaviour(key)?,
        })
    }
}

#[inline(always)]
fn gossipsub_behaviour(keypair: Keypair) -> Result<gossipsub::Behaviour, ChatBehaviourError> {
    use gossipsub::{Behaviour, ConfigBuilder, ValidationMode};
    use gossipsub::{Message, MessageAuthenticity, MessageId};

    // if two messages have the same id they wont be published again
    // so we simply use timestamp nanos as the message id
    let message_id_fn = |_: &Message| {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        MessageId::from(nanos.to_ne_bytes())
    };

    let gossipsub_config = ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()
        .map_err(ChatBehaviourError::GossipsubConfig)?;

    Behaviour::new(MessageAuthenticity::Signed(keypair), gossipsub_config)
        .map_err(ChatBehaviourError::Gossipsub)
}

#[inline(always)]
fn mdns_behaviour(keypair: &Keypair) -> Result<mdns::tokio::Behaviour, ChatBehaviourError> {
    use mdns::{Config, tokio::Behaviour};

    Behaviour::new(Config::default(), keypair.public().to_peer_id())
        .map_err(ChatBehaviourError::MDNS)
}

#[inline(always)]
fn identify_behaviour(keypair: &Keypair) -> identify::Behaviour {
    use identify::{Behaviour, Config};

    let config = Config::new(ChatBehaviour::PROTOCOL_VERSION.into(), keypair.public());
    Behaviour::new(config)
}
