use crate::graph::hash_table::{HashTable, VertexId};

struct Vertex {
  id: VertexId,
  visited: bool,
  on_stack: bool,
}

pub struct CycleDetection<'a> {
  graph: &'a HashTable,
  vertices: Vec<Vertex>,
}

impl<'a> CycleDetection<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self {
      graph,
      vertices: vec![],
    }
  }

  fn initial_cycle_vertices(&self) -> Vec<Vertex> {
    self
      .graph
      .vertices()
      .iter()
      .fold(vec![], |mut cycle_vertices, &vertex_id| {
        cycle_vertices.push(Vertex {
          id: vertex_id,
          visited: false,
          on_stack: false,
        });

        cycle_vertices
      })
  }

  // Quelle: https://www.geeksforgeeks.org/detect-cycle-in-a-graph/
  pub fn is_cyclic(&mut self) -> bool {
    self.vertices = self.initial_cycle_vertices();

    for i in 0..self.vertices.len() {
      if self.is_cyclic_util(i) {
        return true;
      }
    }

    false
  }

  pub fn is_cyclic_util(&mut self, v: usize) -> bool {
    if self.vertices[v].on_stack {
      return true;
    }

    if self.vertices[v].visited {
      return false;
    }

    self.vertices[v].visited = true;
    self.vertices[v].on_stack = true;

    for neighbor in self.graph.neighborhood(&self.vertices[v].id) {
      let w = self
        .vertices
        .iter()
        .position(|i| i.id == *neighbor)
        .unwrap();

      if self.is_cyclic_util(w) {
        return true;
      }
    }

    self.vertices[v].on_stack = false;
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
