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
    let mut deleted_nodes = HashSet::new();
    let initial_fas_nodes = self.initial_fas_nodes();

    println!("{:?}", initial_fas_nodes);

    let mut s1 = VecDeque::new();
    let mut s2 = VecDeque::new();

    // as long as the graph has vertices
    // while self.graph.vertices().len() != deleted_nodes.len() {
    // find all sinks
    let sinks = initial_fas_nodes
      .iter()
      .filter(|fas_node| fas_node.in_degree > fas_node.out_degree)
      .collect::<Vec<_>>();

    for sink in sinks {
      println!("sinks: {:?}", sink);
      // todo update initial_fas_nodes
      s2.push_front(sink);
      deleted_nodes.insert(sink);
    }

    // find all sources
    let sources = initial_fas_nodes
      .iter()
      .filter(|fas_node| fas_node.in_degree < fas_node.out_degree)
      .collect::<Vec<_>>();

    for source in sources {
      println!("sources: {:?}", source);
      // todo update initial_fas_nodes
      s1.push_back(source);
      deleted_nodes.insert(source);
    }
    // }

    HashSet::new()
  }
}

impl GreedyHeuristic<'_> {
  fn initial_fas_nodes(&self) -> Vec<FasNode> {
    self
      .graph
      .vertices()
      .iter()
      .fold(vec![], |mut fas_nodes, &vertex_id| {
        let out_degree = self
          .graph
          .edges(vertex_id, Direction::Outbound)
          .iter()
          .count();
        let in_degree = self
          .graph
          .edges(vertex_id, Direction::Inbound)
          .iter()
          .count();
        let degree = out_degree + in_degree;

        fas_nodes.push(FasNode {
          vertex_id,
          degree,
          out_degree,
          in_degree,
        });
        fas_nodes
      })
  }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct FasNode {
  vertex_id: VertexId,
  degree: usize,
  out_degree: usize,
  in_degree: usize,
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
    algo.feedback_arc_set();
  }
}
