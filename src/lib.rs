mod behaviour;
pub use behaviour::{ChatBehaviour, ChatBehaviourEvent};

mod client;
pub use client::{ChatClient, ChatClientError};

#[cfg(feature = "ffi")]
mod external;
pub use external::*;
