use crate::graph::hash_table::VertexId;
use std::collections::HashSet;

pub trait StronglyConnectedComponents {
  fn strongly_connected_components(&mut self) -> Vec<HashSet<VertexId>>;
}

#[cfg(test)]
pub mod tests {}
