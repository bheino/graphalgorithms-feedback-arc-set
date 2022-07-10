use petgraph::{
  graph::GraphIndex,
  visit::{
    GraphProp, IntoEdgeReferences, IntoNeighbors, IntoNodeIdentifiers, NodeCount, NodeIndexable,
  },
  Directed,
};

pub mod stupid;

trait FeedbackArcSet<G>
where
  G: IntoEdgeReferences + GraphProp<EdgeType = Directed>,
  G::NodeId: GraphIndex,
  G: NodeCount,
  G: IntoNodeIdentifiers + IntoNeighbors + NodeIndexable,
{
  fn compute(graph: G) -> Vec<G::EdgeRef>;
}
