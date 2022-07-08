pub struct APFAS<'a, G>
where
  G:
    petgraph::visit::IntoEdgeReferences + petgraph::visit::GraphProp<EdgeType = petgraph::Directed>,
  G::NodeId: petgraph::graph::GraphIndex,
  G: petgraph::visit::NodeCount,
  G: petgraph::visit::IntoNodeIdentifiers
    + petgraph::visit::IntoNeighbors
    + petgraph::visit::NodeIndexable,
{
  graph: &'a G,
}

impl<G> APFAS<'_, G>
where
  G:
    petgraph::visit::IntoEdgeReferences + petgraph::visit::GraphProp<EdgeType = petgraph::Directed>,
  G::NodeId: petgraph::graph::GraphIndex,
  G: petgraph::visit::NodeCount,
  G: petgraph::visit::IntoNodeIdentifiers
    + petgraph::visit::IntoNeighbors
    + petgraph::visit::NodeIndexable,
{
  fn compute_fas() {}

  fn mdfs(graph: &G) -> G {
    todo!()
  }

  fn back_track(vertex_set: Vec<u32>, vi: u32, vk: u32) {
    todo!()
  }

  fn least_fas(graph: &G) -> G
  where
    G: petgraph::visit::IntoEdgeReferences
      + petgraph::visit::GraphProp<EdgeType = petgraph::Directed>,
    G::NodeId: petgraph::graph::GraphIndex,
    G: petgraph::visit::NodeCount,
    G: petgraph::visit::IntoNodeIdentifiers
      + petgraph::visit::IntoNeighbors
      + petgraph::visit::NodeIndexable,
  {
    todo!()
  }

  fn ap_fas(graph: G) -> G
  where
    G: petgraph::visit::IntoEdgeReferences
      + petgraph::visit::GraphProp<EdgeType = petgraph::Directed>,
    G::NodeId: petgraph::graph::GraphIndex,
    G: petgraph::visit::NodeCount,
    G: petgraph::visit::IntoNodeIdentifiers
      + petgraph::visit::IntoNeighbors
      + petgraph::visit::NodeIndexable,
  {
    todo!()
  }
}

#[cfg(test)]
mod test {
  #[test]
  pub fn test() {}
}
