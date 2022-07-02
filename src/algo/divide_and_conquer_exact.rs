use crate::feedback_arc_set::FeedbackArcSet;
use petgraph::algo::tarjan_scc;
use petgraph::graph::GraphIndex;
use petgraph::visit::{
  GraphProp, IntoEdgeReferences, IntoNeighbors, IntoNodeIdentifiers, NodeCount, NodeIndexable,
};
use petgraph::Directed;

struct DivideAndConquerExact {}

impl FeedbackArcSet for DivideAndConquerExact {
  fn compute_fas<G>(&self, g: G) -> Vec<G::EdgeRef>
  where
    G: IntoEdgeReferences + GraphProp<EdgeType = Directed>,
    G::NodeId: GraphIndex,
    G: NodeCount,
    G: IntoNodeIdentifiers + IntoNeighbors + NodeIndexable,
  {
    // todo("A divide-and-conquer algorithm that tests all partitions of the vertices into two equal subsets and recurses within each subset can solve the problem in time {\displaystyle O(4^{n}/{\sqrt {n}})}{\displaystyle O(4^{n}/{\sqrt {n}})}, using polynomial space.")
    /*
    fas(G)
      P:= scc(G);
      if P has only one element then {G is strongly connected}
        (V1 , V2 ) := bisect(G);
        F := fas(G[V1 ]) ∪ fas(G[V2 ]) ∪ {i → j : i ∈ V2 and j ∈ V1 }
      else
        F := ;
        for each S ∈ P do
        F := F ∪ fas(G[S])
      return F.
         */
    let partition = tarjan_scc(g);
    let mut fas: Vec<G::EdgeRef> = vec![];
    if partition.len() == 1 {
      let bisection = StochasticEvolution {}; // or DynamicClustering {}
      let (v1, v2) = bisection.compute_bisect(&g);
      let mut fas_v1 = self.compute_fas(&v1);
      let mut fas_v2 = self.compute_fas(&v2);

      fas.append(&mut fas_v1);
      fas.append(&mut fas_v2);
      todo!("F := fas(G[v1 ]) ∪ fas(G[v2 ]) ∪ {{i → j : i ∈ v2 and j ∈ v1 }}")
    } else {
      fas = vec![];
      for scc in partition.into_iter() {
        todo!("F := F ∪ fas(G[S])");
      }
    }

    fas
  }
}

trait GraphBisection {
  fn compute_bisect<G>(&self, graph: G) -> (G, G)
  where
    G: GraphProp<EdgeType = Directed>
      + IntoEdgeReferences
      + IntoNeighbors
      + IntoNodeIdentifiers
      + NodeCount
      + NodeIndexable;
}

struct StochasticEvolution {}
impl GraphBisection for StochasticEvolution {
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

struct DynamicClustering {}
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
