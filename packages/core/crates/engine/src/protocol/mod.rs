//! Command protocol layer for communicating with the rendering engine.
//! Defines commands, buffers, processing, and error types.

pub mod buffer;
pub mod command;
mod command_ext;
pub mod error;
pub mod processor;
pub mod result;

pub use buffer::CommandBuffer;
pub use command::Command;
pub use command_ext::{CommandEntry, CommandRegistry, RegistryResult};
pub use error::{CommandError, CommandWarning};
pub use processor::CommandProcessor;
pub use result::CommandResult;
