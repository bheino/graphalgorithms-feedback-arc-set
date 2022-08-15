use crate::bisection::stochastic_evolution::StochasticEvolution;
use crate::fas::feedback_arc_set::FeedbackArcSet;
use crate::graph::hash_table::{Edge, HashTable};
use crate::scc::strongly_connected_components::StronglyConnectedComponents;
use crate::scc::tarjan::Tarjan;
use std::collections::HashSet;

/*
Function fas returns a feedback arc set F of its input graph G(V, E). If G is not strongly
connected, then F is computed as the union of sets each of which is a feedback arc set in
one strongly connected component of G. If G is strongly connected, then function bisect is
used to decompose the vertex set of G into two subsets V1 and V2 of about equal size. The
set F is then computed as the union of a feedback set of F1 of G[V1 ], a feedback set F2 of
(G[V2 ] and the set L = {i → j : i ∈ V2 and j ∈ V1 }. Function f as is efficient because the
input graph is quickly decomposed into smaller subgraphs either by function bisect or by
function scc.
 */
pub struct DivideAndConquerByBisectionHeuristic<'a> {
  pub graph: &'a HashTable,
}

impl<'a> DivideAndConquerByBisectionHeuristic<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl<'a> FeedbackArcSet for DivideAndConquerByBisectionHeuristic<'a> {
  fn feedback_arc_set(&self) -> HashSet<Edge> {
    let sc_components = Tarjan::new(self.graph).strongly_connected_components();
    debug_assert!(!sc_components.is_empty(), "No SCCs = {:?}", self.graph);
    let mut fas = HashSet::new();

    if sc_components.len() == 1 {
      let (v_1, v_2) = StochasticEvolution::new(self.graph).bisection();
      /* Doesn't make sense, although proposed by paper, because recursion would not terminate (fas on a bisection of an scc is simply one arc)...
            let graph_from_v_1 =
              HashTable::from_graph(self.graph, &v_1.iter().cloned().collect::<Vec<_>>());
            let graph_from_v_2 =
              HashTable::from_graph(self.graph, &v_2.iter().cloned().collect::<Vec<_>>());
            let fas_from_v_1 =
              DivideAndConquerByBisectionHeuristic::new(&graph_from_v_1).feedback_arc_set();
            let fas_from_v_2 =
              DivideAndConquerByBisectionHeuristic::new(&graph_from_v_2).feedback_arc_set();

            fas.extend(&fas_from_v_1);
            fas.extend(&fas_from_v_2);
      */
      fas.extend(self.graph.edges_from_to(&v_2, &v_1));
    } else {
      for scc in sc_components {
        let graph_from_scc =
          HashTable::from_graph(self.graph, &scc.iter().cloned().collect::<Vec<_>>());
        let fas_from_scc =
          DivideAndConquerByBisectionHeuristic::new(&graph_from_scc).feedback_arc_set();
        fas.extend(fas_from_scc);
      }
    }

    fas
  }
}

#[cfg(test)]
mod tests {
  // use super::*;
  // use crate::fas::feedback_arc_set::tests::fas_tests;
  // fas_tests!(DivideAndConquerByBisectionHeuristic, [h_001, h_025]);
}
