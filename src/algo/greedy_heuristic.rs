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
  use petgraph::graph::{DiGraph, EdgeIndex};
  use petgraph::visit::EdgeRef;

  #[test]
  fn deterministic_on_simple_clique() {
    let clique = DiGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 1)]);

    let edges = GreedyHeuristic {}.compute(&clique);

    assert_eq!(edges.len(), 1);
    assert_eq!(edges.get(0).unwrap().id(), EdgeIndex::new(1));
  }
}
