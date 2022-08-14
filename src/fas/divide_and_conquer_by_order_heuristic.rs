use crate::fas::feedback_arc_set::FeedbackArcSet;
use crate::graph::hash_table::{Direction, Edge, HashTable, VertexId};
use crate::ordering::topological_sort::TopologicalSort;
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
  graph: &'a HashTable,
}

impl<'a> DivideAndConquerByOrderHeuristic<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl<'a> FeedbackArcSet for DivideAndConquerByOrderHeuristic<'a> {
  fn feedback_arc_set(&self) -> HashSet<Edge> {
    let ordering = order(self.graph.clone());

    collect_leftward_edges(self.graph, ordering)
  }
}

fn collect_leftward_edges(graph: &HashTable, ordering: Vec<VertexId>) -> HashSet<Edge> {
  let mut leftward_edges = HashSet::new();

  for v in ordering {
    for edge in graph.edges(v, Direction::Outbound) {
      if edge.1 < v {
        leftward_edges.insert(edge);
      }
    }
  }

  leftward_edges
}

fn order(mut g: HashTable) -> Vec<VertexId> {
  let ordering;
  let edge_count = g.edge_count();

  if g.edge_count() == 0 {
    ordering = g.vertices();
  } else if edge_count % 2 == 1 {
    let v = vertex_with_min_indegree(&g);
    g.remove_vertex(v);

    let mut ordering1 = order(g);
    ordering1.insert(0, v);

    ordering = ordering1;
  } else {
    let sorted = TopologicalSort::new(&g).sort_by_indegree_asc();
    let g1 = subgraph(&g, &sorted[0..sorted.len() / 2]);
    let g2 = subgraph(&g, &sorted[sorted.len() / 2..sorted.len()]);
    let mut s1 = order(g1);
    let s2 = order(g2);
    s1.extend(s2);
    ordering = s1;
  }

  ordering
}

fn subgraph(graph: &HashTable, vertices_to_keep: &[VertexId]) -> HashTable {
  let edges = graph
    .vertices()
    .into_iter()
    .flat_map(|v| graph.edges(v, Direction::Outbound))
    .filter(|edge| vertices_to_keep.contains(&edge.0) && vertices_to_keep.contains(&edge.1))
    .collect::<Vec<_>>();

  HashTable::from_edges(edges.as_slice())
}

fn vertex_with_min_indegree(graph: &HashTable) -> VertexId {
  graph
    .vertices()
    .iter()
    .map(|v| (*v, graph.edges(*v, Direction::Inbound).len()))
    .min_by(|v1, v2| (*v1).1.cmp(&(*v2).1))
    .unwrap()
    .0
}

#[cfg(test)]
mod tests {
  use crate::fas::divide_and_conquer_by_order_heuristic::DivideAndConquerByOrderHeuristic;
  use crate::fas::feedback_arc_set::tests::test_feedback_arc_set;
  use crate::fas::feedback_arc_set::FeedbackArcSet;
  use crate::graph::hash_table::HashTable;
  use crate::tools::graphs::{
    graph_from_file, graph_from_wikipedia_scc, graph_with_multiple_cliques,
  };
  use std::collections::HashSet;

  #[test]
  fn works_on_simple_clique() {
    let edges = [(0, 1), (1, 2), (2, 0)];
    let clique = HashTable::from_edges(&edges);

    let fas = DivideAndConquerByOrderHeuristic { graph: &clique }.feedback_arc_set();

    assert_eq!(fas.len(), 1);
    assert!(fas.is_subset(&HashSet::from(edges)));
  }

  #[test]
  fn works_on_multiple_cliques() {
    let cyclic_graph = graph_with_multiple_cliques();
    let algorithm = DivideAndConquerByOrderHeuristic {
      graph: &cyclic_graph,
    };
    test_feedback_arc_set(algorithm, &cyclic_graph);
  }

  #[test]
  fn works_on_h_001() {
    let cyclic_graph = graph_from_file("h_001");
    let algorithm = DivideAndConquerByOrderHeuristic {
      graph: &cyclic_graph,
    };
    test_feedback_arc_set(algorithm, &cyclic_graph);
  }

  #[test]
  fn works_on_h_025() {
    let cyclic_graph = graph_from_file("h_025");
    let algorithm = DivideAndConquerByOrderHeuristic {
      graph: &cyclic_graph,
    };
    test_feedback_arc_set(algorithm, &cyclic_graph);
  }

  #[test]
  fn works_on_wikipedia_scc() {
    let cyclic_graph = graph_from_wikipedia_scc();
    let algorithm = DivideAndConquerByOrderHeuristic {
      graph: &cyclic_graph,
    };
    test_feedback_arc_set(algorithm, &cyclic_graph);
  }
}
