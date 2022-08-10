use crate::graph::hash_table::{HashTable, VertexId};
use crate::scc::strongly_connected_components::StronglyConnectedComponents;
use std::collections::HashSet;

pub struct Tarjan<'a> {
  graph: &'a HashTable,
}

impl<'a> Tarjan<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl<'a> StronglyConnectedComponents for Tarjan<'a> {
  fn strongly_connected_components(&self) -> HashSet<VertexId> {
    HashSet::from_iter(self.graph.vertices())
  }
}

#[cfg(test)]
mod tests {
  use crate::scc::strongly_connected_components::StronglyConnectedComponents;
  use crate::scc::tarjan::Tarjan;
  use crate::tools::graphs::graph_with_simple_clique;
  use std::collections::HashSet;

  #[test]
  fn works_on_simple_clique() {
    let clique = graph_with_simple_clique();
    let sc_components = Tarjan { graph: &clique }.strongly_connected_components();

    assert_eq!(sc_components.len(), clique.vertices().len());
    assert_eq!(sc_components, HashSet::from_iter(clique.vertices()));
  }
}
