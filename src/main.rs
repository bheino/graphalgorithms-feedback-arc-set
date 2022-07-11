use std::time::Instant;

use graphalgorithms_feedback_arc_set::{
  algo::exact::stupid::Stupid, tools::metis::graph_from_file,
};
use petgraph::dot::{Config, Dot};

fn main() {
  let graph = graph_from_file("test/resources/exact/e_025");

  println!(
    "{:?}",
    Dot::with_config(&graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
  );

  let now = Instant::now();

  println!("{:?}", Stupid::compute_fas(&graph));

  println!("{:?}", now.elapsed().as_secs())
}
