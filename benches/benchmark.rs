use criterion::{criterion_group, criterion_main, Criterion};
use graphalgorithms_feedback_arc_set::algo::greedy_heuristic::GreedyHeuristic;
use graphalgorithms_feedback_arc_set::feedback_arc_set::FeedbackArcSet;
use graphalgorithms_feedback_arc_set::tools::metis::graph_from_file;

pub fn criterion_benchmark(c: &mut Criterion) {
  let cyclic_graph = graph_from_file("test/resources/heuristic/h_025");
  let algo = GreedyHeuristic {};
  c.bench_function("greedy heuristic h_025", |b| {
    b.iter(|| algo.compute_fas(&cyclic_graph))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
