use crate::graph::hash_table::{HashTable, VertexId};

pub struct CycleDetection<'a> {
  graph: &'a HashTable,
  stack: Vec<bool>,
  visited: Vec<bool>,
}

impl<'a> CycleDetection<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    let stack = vec![false; graph.order() as usize];
    let visited = vec![false; graph.order() as usize];
    Self {
      graph,
      stack,
      visited,
    }
  }

  // Quelle: https://www.geeksforgeeks.org/detect-cycle-in-a-graph/
  pub fn is_cyclic(&mut self) -> bool {
    for v in 0..self.graph.order() as VertexId {
      if self.is_cyclic_util(v) {
        return true;
      }
    }

    false
  }

  pub fn is_cyclic_util(&mut self, v: VertexId) -> bool {
    if self.stack[v as usize] {
      return true;
    }

    if self.visited[v as usize] {
      return false;
    }

    self.visited[v as usize] = true;
    self.stack[v as usize] = true;

    for neighbor in self.graph.neighborhood(&v) {
      if self.is_cyclic_util(*neighbor) {
        return true;
      }
    }

    self.stack[v as usize] = false;
    false
  }
}

#[cfg(test)]
pub mod tests {
  use crate::graph::hash_table::HashTable;
  use crate::tools::cycle::CycleDetection;

  #[test]
  fn is_cyclic() {
    let mut graph = HashTable::new();
    graph.add_edge((0, 1));
    graph.add_edge((1, 2));
    graph.add_edge((2, 0));
    let mut dfs = CycleDetection::new(&graph);

    assert!(dfs.is_cyclic());
  }

  #[test]
  fn is_acyclic() {
    let mut graph = HashTable::new();
    graph.add_edge((0, 1));
    graph.add_edge((1, 2));
    let mut dfs = CycleDetection::new(&graph);

    assert!(!dfs.is_cyclic());
  }
}
