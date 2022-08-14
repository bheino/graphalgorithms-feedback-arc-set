use rand::Rng;
use std::collections::HashMap;

pub type VertexId = u32;
pub type Edge = (VertexId, VertexId);

pub enum Direction {
  Inbound,
  Outbound,
}

#[derive(Clone, Debug)]
pub struct HashTable {
  data: HashMap<VertexId, Vec<VertexId>>,
}

impl HashTable {
  // ======= Creational Methods =======

  pub fn new() -> Self {
    Self {
      data: HashMap::new(),
    }
  }

  pub fn from_edges(edges: &[Edge]) -> Self {
    let mut d = HashTable::new();
    edges.iter().for_each(|e| d.add_edge(*e));

    d
  }

  pub fn from_vertices_and_edges(vertices: &[VertexId], edges: &[Edge]) -> Self {
    let mut d = HashTable::new();
    vertices.iter().for_each(|v| d.add_vertex(*v));
    edges.iter().for_each(|e| d.add_edge(*e));

    d
  }

  pub fn random<R: Rng>(n: usize, p: f64, rng: &mut R) -> HashTable {
    assert!(p <= 1.0);
    assert!(p >= 0.0);
    let mut graph = HashTable::new();
    for u in 0..n as u32 {
      for v in 0..n as u32 {
        if u == v {
          continue;
        }
        let random_value: f64 = rng.gen();
        if p > random_value {
          graph.add_edge((u, v));
        }
      }
    }
    graph
  }

  /// Creates a complete graph, i.e. a clique of size *n*
  pub fn complete(n: usize) -> HashTable {
    let mut graph = HashTable::new();
    for u in 0..n as VertexId {
      for v in (u + 1)..n as VertexId {
        graph.add_edge((u, v));
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
      None => panic!("Unknown VertexId {u}"),
      Some(s) => s.len(),
    }
  }

  pub fn edge_count(&self) -> usize {
    self.data.iter().map(|edges| edges.1.len()).sum()
  }

  // Returns all vertices
  pub fn vertices(&self) -> Vec<VertexId> {
    self.data.clone().into_keys().collect()
  }

  // Returns all edges of a vertex for a specified direction
  pub fn edges(&self, v: VertexId, d: Direction) -> Vec<Edge> {
    match d {
      Direction::Outbound => self
        .data
        .get(&v)
        .unwrap()
        .iter()
        .map(|v2| (v, *v2))
        .collect(),
      Direction::Inbound => self
        .data
        .iter()
        .filter(|(_, neighbours)| neighbours.contains(&v))
        .map(|(vertex, _)| *vertex)
        .map(|v2| (v2, v))
        .collect(),
    }
  }

  pub fn neighborhood(&self, v: &VertexId) -> &[VertexId] {
    self
      .data
      .get(v)
      .map(|neighbors| neighbors.as_slice())
      .unwrap_or_default()
  }

  /// Checks if the edge (u, v) exists
  pub fn has_edge(&self, u: VertexId, v: VertexId) -> bool {
    match self.data.get(&u) {
      Some(edges) => edges.contains(&v),
      None => false,
    }
  }

  // ======= Mutating Methods =======

  fn add_vertex(&mut self, v: VertexId) {
    self.data.entry(v).or_insert_with(Vec::new);
  }

  /// Adds the directed edge (u, v)
  pub fn add_edge(&mut self, e: Edge) {
    let edges = self.data.entry(e.0).or_insert_with(Vec::new);
    if !edges.contains(&e.1) {
      edges.push(e.1);
    }

    self.data.entry(e.1).or_insert_with(Vec::new);
  }

  pub fn remove_vertex(&mut self, v: VertexId) {
    for neighbors in self.data.values_mut() {
      neighbors
        .iter()
        .position(|&neighbor| neighbor == v)
        .map(|index| neighbors.remove(index));
    }
    self.data.remove(&v);
  }

  pub fn remove_edge(&mut self, e: Edge) {
    match self.data.get_mut(&e.0) {
      Some(edges) => {
        edges
          .iter()
          .position(|&neighbor| neighbor == e.1)
          .map(|index| edges.remove(index));
      }
      None => (),
    };
  }

  // ======= Algorithm Methods =======

  pub fn random_vertex(&self) -> VertexId {
    let idx = rand::thread_rng().gen_range(0..self.data.len());
    self.data.keys().nth(idx).copied().unwrap()
  }
}

#[cfg(test)]
pub mod tests {
  use crate::graph::hash_table::HashTable;
  use std::panic;

  #[test]
  fn construct_graph() {
    let mut empty_graph = HashTable::new();
    assert_eq!(empty_graph.order(), 0);

    let should_panic = panic::catch_unwind(|| {
      empty_graph.degree(0);
    });
    assert!(should_panic.is_err());

    empty_graph.add_edge((2, 3));
    assert_eq!(empty_graph.order(), 2);
  }

  #[test]
  fn add_edges() {
    let mut graph = HashTable::new();
    for u in 0..5 {
      for v in (u + 1)..5 {
        graph.add_edge((u, v));
        graph.add_edge((v, u));
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

  #[test]
  fn neighborhood() {
    let mut graph = HashTable::new();

    let to_add = vec![3, 4, 1, 1, 4];
    let u = 2;
    for v in &to_add {
      graph.add_edge((u, *v));
    }

    let mut added: Vec<_> = graph.neighborhood(&u).to_vec();
    added.sort_unstable();

    assert_eq!(added, vec![1, 3, 4]);
  }
}
