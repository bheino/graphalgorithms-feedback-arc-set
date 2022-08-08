use crate::graph::hash_table::Edge;
use std::collections::HashSet;

pub trait FeedbackArcSet {
  fn feedback_arc_set(&self) -> HashSet<Edge>;
}

#[cfg(test)]
pub mod tests {
  use crate::fas::feedback_arc_set::FeedbackArcSet;
  use crate::graph::hash_table::{Edge, HashTable};
  use crate::tools::cycle::CycleDetection;
  use crate::tools::dot::Dot;
  use std::collections::HashSet;

  pub fn test_feedback_arc_set(algorithm: impl FeedbackArcSet, cyclic_graph: &HashTable) {
    let is_cyclic = |graph: &HashTable| -> bool { CycleDetection::new(graph).is_cyclic() };
    assert!(is_cyclic(cyclic_graph), "No cycles found in graph!?");

    let removable_edges = algorithm.feedback_arc_set();
    let remove_edges = |cyclic_graph: &HashTable, edges: &HashSet<Edge>| {
      let mut acyclic_graph = cyclic_graph.clone();
      for edge in edges {
        acyclic_graph.remove_edge(*edge);
      }
      acyclic_graph
    };

    let acyclic_graph = remove_edges(cyclic_graph, &removable_edges);

    if is_cyclic(&acyclic_graph) {
      let print_dot = |prefix, graph| {
        println!("{}", prefix);
        println!("{}", Dot::new(graph));
      };

      print_dot("Cyclic Graph:", cyclic_graph);
      print_dot("Acyclic Graph:", &acyclic_graph);

      panic!("Graph still has cycles!");
    }
  }
}
