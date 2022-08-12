use crate::graph::hash_table::{HashTable, VertexId};
use crate::scc::strongly_connected_components::StronglyConnectedComponents;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

const UNDEFINED: i32 = -1;

pub struct Tarjan<'a> {
  graph: &'a HashTable,
  vertices: HashMap<VertexId, Vertex>,
  stack: Vec<&'a mut Vertex>,
  index: i32,
}

impl<'a> Tarjan<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self {
      graph,
      vertices: graph
        .vertices()
        .iter()
        .map(|v| (*v, Vertex::new(*v)))
        .collect(),
      stack: vec![],
      index: 0,
    }
  }

  fn scc(&mut self, v: &'a mut Vertex) {
    v.index = self.index;
    v.low_link = self.index;
    self.index += 1;

    self.stack.push(v);
    v.on_stack = true;

    for neighbor in self.graph.neighborhood(&v.id) {
      let w = self.vertices.get_mut(neighbor).unwrap();
      if w.index == UNDEFINED {
        self.scc(w);
      } else if w.on_stack {
        v.low_link = min(v.low_link, w.index);
      }
    }

    if v.low_link == v.index {
      let mut scc = vec![];

      loop {
        let mut w = self.stack.pop().unwrap();
        w.on_stack = false;
        scc.push(w);
        if w == v {
          break;
        }
      }
      println!("SCC: {:?}", scc);
    }
  }
}

impl<'a> StronglyConnectedComponents for Tarjan<'a> {
  fn strongly_connected_components(&mut self) -> HashSet<VertexId> {
    let mut stack: Vec<&Vertex> = Vec::new();

    for v in self.vertices.values_mut() {
      if v.index == UNDEFINED {
        self.scc(v);
      }
    }

    HashSet::from_iter(self.graph.vertices())
  }
}

#[derive(PartialEq, Debug)]
struct Vertex {
  id: VertexId,
  index: i32,
  low_link: i32,
  on_stack: bool,
}

impl Vertex {
  fn new(id: VertexId) -> Self {
    Self {
      id,
      index: UNDEFINED,
      low_link: UNDEFINED,
      on_stack: false,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::scc::strongly_connected_components::StronglyConnectedComponents;
  use crate::scc::tarjan::Tarjan;
  use crate::tools::graphs::graph_with_simple_clique;
  use std::collections::HashSet;

  #[test]
  fn works_on_simple_clique() {
    let clique = graph_with_simple_clique();
    let sc_components = Tarjan::new(&clique).strongly_connected_components();

    assert_eq!(sc_components.len(), clique.vertices().len());
    assert_eq!(sc_components, HashSet::from_iter(clique.vertices()));
  }
}
