use crate::feedback_arc_set::FeedbackArcSet;
use petgraph::graph::GraphIndex;
use petgraph::visit::{GraphProp, IntoEdgeReferences, NodeCount};
use petgraph::Directed;

struct DivideAndConquerExact {}

impl FeedbackArcSet for DivideAndConquerExact {
  fn compute<G>(&self, g: G) -> Vec<G::EdgeRef>
  where
    G: IntoEdgeReferences + GraphProp<EdgeType = Directed>,
    G::NodeId: GraphIndex,
    G: NodeCount,
  {
    // todo("A divide-and-conquer algorithm that tests all partitions of the vertices into two equal subsets and recurses within each subset can solve the problem in time {\displaystyle O(4^{n}/{\sqrt {n}})}{\displaystyle O(4^{n}/{\sqrt {n}})}, using polynomial space.")
    petgraph::algo::feedback_arc_set::greedy_feedback_arc_set(g).collect()
  }
}
