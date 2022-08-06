use crate::graph::hash_table::{HashTable, VertexId};

pub struct DepthFirstSearch<'a> {
  graph: &'a HashTable,
  stack: Vec<VertexId>,
  visited: Vec<usize>,
}

impl<'a> DepthFirstSearch<'a> {
  pub fn new(graph: &'a HashTable, start: VertexId) -> Self {
    let stack = vec![start];
    let visited = vec![0; graph.order() as usize];
    Self {
      graph,
      stack,
      visited,
    }
  }

  pub fn is_acyclic(&mut self) -> bool {
    while let Some(u) = self.next() {
      if self.visited[u as usize] > 1 {
        return false;
      }
    }
    self.visited.iter().all(|&v| v < 2)
  }
}

impl<'a> Iterator for DepthFirstSearch<'a> {
  type Item = VertexId;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(u) = self.stack.pop() {
      if self.visited[u as usize] == 0 {
        self.visited[u as usize] += 1;
        self.stack.extend_from_slice(self.graph.neighborhood(u));
        return Some(u);
      } else {
        self.visited[u as usize] += 1;
      }
    }
    None
  }
}

#[cfg(test)]
pub mod tests {
  use crate::graph::hash_table::HashTable;
  use crate::tools::cycle::DepthFirstSearch;

  #[test]
  fn dfs() {
    let mut graph = HashTable::new(5);
    graph.add_edge((0, 1));
    graph.add_edge((0, 2));
    graph.add_edge((0, 3));
    graph.add_edge((2, 3));
    graph.add_edge((3, 2));
    graph.add_edge((3, 0));

    let dfs_order: Vec<_> = DepthFirstSearch::new(&graph, 0).collect();
    let expected_order: Vec<_> = vec![0, 3, 2, 1];
    assert_eq!(dfs_order, expected_order);
  }

  #[test]
  fn is_not_acyclic() {
    let mut graph = HashTable::new(3);
    graph.add_edge((0, 1));
    graph.add_edge((1, 2));
    graph.add_edge((2, 0));
    let mut dfs = DepthFirstSearch::new(&graph, 0);

    assert!(!dfs.is_acyclic());
  }

  #[test]
  fn is_acyclic() {
    let mut graph = HashTable::new(3);
    graph.add_edge((0, 1));
    graph.add_edge((1, 2));
    let mut dfs = DepthFirstSearch::new(&graph, 0);

    assert!(dfs.is_acyclic());
  }
}
