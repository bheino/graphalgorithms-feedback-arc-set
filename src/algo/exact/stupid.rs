use itertools::Itertools;
use petgraph::{algo::is_cyclic_directed, prelude::*, visit::IntoEdgeReferences};

pub struct Stupid;

impl Stupid {
  pub fn compute_fas(
    graph: &StableDiGraph<i32, ()>,
  ) -> Vec<petgraph::stable_graph::EdgeReference<'_, ()>> {
    for idx in 1..=graph.edge_count() {
      for permutation in graph.edge_references().permutations(idx) {
        let mut acyclic_graph = graph.clone();

        permutation.iter().for_each(|edge| {
          acyclic_graph.remove_edge(edge.id());
        });

        if !is_cyclic_directed(&acyclic_graph) {
          return permutation;
        }
      }
    }

    vec![]
  }
}

#[cfg(test)]
mod test {
  use petgraph::{
    dot::{Config, Dot},
    stable_graph::StableDiGraph,
  };

  use crate::{algo::exact::stupid::Stupid, tools::metis::graph_from_file};

  #[test]
  fn it_works() {
    let graph = StableDiGraph::<i32, ()>::from_edges(&[
      (0, 1),
      (0, 7),
      (1, 2),
      (1, 3),
      (2, 4),
      (2, 5),
      (2, 6),
      (3, 7),
      (6, 8),
      (6, 9),
      (7, 9),
      (5, 10),
      (8, 10),
      (9, 10),
      (4, 11),
      (4, 12),
      (12, 11),
      (10, 13),
      (11, 13),
      (10, 14),
      (14, 15),
      (14, 16),
      (16, 15),
      (16, 17),
      (17, 18),
      (12, 18),
      // Ab hier kommen Zyklen rein
      (13, 2),
      (7, 1),
      (6, 7),
      (15, 10),
      (15, 13),
    ]);

    println!(
      "{:?}",
      Dot::with_config(&graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    );

    println!("{:?}", Stupid::compute_fas(&graph));
  }

  #[test]
  fn works_on_e_001() {
    let graph = graph_from_file("test/resources/exact/e_001_with_comments");

    println!(
      "{:?}",
      Dot::with_config(&graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    );

    println!("{:?}", Stupid::compute_fas(&graph));
  }
}
