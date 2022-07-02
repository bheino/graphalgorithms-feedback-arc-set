use petgraph::stable_graph::GraphIndex;
use petgraph::visit::{
  GraphProp, IntoEdgeReferences, IntoNeighbors, IntoNodeIdentifiers, NodeCount, NodeIndexable,
};
use petgraph::Directed;

pub trait FeedbackArcSet {
  /// Finds a feedback arc set: a set of edges in the given directed graph, which when removed, make the graph acyclic.
  fn compute_fas<G>(&self, g: G) -> Vec<G::EdgeRef>
  where
    G: IntoEdgeReferences + GraphProp<EdgeType = Directed>,
    G::NodeId: GraphIndex,
    G: NodeCount,
    G: IntoNodeIdentifiers + IntoNeighbors + NodeIndexable;
}

#[cfg(test)]
mod tests {
  use crate::algo::greedy_heuristic::GreedyHeuristic;
  use crate::feedback_arc_set::FeedbackArcSet;
  use crate::tools::metis::graph_from_file;
  use petgraph::algo::is_cyclic_directed;
  use petgraph::dot::{Config, Dot};
  use petgraph::stable_graph::{EdgeReference, StableDiGraph, StableGraph};
  use petgraph::visit::EdgeRef;

  #[test]
  fn deterministic_on_simple_clique() {
    let clique = StableDiGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 1)]);

    let edges = GreedyHeuristic {}.compute_fas(&clique);

    assert_eq!(edges.len(), 1);
    assert_eq!(edges.get(0).unwrap().source().index(), 2);
    assert_eq!(edges.get(0).unwrap().target().index(), 3);
  }

  #[test]
  fn works_on_multiple_cliques() {
    let cyclic_graph = StableDiGraph::<i32, ()>::from_edges(&[
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
    ]);

    test_feedback_arc_set(GreedyHeuristic {}, &cyclic_graph, 4, true, true);
  }

  #[test]
  fn works_on_h_001() {
    let cyclic_graph = graph_from_file("test/resources/heuristic/h_001");
    test_feedback_arc_set(GreedyHeuristic {}, &cyclic_graph, 143, true, false);
  }

  #[test]
  fn works_on_h_025() {
    let cyclic_graph = graph_from_file("test/resources/heuristic/h_025");
    test_feedback_arc_set(GreedyHeuristic {}, &cyclic_graph, 1574, true, false);
  }

  fn test_feedback_arc_set<A: FeedbackArcSet>(
    algorithm: A,
    cyclic_graph: &StableGraph<i32, ()>,
    expected_set_count: usize,
    should_print_edges: bool,
    should_print_dot: bool,
  ) {
    assert!(is_cyclic_directed(cyclic_graph));

    if should_print_dot {
      print_dot("Cyclic Graph:", cyclic_graph)
    };

    let removable_edges = algorithm.compute_fas(cyclic_graph);
    if should_print_edges {
      print_edges(&removable_edges);
    }

    let acyclic_graph = remove_edges(cyclic_graph, &removable_edges);
    if should_print_dot {
      print_dot("Acyclic Graph:", &acyclic_graph);
    }

    assert_eq!(removable_edges.len(), expected_set_count);
    assert!(!is_cyclic_directed(&acyclic_graph));
  }

  fn print_edges(removable_edges: &[EdgeReference<()>]) {
    println!("Edges to be removed:");
    removable_edges
      .iter()
      .map(|edge| (edge.source(), edge.target()))
      .for_each(|(source, target)| println!("\t{:?} -> {:?}", source.index(), target.index()));
    println!();
  }

  fn remove_edges(
    cyclic_graph: &StableGraph<i32, ()>,
    removable_edges: &Vec<EdgeReference<()>>,
  ) -> StableGraph<i32, ()> {
    let mut acyclic_graph = cyclic_graph.clone();
    for edge in removable_edges.as_slice() {
      acyclic_graph.remove_edge(edge.id());
    }
    acyclic_graph
  }

  fn print_dot(prefix: &str, graph: &StableGraph<i32, ()>) {
    println!("{}", prefix);
    println!(
      "{:?}",
      // zeigt in IntelliJ Fehler (Dot doesn't implement Debug) an, aber läuft.
      Dot::with_config(graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]),
    );
  }
}
