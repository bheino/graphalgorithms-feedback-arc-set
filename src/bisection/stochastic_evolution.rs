use crate::bisection::graph_bisection::GraphBisection;
use petgraph::stable_graph::StableDiGraph;

pub struct StochasticEvolution {}
impl GraphBisection for StochasticEvolution {
  fn compute_bisect<'a>(
    &self,
    graph: &'a StableDiGraph<i32, ()>,
  ) -> (&'a StableDiGraph<i32, ()>, &'a StableDiGraph<i32, ()>) {
    todo!()
  }
}
