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
  use crate::graph::hash_table::{Direction, HashTable, VertexId};
  use crate::ordering::topological_sort::TopologicalSort;
  use crate::tools::graphs::{graph_from_file, graph_with_multiple_cliques};

  #[test]
  fn works_on_simple_clique() {
    let edges = [(0, 1), (1, 2), (2, 0)];
    let clique = HashTable::from_edges(&edges);

    let order = TopologicalSort::new(&clique).sort_by_indegree_asc();

    assert_eq!(order.len(), 3);
    assert_indegree_increasing(clique, order);
  }

  #[test]
  fn works_on_multiple_cliques() {
    let cyclic_graph = graph_with_multiple_cliques();
    let order = TopologicalSort::new(&cyclic_graph).sort_by_indegree_asc();

    assert_eq!(order.len(), 19);
    assert_indegree_increasing(cyclic_graph, order);
  }

  #[test]
  fn works_on_h_001() {
    let cyclic_graph = graph_from_file("h_001");
    let order = TopologicalSort::new(&cyclic_graph).sort_by_indegree_asc();

    assert_eq!(order.len(), 1024);
    assert_indegree_increasing(cyclic_graph, order);
  }

  #[test]
  fn works_on_h_025() {
    let cyclic_graph = graph_from_file("h_025");
    let order = TopologicalSort::new(&cyclic_graph).sort_by_indegree_asc();

    assert_eq!(order.len(), 1024);
    assert_indegree_increasing(cyclic_graph, order);
  }

  fn assert_indegree_increasing(clique: HashTable, order: Vec<VertexId>) {
    let mut last_edge_count_in = usize::MIN;
    let mut last_vertex = 0;

    print!("Edge Count Indegree: ");
    for v in order {
      let edge_count_in = clique.edges(v, Direction::Inbound).len();
      print!("{}, ", edge_count_in);
      assert!(
        edge_count_in >= last_edge_count_in,
        "v({},{}) is smaller than last_vertex({},{})",
        v,
        edge_count_in,
        last_vertex,
        last_edge_count_in
      );
      last_edge_count_in = edge_count_in;
      last_vertex = v;
    }
  }
}
