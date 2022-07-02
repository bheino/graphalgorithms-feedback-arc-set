use crate::feedback_arc_set::FeedbackArcSet;
use petgraph::stable_graph::GraphIndex;
use petgraph::visit::{GraphProp, IntoEdgeReferences, NodeCount};
use petgraph::Directed;

pub struct GreedyHeuristic {}

impl FeedbackArcSet for GreedyHeuristic {
  fn compute_fas<G>(&self, g: G) -> Vec<G::EdgeRef>
  where
    G: IntoEdgeReferences + GraphProp<EdgeType = Directed>,
    G::NodeId: GraphIndex,
    G: NodeCount,
  {
    petgraph::algo::feedback_arc_set::greedy_feedback_arc_set(g).collect()
  }
}
