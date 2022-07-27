use petgraph::visit::{
  GraphProp, IntoEdgeReferences, IntoNeighbors, IntoNodeIdentifiers, NodeCount, NodeIndexable,
};
use petgraph::{Directed, EdgeType};

pub trait GraphBisection {
  fn compute_bisect<G>(&self, graph: G) -> (G, G)
  where
    G: GraphProp<EdgeType = Directed>
      + IntoEdgeReferences
      + IntoNeighbors
      + IntoNodeIdentifiers
      + NodeCount
      + NodeIndexable;
}
