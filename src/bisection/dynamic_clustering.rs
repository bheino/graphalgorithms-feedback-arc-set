use crate::bisection::graph_bisection::GraphBisection;
use petgraph::visit::{
  GraphProp, IntoEdgeReferences, IntoNeighbors, IntoNodeIdentifiers, NodeCount, NodeIndexable,
};
use petgraph::Directed;

pub struct DynamicClustering {}
impl GraphBisection for DynamicClustering {
  fn compute_bisect<G>(&self, graph: G) -> (G, G)
  where
    G: GraphProp<EdgeType = Directed>
      + IntoEdgeReferences
      + IntoNeighbors
      + IntoNodeIdentifiers
      + NodeCount
      + NodeIndexable,
  {
    todo!()
  }
}
