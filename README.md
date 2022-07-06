# graphalgorithms-feedback-arc-set
[![CI](https://github.com/boerdy/graphalgorithms-feedback-arc-set/actions/workflows/rust.yml/badge.svg)](https://github.com/boerdy/graphalgorithms-feedback-arc-set/actions/workflows/rust.yml)
[![Coverage](https://github.com/boerdy/graphalgorithms-feedback-arc-set/actions/workflows/coverage.yml/badge.svg)](https://github.com/boerdy/graphalgorithms-feedback-arc-set/actions/workflows/coverage.yml)

## Used Literature
- PACE 2022
  - Input format: https://pacechallenge.org/2022/tracks/#input-format
  - Example cyclic graphs: https://pacechallenge.org/2022/01/12/public-instances/
- Feedback-Arc-Set:
  - Problem Description: https://en.wikipedia.org/wiki/Feedback_arc_set
  - Exact (greedy) Algorithm: https://www.mat.univie.ac.at/~herman/fwf-P27891-N32/minimum_feedback_arc_set.pdf
  - Heuristic (divide and conquer) Algorithm: https://link.springer.com/content/pdf/10.1023/A:1011315014322.pdf
  - Heuristic (simple, using count of incoming/outgoing arcs) Algorithm: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.7745&rep=rep1&type=pdf
- Graph-Bisection (used in Feedback-Arc-Set):
  - Problem Description: https://tracer.lcc.uma.es/problems/bisect/bisect.htm
  - Heuristic (Stochastical Evolution) Algorithm: See FAS Heuristic Paper
  - Heuristic (Dynamic Clustering) Algorithm: See FAS Heuristic Paper
- Strongly-Connected-Components (used in Feedback-Arc-Set):
  - Problem Description: https://en.wikipedia.org/wiki/Strongly_connected_component
  - Kosaraju's Algorithm: https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm
  - Tarjan's Algorithm: https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm

## Used Libraries
- Benchmarking: https://docs.rs/criterion/latest/criterion/
- Graph datastructure & Greedy Feedbac Arc Set Algorithm: https://docs.rs/petgraph/latest/petgraph/
- Coverage: https://github.com/xd009642/tarpaulin

