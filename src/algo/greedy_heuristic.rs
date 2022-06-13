use crate::feedback_arc_set::FeedbackArcSet;
use petgraph::graph::GraphIndex;
use petgraph::visit::{GraphProp, IntoEdgeReferences, NodeCount};
use petgraph::Directed;

pub struct GreedyHeuristic {}

impl FeedbackArcSet for GreedyHeuristic {
  fn compute<G>(&self, g: G) -> Vec<G::EdgeRef>
  where
    G: IntoEdgeReferences + GraphProp<EdgeType = Directed>,
    G::NodeId: GraphIndex,
    G: NodeCount,
  {
    petgraph::algo::feedback_arc_set::greedy_feedback_arc_set(g).collect()
  }
}

#[cfg(test)]
mod tests {
  use crate::algo::greedy_heuristic::GreedyHeuristic;
  use crate::feedback_arc_set::FeedbackArcSet;
  use petgraph::algo::is_cyclic_directed;
  use petgraph::dot::{Config, Dot};
  use petgraph::graph::{DiGraph, EdgeIndex, EdgeReference};
  use petgraph::visit::EdgeRef;
  use petgraph::Graph;

  #[test]
  fn deterministic_on_simple_clique() {
    let clique = DiGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 1)]);

    let edges = GreedyHeuristic {}.compute(&clique);

    assert_eq!(edges.len(), 1);
    assert_eq!(edges.get(0).unwrap().id(), EdgeIndex::new(1));
  }

  #[test]
  fn works_on_multiple_cliques() {
    let cyclic_graph = DiGraph::<i32, ()>::from_edges(&[
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

    print_dot(&cyclic_graph);

    let removable_edges = GreedyHeuristic {}.compute(&cyclic_graph);
    let acyclic_graph = remove_edges(&cyclic_graph, &removable_edges);

    print_dot(&acyclic_graph);

    assert_eq!(removable_edges.len(), 4);
    assert!(!is_cyclic_directed(&acyclic_graph));
  }

  fn remove_edges(
    cyclic_graph: &Graph<i32, ()>,
    removable_edges: &Vec<EdgeReference<()>>,
  ) -> Graph<i32, ()> {
    let mut acyclic_graph = cyclic_graph.clone();
    for edge in removable_edges.as_slice() {
      acyclic_graph.remove_edge(edge.id());
    }
    acyclic_graph
  }

  fn print_dot(graph: &Graph<i32, ()>) {
    println!(
      "{:?}",
      // zeigt in IntelliJ Fehler (Dot doesn't implement Debug) an, aber läuft.
      Dot::with_config(graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    );
  }
}
