use crate::fas::feedback_arc_set::{FeedbackArcSet, Graph};
use petgraph::algo::feedback_arc_set::greedy_feedback_arc_set;
use petgraph::stable_graph::EdgeReference;

pub struct GreedyHeuristic {}

impl FeedbackArcSet for GreedyHeuristic {
  fn compute_fas<'a>(&'a self, graph: &'a Graph) -> Vec<EdgeReference<'_, ()>> {
    greedy_feedback_arc_set(graph).collect()
  }
}
