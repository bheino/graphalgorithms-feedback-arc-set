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
      SimpleHeuristic,
      GreedyHeuristic,
      DivideAndConquerByOrderHeuristic
    ],
    [h_001, h_025]
  );
}

criterion_group! {
  name = benches;
  config = Criterion::default();
  targets = file_benchmarks
}

criterion_main!(benches);

macro_rules! generate_benchmarks {
  (
    $bencher: expr,
    $name: expr,
    [$($algo: ident),*],
    $args:tt
  ) => {
    let mut group = $bencher.benchmark_group($name.to_string());
    group.sampling_mode(criterion::SamplingMode::Flat);
    $(
      generate_benchmarks!(@call group, $algo, $args);
    )*
    group.finish()
  };
  (@call $bencher:expr, $algo:ident, [$($file_name:ident),*]) => {
    paste::paste! {
      $(
        let cyclic_graph = graph_from_file(stringify!($file_name));
        let algo = $algo {
          graph: &cyclic_graph,
        };
        $bencher.bench_function(stringify!([<$algo _$file_name>]), |b| {
          b.iter(|| criterion::black_box(algo.feedback_arc_set()))
        });
      )*
    }
  }
}

pub(crate) use generate_benchmarks;
