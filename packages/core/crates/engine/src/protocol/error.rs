use std::fmt;

use crate::tree::NodeId;

/// Errors that can occur during command processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    /// Node with the given ID was not found.
    NodeNotFound(NodeId),
    /// A cycle would be created by this operation.
    CycleDetected { node: NodeId, ancestor: NodeId },
    /// The operation is invalid for some other reason.
    InvalidOperation(String),
    /// The command is not valid in the current state.
    InvalidState(String),
    /// The command references a node that was already removed.
    StaleReference(NodeId),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeNotFound(id) => write!(f, "Node not found: {id:?}"),
            Self::CycleDetected { node, ancestor } => {
                write!(
                    f,
                    "Cycle detected: node {node:?} is ancestor of {ancestor:?}"
                )
            }
            Self::InvalidOperation(msg) => write!(f, "Invalid operation: {msg}"),
            Self::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
            Self::StaleReference(id) => write!(f, "Stale reference: {id:?}"),
        }
    }
}

impl std::error::Error for CommandError {}

impl From<crate::tree::TreeError> for CommandError {
    fn from(err: crate::tree::TreeError) -> Self {
        match err {
            crate::tree::TreeError::NodeNotFound(id) => Self::NodeNotFound(id),
            crate::tree::TreeError::CycleDetected { node, ancestor } => {
                Self::CycleDetected { node, ancestor }
            }
            crate::tree::TreeError::InvalidOperation(msg) => Self::InvalidOperation(msg),
        }
    }
}

/// Non-fatal warnings that can occur during command processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandWarning {
    /// A command targeted a node that doesn't exist (skipped).
    NodeSkipped(NodeId),
    /// A style property was set but has no effect in the current context.
    NoEffect(String),
    /// A command was redundant (e.g., setting the same value twice).
    Redundant(String),
    /// A deprecated command was used.
    Deprecated(String),
}

impl fmt::Display for CommandWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeSkipped(id) => write!(f, "Node skipped: {id:?}"),
            Self::NoEffect(msg) => write!(f, "No effect: {msg}"),
            Self::Redundant(msg) => write!(f, "Redundant: {msg}"),
            Self::Deprecated(msg) => write!(f, "Deprecated: {msg}"),
        }
    }
}

impl std::error::Error for CommandWarning {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = CommandError::NodeNotFound(NodeId::default());
        assert!(format!("{err}").contains("Node not found"));

        let err = CommandError::InvalidOperation("test".into());
        assert!(format!("{err}").contains("Invalid operation"));
    }

    #[test]
    fn warning_display() {
        let warn = CommandWarning::NodeSkipped(NodeId::default());
        assert!(format!("{warn}").contains("Node skipped"));

        let warn = CommandWarning::NoEffect("test".into());
        assert!(format!("{warn}").contains("No effect"));
    }

    #[test]
    fn from_tree_error() {
        let tree_err = crate::tree::TreeError::NodeNotFound(NodeId::default());
        let cmd_err: CommandError = tree_err.into();
        assert!(matches!(cmd_err, CommandError::NodeNotFound(_)));
    }
}
