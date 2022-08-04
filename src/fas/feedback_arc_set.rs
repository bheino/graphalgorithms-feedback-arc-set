use petgraph::stable_graph::{EdgeReference, StableDiGraph};

pub type Graph = StableDiGraph<i32, ()>;

pub trait FeedbackArcSet {
  /// Finds a feedback arc set: a set of edges in the given directed graph, which when removed, make the graph acyclic.
  fn compute_fas<'a>(&'a self, graph: &'a Graph) -> Vec<EdgeReference<'_, ()>>;
}
