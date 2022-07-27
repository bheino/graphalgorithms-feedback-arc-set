use petgraph::stable_graph::StableDiGraph;

pub trait GraphBisection {
  fn compute_bisect<'a>(
    &self,
    graph: &'a StableDiGraph<i32, ()>,
  ) -> (&'a StableDiGraph<i32, ()>, &'a StableDiGraph<i32, ()>);
}
