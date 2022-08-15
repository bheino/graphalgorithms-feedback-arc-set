/*
Procedure GR
(G: DiGraph; var s: VertexSequence);
s1 <- ∅; s2 <- ∅;
while G != ∅ do
    {while G contains a sink do
        {choose a sink u; s2 <- us2; G <&- &G - u);
    while G contains a source do
        {choose a source u; s1 <- s1u; G <- G - u};
    if G != ∅ then
    {choose a vertex u for which 6(u) is a
    maximum; s1 <- s1u; G <- G - u}};
s <- s1s2.
*/

use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::hash_table::{Direction, Edge, HashTable, VertexId};

use super::feedback_arc_set::FeedbackArcSet;

pub struct GreedyHeuristic<'a> {
  pub graph: &'a HashTable,
}

impl<'a> GreedyHeuristic<'a> {
  fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl<'a> FeedbackArcSet for GreedyHeuristic<'a> {
  fn feedback_arc_set(&self) -> HashSet<Edge> {
    let mut container = FasContainer::new(self.graph);

    let mut s1 = VecDeque::new();
    let mut s2 = VecDeque::new();

    // as long as the graph has vertices
    while self.graph.vertices().len() != container.deleted_nodes.len() {
      for sink in container.sinks() {
        s2.push_front(sink.vertex_id);
        container.deleted_nodes.push(sink.vertex_id);
        container.update_fas_nodes(self.graph);
      }

      for source in container.sources() {
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

    let s = s1
      .into_iter()
      .chain(s2)
      .enumerate()
      .map(|(idx, node_idx)| (node_idx, idx))
      .collect::<HashMap<VertexId, usize>>();

    self
      .graph
      .all_edges()
      .into_iter()
      .filter(|(source_idx, target_idx)| s[source_idx] >= s[target_idx])
      .collect()
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
mod tests {
  use super::*;
  use crate::fas::feedback_arc_set::tests::fas_tests;

  fas_tests!(GreedyHeuristic, [h_001, h_025]);
}
