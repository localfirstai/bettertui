use std::fmt;

use super::node_id::NodeId;

/// Errors that can occur during tree operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeError {
    /// Node with the given ID was not found in the arena.
    NodeNotFound(NodeId),
    /// A cycle would be created by this operation.
    CycleDetected { node: NodeId, ancestor: NodeId },
    /// The operation is invalid for some other reason.
    InvalidOperation(String),
}

impl fmt::Display for TreeError {
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
        }
    }
}

impl std::error::Error for TreeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = TreeError::NodeNotFound(NodeId::default());
        assert!(format!("{err}").contains("Node not found"));

        let err = TreeError::InvalidOperation("test".into());
        assert!(format!("{err}").contains("Invalid operation"));
    }
}
