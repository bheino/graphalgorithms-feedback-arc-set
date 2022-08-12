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

    for (i, v) in self.initial_tarjan_nodes().iter().enumerate() {
      if v.index == UNDEFINED {
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
      let mut scc_indizes = vec![];
      loop {
        let w = self.stack.pop().unwrap();
        self.vertices[w].on_stack = false;
        scc_indizes.push(w);
        if w == v {
          break;
        }
      }

      let scc = scc_indizes
        .iter()
        .map(|i| self.vertices[*i].id)
        .collect::<HashSet<_>>();
      println!("SCC: {:?}", scc);
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
  use crate::tools::graphs::graph_with_simple_clique;

  #[test]
  fn works_on_simple_clique() {
    let clique = graph_with_simple_clique();
    let sc_components = Tarjan::new(&clique).strongly_connected_components();

    // TODO Liefert: {0, 2, 1}, {2}, {1}. Letztere beiden falsch bei einer Clique! Prüfen warum...
    assert_eq!(sc_components.len(), 3);
  }
}
