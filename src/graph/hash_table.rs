use rand::Rng;
use std::collections::{HashMap, HashSet};

type VertexId = u32;

pub struct HashTable {
  data: HashMap<VertexId, HashSet<VertexId>>,
}

impl HashTable {
  // ======= Creational Methods =======

  pub fn new(n: usize) -> HashTable {
    HashTable {
      data: HashMap::with_capacity(n),
    }
  }

  pub fn random<R: Rng>(n: usize, p: f64, rng: &mut R) -> HashTable {
    assert!(p <= 1.0);
    assert!(p >= 0.0);
    let mut graph = HashTable::new(n);
    for u in 0..n as u32 {
      for v in 0..n as u32 {
        if u == v {
          continue;
        }
        let random_value: f64 = rng.gen();
        if p > random_value {
          graph.add_edge(u, v);
        }
      }
    }
    graph
  }

  /// Creates a complete graph, i.e. a clique of size *n*
  pub fn complete(n: usize) -> HashTable {
    let mut graph = HashTable::new(n);
    for u in 0..n as VertexId {
      for v in (u + 1)..n as VertexId {
        graph.add_edge(u, v);
      }
    }
    graph
  }

  // ======= Informational Methods =======

  /// Returns the number of vertices contained in the graph
  pub fn order(&self) -> usize {
    self.data.len()
  }

  /// Returns the number of neighbors of vertex *u*
  pub fn degree(&self, u: VertexId) -> usize {
    match self.data.get(&u) {
      None => 0,
      Some(s) => s.len(),
    }
  }

  /// Checks if the edge (u, v) exists
  pub fn has_edge(&self, u: VertexId, v: VertexId) -> bool {
    match self.data.get(&u) {
      Some(edges) => edges.contains(&v),
      None => false,
    }
  }

  // ======= Mutating Methods =======

  /// Adds the directed edge (u, v)
  pub fn add_edge(&mut self, u: VertexId, v: VertexId) {
    if !self.data.contains_key(&u) {
      self.data.insert(u, HashSet::default());
    }

    self.data.get_mut(&u).unwrap().insert(v);
  }
}

#[cfg(test)]
pub mod tests {
  use crate::graph::hash_table::HashTable;
  use std::panic;

  #[test]
  fn construct_graph() {
    let empty_graph = HashTable::new(0);
    assert_eq!(empty_graph.order(), 0);

    let should_panic = panic::catch_unwind(|| {
      empty_graph.degree(0);
    });
    assert!(should_panic.is_err());

    let graph = HashTable::new(5);
    assert_eq!(graph.order(), 5);
  }

  #[test]
  fn add_edges() {
    let mut graph = HashTable::new(5);
    for u in 0..5 {
      for v in (u + 1)..5 {
        graph.add_edge(u, v);
        graph.add_edge(v, u);
      }
    }

    for u in 0..5 {
      assert_eq!(graph.degree(u), 4);
    }

    for u in 0..5 {
      for v in (u + 1)..5 {
        assert!(graph.has_edge(u, v));
      }
    }

    for u in 0..5 {
      assert!(!graph.has_edge(u, u));
    }
  }
}
