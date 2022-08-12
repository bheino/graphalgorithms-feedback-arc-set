/*
Procedure GR
(G: DiGraph; var s: VertexSequence);
s1 <- ∅; s2 <- ∅;
while G != ∅ do
    {while G contains a sink do
        {choose a sink u; s2 <- us2; G <- G - u);
    while G contains a source do
        {choose a source u; s1 <- s1u; G <- G - u};
    if G != ∅ then
    {choose a vertex u for which 6(u) is a
    maximum; s1 <- s1u; G <- G - u}};
s <- s1s2.
*/

use std::collections::{HashSet, VecDeque};

use crate::graph::hash_table::{Direction, Edge, HashTable, VertexId};

use super::feedback_arc_set::FeedbackArcSet;

pub struct GreedyHeuristic<'a> {
  graph: &'a HashTable,
}

impl<'a> GreedyHeuristic<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl FeedbackArcSet for GreedyHeuristic<'_> {
  fn feedback_arc_set(&self) -> HashSet<Edge> {
    let mut container = FasContainer::new(self.graph);

    let mut s1 = VecDeque::new();
    let mut s2 = VecDeque::new();

    // as long as the graph has vertices
    while self.graph.vertices().len() != container.deleted_nodes.len() {
      for sink in container.sinks() {
        println!("sinks: {:?}", sink);
        s2.push_front(sink.vertex_id);
        container.deleted_nodes.push(sink.vertex_id);
        container.update_fas_nodes(self.graph);
      }

      for source in container.sources() {
        println!("sources: {:?}", source);
        s1.push_back(source.vertex_id);
        container.deleted_nodes.push(source.vertex_id);
        container.update_fas_nodes(self.graph);
      }

      let maximum_delta = container
        .fas_nodes
        .iter()
        .max_by(|x, y| x.delta.cmp(&y.delta));

      if let Some(maximum_delta) = maximum_delta {
        s1.push_back(maximum_delta.vertex_id);
        container.deleted_nodes.push(maximum_delta.vertex_id);
        container.update_fas_nodes(self.graph)
      }
    }

    let s = s1.iter().chain(s2.iter()).map(|x| *x).collect::<Vec<_>>();
    println!("{:?}", s);

    HashSet::new()
  }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct FasNode {
  vertex_id: VertexId,
  delta: isize,
  out_degree: usize,
  in_degree: usize,
}

#[derive(Debug, Clone)]
struct FasContainer {
  deleted_nodes: Vec<VertexId>,
  fas_nodes: Vec<FasNode>,
}

impl FasContainer {
  fn new(graph: &HashTable) -> Self {
    let fas_nodes = graph
      .vertices()
      .iter()
      .fold(vec![], |mut fas_nodes, &vertex_id| {
        let out_degree = graph.edges(vertex_id, Direction::Outbound).iter().count();
        let in_degree = graph.edges(vertex_id, Direction::Inbound).iter().count();
        let delta = out_degree as isize - in_degree as isize;

        fas_nodes.push(FasNode {
          vertex_id,
          delta,
          out_degree,
          in_degree,
        });
        fas_nodes
      });

    Self {
      deleted_nodes: vec![],
      fas_nodes,
    }
  }

  fn delta_positive(&self) -> Vec<FasNode> {
    self
      .fas_nodes
      .iter()
      .cloned()
      .filter(|fas_node| fas_node.delta >= 0)
      .collect()
  }

  fn delta_negative(&self) -> Vec<FasNode> {
    self
      .fas_nodes
      .iter()
      .cloned()
      .filter(|fas_node| fas_node.delta < 0)
      .collect()
  }

  fn sources(&self) -> Vec<FasNode> {
    self
      .fas_nodes
      .iter()
      .cloned()
      .filter(|fas_node| fas_node.in_degree == 0)
      .collect()
  }

  fn sinks(&self) -> Vec<FasNode> {
    self
      .fas_nodes
      .iter()
      .cloned()
      .filter(|fas_node| fas_node.out_degree == 0)
      .collect()
  }

  fn update_fas_nodes(&mut self, graph: &HashTable) {
    self.fas_nodes = graph
      .vertices()
      .iter()
      .filter(|&vertex_id| !self.deleted_nodes.contains(vertex_id))
      .fold(vec![], |mut fas_nodes, &vertex_id| {
        let out_degree = graph
          .edges(vertex_id, Direction::Outbound)
          .iter()
          .filter(|(source_id, target_id)| {
            !self.deleted_nodes.contains(source_id) && !self.deleted_nodes.contains(target_id)
          })
          .count();
        let in_degree = graph
          .edges(vertex_id, Direction::Inbound)
          .iter()
          .filter(|(source_id, target_id)| {
            !self.deleted_nodes.contains(source_id) && !self.deleted_nodes.contains(target_id)
          })
          .count();
        let delta = out_degree as isize - in_degree as isize;

        fas_nodes.push(FasNode {
          vertex_id,
          delta,
          out_degree,
          in_degree,
        });
        fas_nodes
      });
  }
}

#[cfg(test)]
mod test {
  use crate::{
    fas::feedback_arc_set::FeedbackArcSet, graph::hash_table::HashTable, tools::dot::Dot,
  };

  use super::GreedyHeuristic;

  fn graph() -> HashTable {
    HashTable::from_edges(&[
      (0, 1),
      (0, 7),
      (1, 2),
      (1, 3),
      (2, 4),
      (2, 5),
      (2, 6),
      (3, 7),
      (6, 8),
      (6, 9),
      (7, 9),
      (5, 10),
      (8, 10),
      (9, 10),
      (4, 11),
      (4, 12),
      (12, 11),
      (10, 13),
      (11, 13),
      (10, 14),
      (14, 15),
      (14, 16),
      (16, 15),
      (16, 17),
      (17, 18),
      (12, 18),
      // Ab hier kommen Zyklen rein
      (13, 2),
      (7, 1),
      (6, 7),
      (15, 10),
      (15, 13),
    ])
  }

  #[test]
  fn it_works() {
    let graph = graph();

    print!("{}", Dot::new(&graph));

    let algo = GreedyHeuristic::new(&graph);
    println!("{:?}", algo.feedback_arc_set());
  }
}
