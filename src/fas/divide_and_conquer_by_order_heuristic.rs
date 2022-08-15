use crate::fas::feedback_arc_set::FeedbackArcSet;
use crate::graph::hash_table::{Edge, HashTable, VertexId};
use crate::ordering::topological_sort::{leftward_edges, TopologicalSort};
use std::collections::HashSet;

/*
Another heuristic by Eades, Smyth and Lin (ESL) (1989) finds a feedback arc set of
all leftward arcs in a vertex ordering obtained using the following divide-and-conquer
procedure:
order(G)
    if G has no arcs then
        S := any vertex sequence
    else if G has an odd number of vertices then
        let v be a vertex of minimal indegree in G;
        remove v and all arcs incident to it from G;
        S1 := order(G);
        prepend v to S1 to form S
    else
        sort vertices of G into non-decreasing indegree order v1 , . . . , vn ;
        G 1 := subgraph of G induced by v1 , . . . , vn/2 ;
        G 2 := subgraph of G induced by vn/2+1 , . . . , vn ;
        S1 := order(G1 );
        S2 := order(G2 );
        concatenate S1 with S2 to form S
return S.
 */
pub struct DivideAndConquerByOrderHeuristic<'a> {
  pub graph: &'a HashTable,
}

impl<'a> DivideAndConquerByOrderHeuristic<'a> {
  fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl FeedbackArcSet for DivideAndConquerByOrderHeuristic<'_> {
  fn feedback_arc_set(&self) -> HashSet<Edge> {
    let ordering = order(self.graph.clone());
    debug_assert_eq!(self.graph.vertices().len(), ordering.len());

    leftward_edges(self.graph, ordering)
  }
}

fn order(mut g: HashTable) -> Vec<VertexId> {
  let s;
  let edge_count = g.edge_count();
  let sorted = TopologicalSort::new(&g).sort_by_indegree_asc();

  if g.edge_count() == 0 {
    s = g.vertices();
  } else if edge_count % 2 == 1 {
    let v = sorted.first().unwrap();
    g.remove_vertex(*v);

    let mut s1 = order(g.clone());
    s1.insert(0, *v);

    s = s1;
  } else {
    let first_half = &sorted[0..(sorted.len() / 2)];
    let second_half = &sorted[(sorted.len() / 2)..sorted.len()];

    let g1 = HashTable::from_graph(&g, first_half);
    let g2 = HashTable::from_graph(&g, second_half);

    let mut s1 = order(g1);
    let s2 = order(g2);

    s1.extend(s2);
    s = s1;
  }

  s
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fas::feedback_arc_set::tests::fas_tests;
  fas_tests!(DivideAndConquerByOrderHeuristic, [h_001, h_025]);
}
