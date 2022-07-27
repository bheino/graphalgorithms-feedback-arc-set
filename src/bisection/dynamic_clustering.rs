use crate::bisection::graph_bisection::GraphBisection;
use petgraph::stable_graph::StableDiGraph;

pub struct DynamicClustering {}
impl GraphBisection for DynamicClustering {
  fn compute_bisect<'a>(
    &self,
    graph: &'a StableDiGraph<i32, ()>,
  ) -> (&'a StableDiGraph<i32, ()>, &'a StableDiGraph<i32, ()>) {
    todo!()
  }
}
