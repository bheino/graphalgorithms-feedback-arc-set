use crate::graph::hash_table::VertexId;
use std::collections::HashSet;

pub trait StronglyConnectedComponents<'a> {
  fn strongly_connected_components(&'a mut self) -> HashSet<VertexId>;
}

#[cfg(test)]
pub mod tests {}
