mod behaviour;
pub use behaviour::{ChatBehaviour, ChatBehaviourError, ChatBehaviourEvent};

mod client;
pub use client::{ChatClient, ChatClientError};

#[cfg(feature = "ffi")]
mod external;
#[cfg(feature = "ffi")]
pub use external::*;
