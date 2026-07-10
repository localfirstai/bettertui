use slotmap::DefaultKey;

/// Uniquely identifies a node in the arena.
///
/// Uses generational indices via `slotmap::DefaultKey` to prevent use-after-free.
/// If a node is removed and a new node allocated at the same index,
/// the generation mismatch catches stale references.
///
/// Size: 8 bytes (two u32 values). Stack-allocated. O(1) comparison.
pub type NodeId = DefaultKey;
