use crate::graph::hash_table::{Direction, Edge, HashTable, VertexId};
use std::collections::HashSet;

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

    vec.sort_by(|(_, v1_edge_count), (_, v2_edge_count)| v1_edge_count.cmp(v2_edge_count));
    vec.iter().map(|(v, _)| *v).collect()
  }
}

pub fn leftward_edges(graph: &HashTable, ordering: Vec<VertexId>) -> HashSet<Edge> {
  let mut leftward_edges = HashSet::new();

  for source_idx in 0..ordering.len() {
    let source = ordering[source_idx];

    for (_, destination) in graph.edges(source, Direction::Outbound) {
      let destination_idx = ordering
        .iter()
        .position(|v| *v == destination)
        .unwrap_or_else(|| panic!("Ordering = {:?}, Destination = {:?}", ordering, destination));
      if destination_idx < source_idx {
        leftward_edges.insert((source, destination));
      }
    }
  }

  leftward_edges
}

#[cfg(test)]
mod tests {
  use crate::graph::hash_table::{Direction, HashTable, VertexId};
  use crate::ordering::topological_sort::TopologicalSort;
  use crate::tools::graphs::{
    graph_from_file, graph_from_wikipedia_scc, graph_with_multiple_cliques,
    graph_with_simple_clique,
  };

  #[test]
  fn works_on_simple_clique() {
    let clique = graph_with_simple_clique();

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

  #[test]
  fn works_on_wikipedia_scc() {
    let cyclic_graph = graph_from_wikipedia_scc();
    let order = TopologicalSort::new(&cyclic_graph).sort_by_indegree_asc();

    assert_eq!(order.len(), 8);
    assert_indegree_increasing(cyclic_graph, order);
  }

  fn assert_indegree_increasing(clique: HashTable, order: Vec<VertexId>) {
    let mut last_edge_count_in = usize::MIN;
    let mut last_vertex = 0;

    print!("Edge Count Indegree: ");
    for v in order {
      let edge_count_in = clique.edges(v, Direction::Inbound).len();
      print!("{:?}, ", (v, edge_count_in));
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
