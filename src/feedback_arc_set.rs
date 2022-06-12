use petgraph::graph::GraphIndex;
use petgraph::visit::{GraphProp, IntoEdgeReferences, NodeCount};
use petgraph::Directed;

pub trait FeedbackArcSet {
  /// Finds a feedback arc set: a set of edges in the given directed graph, which when removed, make the graph acyclic.
  fn compute<G>(&self, g: G) -> Vec<G::EdgeRef>
  where
    G: IntoEdgeReferences + GraphProp<EdgeType = Directed>,
    G::NodeId: GraphIndex,
    G: NodeCount;
}
