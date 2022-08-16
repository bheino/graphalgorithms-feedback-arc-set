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
        s2.push_front(sink);
        container.update_fas_nodes(sink);
      }

      for source in container.sources() {
        s1.push_back(source);
        container.update_fas_nodes(source);
      }

      let maximum_delta = container
        .fas_nodes
        .iter()
        .max_by(|(_, x), (_, y)| x.delta.cmp(&y.delta))
        .map(|(vertex_id, _)| vertex_id.clone());

      if let Some(maximum_delta) = maximum_delta {
        s1.push_back(maximum_delta);
        container.update_fas_nodes(maximum_delta)
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
  out_edges: Vec<VertexId>,
  in_edges: Vec<VertexId>,
  out_degree: usize,
  in_degree: usize,
  delta: isize,
}

impl FasNode {
  fn update(&mut self, delta: isize, out_degree: usize, in_degree: usize) {
    self.delta = delta;
    self.out_degree = out_degree;
    self.in_degree = in_degree;
  }

  fn update_by_vertex_id(&mut self, other: VertexId) {
    self.in_edges.retain(|&vertex_id| vertex_id != other);
    self.out_edges.retain(|&vertex_id| vertex_id != other);
    self.out_degree = self.out_edges.len();
    self.in_degree = self.in_edges.len();
    self.delta = self.out_degree as isize - self.in_degree as isize;
  }
}

#[derive(Debug, Clone)]
struct FasContainer {
  deleted_nodes: Vec<VertexId>,
  fas_nodes: HashMap<VertexId, FasNode>,
}

impl FasContainer {
  fn new(graph: &HashTable) -> Self {
    let fas_nodes = graph
      .vertices()
      .iter()
      .fold(HashMap::new(), |mut fas_nodes, &vertex_id| {
        let out_edges = graph
          .edges(vertex_id, Direction::Outbound)
          .into_iter()
          .map(|(_, target_idx)| target_idx)
          .collect::<Vec<_>>();
        let in_edges = graph
          .edges(vertex_id, Direction::Inbound)
          .into_iter()
          .map(|(source_idx, _)| source_idx)
          .collect::<Vec<_>>();

        let out_degree = out_edges.len();
        let in_degree = in_edges.len();
        let delta = out_edges.len() as isize - in_edges.len() as isize;

        fas_nodes.insert(
          vertex_id,
          FasNode {
            out_edges,
            in_edges,
            out_degree,
            in_degree,
            delta,
          },
        );
        fas_nodes
      });

    Self {
      deleted_nodes: vec![],
      fas_nodes,
    }
  }

  fn sources(&self) -> Vec<u32> {
    self
      .fas_nodes
      .iter()
      .filter(|(_, fas_node)| fas_node.in_degree == 0)
      .map(|(vertex_id, _)| vertex_id)
      .cloned()
      .collect()
  }

  fn sinks(&self) -> Vec<u32> {
    self
      .fas_nodes
      .iter()
      .filter(|(_, fas_node)| fas_node.out_degree == 0)
      .map(|(vertex_id, _)| vertex_id)
      .cloned()
      .collect()
  }

  fn delete_fas_node(&mut self, vertex_id: VertexId) {
    self.deleted_nodes.push(vertex_id);
    self.fas_nodes.remove(&vertex_id);
    self
      .fas_nodes
      .values_mut()
      .filter(|fas_node| {
        fas_node.in_edges.contains(&vertex_id) || fas_node.out_edges.contains(&vertex_id)
      })
      .for_each(|fas_node| fas_node.update_by_vertex_id(vertex_id))
  }

  fn update_fas_nodes(&mut self, vertex_id: VertexId) {
    self.delete_fas_node(vertex_id);

    // self
    //   .fas_nodes
    //   .iter_mut()
    //   .filter(|(_, fas_node)| fas_node.in_edges.contains(vertex_id))
    //   .for_each(|(_, fas_node)| {
    //     let out_degree = graph
    //       .edges(fas_node.vertex_id, Direction::Outbound)
    //       .iter()
    //       .filter(|(source_id, target_id)| {
    //         !self.deleted_nodes.contains(source_id) && !self.deleted_nodes.contains(target_id)
    //       })
    //       .count();
    //     let in_degree = graph
    //       .edges(fas_node.vertex_id, Direction::Inbound)
    //       .iter()
    //       .filter(|(source_id, target_id)| {
    //         !self.deleted_nodes.contains(source_id) && !self.deleted_nodes.contains(target_id)
    //       })
    //       .count();
    //     let delta = out_degree as isize - in_degree as isize;
    //     fas_node.update(delta, out_degree, in_degree);
    //   });
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fas::feedback_arc_set::tests::fas_tests;

  fas_tests!(GreedyHeuristic, [h_001, h_025]);
}
