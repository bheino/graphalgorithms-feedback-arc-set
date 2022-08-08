use crate::graph::hash_table::{Direction, HashTable, VertexId};

pub struct TopologicalSort<'a> {
  graph: &'a HashTable,
}

impl<'a> TopologicalSort<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }

  pub fn sort_by_indegree_asc(&self) -> Vec<VertexId> {
    let mut vec = self
      .graph
      .vertices()
      .iter()
      .map(|v| (*v, self.graph.edges(*v, Direction::Inbound).len()))
      .collect::<Vec<_>>();

    vec.sort_by(|v1, v2| (*v1).1.cmp(&(*v2).1));
    vec.iter().map(|pair| pair.0).collect()
  }
}

#[cfg(test)]
mod tests {
  use crate::graph::hash_table::{Edge, HashTable};
  use crate::ordering::topological_sort::TopologicalSort;

  #[test]
  fn works_on_simple_clique() {
    let edges = [(0, 1), (1, 2), (2, 0)];
    let clique = HashTable::from_edges(&edges);

    let order = TopologicalSort::new(&clique).sort_by_indegree_asc();

    assert_eq!(order.len(), 3);
    // TODO Prüfen, ob aufsteigende Reihenfolge
  }
}
