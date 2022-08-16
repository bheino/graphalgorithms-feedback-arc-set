use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use graphalgorithms_feedback_arc_set::{
  fas::{
    divide_and_conquer_by_order_heuristic::DivideAndConquerByOrderHeuristic,
    feedback_arc_set::FeedbackArcSet, greedy::GreedyHeuristic, simple_heuristic::SimpleHeuristic,
  },
  tools::graphs::graph_from_file,
};

pub fn file_benchmarks(c: &mut Criterion) {
  generate_benchmarks!(
    c,
    "File Benchmarks",
    [
      h_001, h_003, h_005, h_007, h_009, h_011, h_013, h_015, h_017, h_019, h_021, h_023, h_025,
      h_027, h_029
    ],
    [
      SimpleHeuristic,
      GreedyHeuristic,
      DivideAndConquerByOrderHeuristic
    ]
  );
}

criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(30));
  targets = file_benchmarks
}

criterion_main!(benches);

macro_rules! generate_benchmarks {
  (
    $bencher: expr,
    $name: expr,
    [$($file_name: ident),*],
    $args:tt
  ) => {
    $(
      let mut group = $bencher.benchmark_group(stringify!($file_name));
      let cyclic_graph = graph_from_file(stringify!($file_name));
      generate_benchmarks!(@call group, cyclic_graph, $file_name, $args);
      group.finish();
    )*

  };
  (@call $bencher:expr, $graph:expr, $file_name:ident, [$($algo:ident),*]) => {
    paste::paste! {
      $(
        let algo = $algo {
          graph: &$graph,
        };
        $bencher.bench_function(stringify!([<$algo _$file_name>]), |b| {
          b.iter(|| criterion::black_box(algo.feedback_arc_set()))
        });
      )*
    }
  }
}

pub(crate) use generate_benchmarks;
