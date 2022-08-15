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
    [
      SimpleHeuristic,
      GreedyHeuristic,
      DivideAndConquerByOrderHeuristic
    ],
    [h_001, h_025]
  );
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = file_benchmarks
}

criterion_main!(benches);

macro_rules! generate_benchmarks {
  (
    $bencher: expr,
    [$($algo: ident),*],
    $args:tt
  ) => {
    $(
      generate_benchmarks!(@call $bencher, $algo, $args);
    )*
  };
  (@call $bencher:expr, $algo:ident, [$($file_name:ident),*]) => {
    paste::paste! {
      $(
        let cyclic_graph = graph_from_file(stringify!($file_name));
        let algo = $algo {
          graph: &cyclic_graph,
        };
        $bencher.bench_function(stringify!([<$algo _$file_name>]), |b| {
          b.iter(|| algo.feedback_arc_set())
        });
      )*
    }
  }
}

pub(crate) use generate_benchmarks;
