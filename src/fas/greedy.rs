/*
A greedy algorithm by Peter Eades, Xuemin Li and W.F. Smyth published in A fast and effective heuristic for the feedback arc set problem.
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

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use crate::graph::hash_table::{Direction, Edge, HashTable, VertexId};

use super::feedback_arc_set::FeedbackArcSet;

/// A struct to hold the information of the greedy heuristic algorithm
pub struct GreedyHeuristic<'a> {
  pub graph: &'a HashTable,
}

impl<'a> GreedyHeuristic<'a> {
  /// Create a new instance of the greedy heuristic algorithm with a reference to the graph
  fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl<'a> FeedbackArcSet for GreedyHeuristic<'a> {
  /// compute the feedback arc set of the referenced
  fn feedback_arc_set(&self) -> HashSet<Edge> {
    // Create a container to hold the calculated fas
    let mut container = FasContainer::new(self.graph);

    // empty lists s1 and s2, as described in the paper.
    // VecDeque since we want to push to the front sometimes.
    let mut s1 = VecDeque::new();
    let mut s2 = VecDeque::new();

    // as long as the virtual graph has vertices since we can't delete vertices in the original graph and don't have a clone
    while self.graph.vertices().len() != container.deleted_nodes.len() {
      // while the graph has sinks (vertices with no outgoing edges)
      for sink in container.sinks() {
        // push the vertex id of the sink into the front of s2
        s2.push_front(sink);
        // update all fas nodes that have something to do with the vertex id (nodes or edges), so the deletion gets noticed by all nodes
        container.update_fas_nodes(sink);
      }

      // the same thing for sources (vertices with only outgoing edges)
      for source in container.sources() {
        s1.push_back(source);
        container.update_fas_nodes(source);
      }

      // find the vertex with the maximum delte (outgoing - ingoing) and map it the the vertex id and push it into s1 if it exists
      if let Some(maximum_delta) = container
        .fas_nodes
        .iter()
        .max_by(|(_, x), (_, y)| x.delta.cmp(&y.delta))
        .map(|(vertex_id, _)| vertex_id.clone())
      {
        s1.push_back(maximum_delta);
        container.update_fas_nodes(maximum_delta)
      }
    }

    // concadenate the two lists as described in the paper and set the index to the vertex id and the value to the enumeration
    let s = s1
      .into_iter()
      .chain(s2)
      .enumerate()
      .map(|(idx, node_idx)| (node_idx, idx))
      .collect::<HashMap<VertexId, usize>>();

    // get all outgoing edges of the graph and map the source and target index to the hashmap s, so we can detect if the arc is a left bound arc
    self
      .graph
      .all_edges()
      .into_iter()
      .filter(|(source_idx, target_idx)| s[source_idx] >= s[target_idx])
      .collect()
  }
}

/// A container to hold information about a vertex so we don't have to iterate all the time through the original graph
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct FasNode {
  /// All target ids of the outgoing edges
  out_edges: Vec<VertexId>,
  /// All source ids of the ingoing edges
  in_edges: Vec<VertexId>,
  /// The number of outgoing edges
  out_degree: usize,
  /// The number of ingoing edges
  in_degree: usize,
  /// The differenze of outgoing and ingoing edges
  delta: isize,
}

impl FasNode {
  /// Update a single fas node by a vertex id
  fn update_by_vertex_id(&mut self, other: VertexId) {
    // Remove the vertex id in the vector of ingoing edges
    self.in_edges.retain(|&vertex_id| vertex_id != other);
    self.out_edges.retain(|&vertex_id| vertex_id != other);
    self.out_degree = self.out_edges.len();
    self.in_degree = self.in_edges.len();
    self.delta = self.out_degree as isize - self.in_degree as isize;
  }
}

/// A container to hold information about all fas nodes created from the graph
#[derive(Debug, Clone)]
struct FasContainer {
  /// all deleted vertex ids
  deleted_nodes: Vec<VertexId>,
  /// all created fas nodes with the vertex id as key
  fas_nodes: BTreeMap<VertexId, FasNode>,
}

impl FasContainer {
  /// Create a new FasContainer
  fn new(graph: &HashTable) -> Self {
    let fas_nodes = graph
      // iter all vertices of the graph
      .vertices()
      .iter()
      // create a new BTreeeMap of all created fas nodes
      .fold(BTreeMap::new(), |mut fas_nodes, &vertex_id| {
        // get the outgoing edges
        let out_edges = graph
          .edges(vertex_id, Direction::Outbound)
          .into_iter()
          .map(|(_, target_idx)| target_idx)
          .collect::<Vec<_>>();
        // get the ingoing edges
        let in_edges = graph
          .edges(vertex_id, Direction::Inbound)
          .into_iter()
          .map(|(source_idx, _)| source_idx)
          .collect::<Vec<_>>();

        // calculate the sizes
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

  /// returns all vertex ids that are sources
  fn sources(&self) -> Vec<u32> {
    self
      .fas_nodes
      .iter()
      .filter(|(_, fas_node)| fas_node.in_degree == 0)
      .map(|(vertex_id, _)| vertex_id)
      .cloned()
      .collect()
  }

  /// returns all vertex ids that are sinks
  fn sinks(&self) -> Vec<u32> {
    self
      .fas_nodes
      .iter()
      .filter(|(_, fas_node)| fas_node.out_degree == 0)
      .map(|(vertex_id, _)| vertex_id)
      .cloned()
      .collect()
  }

  /// deletes a fas node by the vertex id
  fn delete_fas_node(&mut self, vertex_id: VertexId) {
    self.deleted_nodes.push(vertex_id);
    self.fas_nodes.remove(&vertex_id);
    // remove all references to the vertex id in the edges
    self
      .fas_nodes
      .values_mut()
      .filter(|fas_node| {
        fas_node.in_edges.contains(&vertex_id) || fas_node.out_edges.contains(&vertex_id)
      })
      .for_each(|fas_node| fas_node.update_by_vertex_id(vertex_id))
  }

  /// updated a fas node
  fn update_fas_nodes(&mut self, vertex_id: VertexId) {
    self.delete_fas_node(vertex_id);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fas::feedback_arc_set::tests::fas_tests;

  fas_tests!(GreedyHeuristic, [h_001, h_025]);
}
