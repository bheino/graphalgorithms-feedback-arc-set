use crate::graph::hash_table::{HashTable, VertexId};
use crate::scc::strongly_connected_components::StronglyConnectedComponents;
use std::cmp::min;
use std::collections::HashSet;

const UNDEFINED: i32 = -1;

pub struct Tarjan<'a> {
  graph: &'a HashTable,
  vertices: Vec<Vertex>,
  stack: Vec<usize>,
  index: i32,
  sccs: Vec<HashSet<VertexId>>,
}

impl<'a> Tarjan<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self {
      graph,
      vertices: vec![],
      stack: vec![],
      index: 0,
      sccs: vec![],
    }
  }
}

impl StronglyConnectedComponents for Tarjan<'_> {
  fn strongly_connected_components(&mut self) -> Vec<HashSet<VertexId>> {
    self.vertices = self.initial_tarjan_nodes();

    for i in 0..self.vertices.len() {
      if self.vertices[i].index == UNDEFINED {
        self.scc(i);
      }
    }

    self.sccs.clone()
  }
}

impl Tarjan<'_> {
  fn scc(&mut self, v: usize) {
    self.vertices[v].index = self.index;
    self.vertices[v].low_link = self.index;

    self.index += 1;
    self.stack.push(v);
    self.vertices[v].on_stack = true;

    let neighbors = self.graph.neighborhood(&self.vertices[v].id);
    for j in neighbors {
      let w = self.vertices.iter().position(|i| i.id == *j).unwrap();
      if self.vertices[w].index == UNDEFINED {
        self.scc(w);
        self.vertices[v].low_link = min(self.vertices[v].low_link, self.vertices[w].low_link);
      } else if self.vertices[w].on_stack {
        self.vertices[v].low_link = min(self.vertices[v].low_link, self.vertices[w].index);
      }
    }

    if self.vertices[v].low_link == self.vertices[v].index {
      let mut scc_indices = vec![];
      loop {
        let w = self.stack.pop().unwrap();
        self.vertices[w].on_stack = false;
        scc_indices.push(w);
        if w == v {
          break;
        }
      }

      let scc = scc_indices
        .iter()
        .map(|i| self.vertices[*i].id)
        .collect::<HashSet<_>>();
      self.sccs.push(scc);
    }
  }

  fn initial_tarjan_nodes(&self) -> Vec<Vertex> {
    self
      .graph
      .vertices()
      .iter()
      .fold(vec![], |mut tarjan_vertices, &vertex_id| {
        tarjan_vertices.push(Vertex {
          id: vertex_id,
          index: UNDEFINED,
          on_stack: false,
          low_link: UNDEFINED,
        });

        tarjan_vertices
      })
  }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Vertex {
  id: VertexId,
  index: i32,
  low_link: i32,
  on_stack: bool,
}

#[cfg(test)]
mod tests {
  use crate::scc::strongly_connected_components::StronglyConnectedComponents;
  use crate::scc::tarjan::Tarjan;
  use crate::tools::graphs::{
    graph_from_wikipedia_scc, graph_with_multiple_cliques, graph_with_simple_clique,
  };
  use std::collections::HashSet;

  #[test]
  fn works_on_simple_clique() {
    let clique = graph_with_simple_clique();
    let sc_components = Tarjan::new(&clique).strongly_connected_components();

    assert_eq!(sc_components.len(), 1);
    assert_eq!(
      *sc_components.get(0).unwrap(),
      HashSet::from_iter(clique.vertices())
    );
  }

  #[test]
  fn works_on_multiple_cliques() {
    let cyclic_graph = graph_with_multiple_cliques();
    let sc_components = Tarjan::new(&cyclic_graph).strongly_connected_components();

    assert_eq!(sc_components.len(), 4);

    let scc_1 = HashSet::from([18]);
    let scc_2 = HashSet::from([17]);
    let scc_3 = HashSet::from([0]);
    let scc_4 = cyclic_graph
      .vertices()
      .into_iter()
      .filter(|&v| v != 18 && v != 17 && v != 0)
      .collect();
    assert!(sc_components.contains(&scc_1));
    assert!(sc_components.contains(&scc_2));
    assert!(sc_components.contains(&scc_3));
    assert!(sc_components.contains(&scc_4));
  }

  #[test]
  fn works_on_wikipedia_scc() {
    let cyclic_graph = graph_from_wikipedia_scc();
    let sc_components = Tarjan::new(&cyclic_graph).strongly_connected_components();

    assert_eq!(sc_components.len(), 3);

    let scc_1 = HashSet::from([1, 2, 5]);
    let scc_2 = HashSet::from([3, 4, 8]);
    let scc_3 = HashSet::from([6, 7]);
    assert!(sc_components.contains(&scc_1));
    assert!(sc_components.contains(&scc_2));
    assert!(sc_components.contains(&scc_3));
  }
}
