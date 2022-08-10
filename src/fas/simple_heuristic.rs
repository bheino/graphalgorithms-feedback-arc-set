use crate::fas::feedback_arc_set::FeedbackArcSet;
use crate::graph::hash_table::{Direction, Edge, HashTable};
use std::collections::HashSet;

/*
A feedback arc set of size no more than 1/2 |E| can be
obtained using the following heuristic (Berger and Shor, 1990):

F := empty ;
while G != empty do
    select a vertex v in G;
    if d−(v) < d+(v) then
        add all arcs incoming to v to F;
    else
        add all arcs outgoing from v to F;
    remove v and all arcs incident to it from G
return F.
 */
pub struct SimpleHeuristic<'a> {
  graph: &'a HashTable,
}

impl<'a> SimpleHeuristic<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl<'a> FeedbackArcSet for SimpleHeuristic<'a> {
  fn feedback_arc_set(&self) -> HashSet<Edge> {
    let mut graph = self.graph.clone();
    let mut fas = HashSet::new();

    while graph.order() > 0 {
      let v = graph.random_vertex();
      let edges_in = graph.edges(v, Direction::Inbound);
      let edges_out = graph.edges(v, Direction::Outbound);

      if edges_in.len() < edges_out.len() {
        fas.extend(edges_in);
      } else {
        fas.extend(edges_out)
      }

      graph.remove_vertex(v);
    }

    // Nachbedingung: A feedback arc set of size no more than 1/2 |E|
    debug_assert!(
      fas.len() <= self.graph.edge_count() / 2,
      "fas = {}, max = {}",
      fas.len(),
      self.graph.edge_count()
    );
    fas
  }
}

#[cfg(test)]
mod tests {
  use crate::fas::feedback_arc_set::tests::test_feedback_arc_set;
  use crate::fas::feedback_arc_set::FeedbackArcSet;
  use crate::fas::simple_heuristic::SimpleHeuristic;
  use crate::graph::hash_table::HashTable;
  use crate::tools::graphs::{graph_from_file, graph_with_multiple_cliques};
  use std::collections::HashSet;

  #[test]
  fn works_on_simple_clique() {
    let edges = [(0, 1), (1, 2), (2, 0)];
    let clique = HashTable::from_edges(&edges);

    let fas = SimpleHeuristic { graph: &clique }.feedback_arc_set();

    assert_eq!(fas.len(), 1);
    assert!(fas.is_subset(&HashSet::from(edges)));
  }

  #[test]
  fn works_on_multiple_cliques() {
    let cyclic_graph = graph_with_multiple_cliques();
    let algorithm = SimpleHeuristic {
      graph: &cyclic_graph,
    };

    test_feedback_arc_set(algorithm, &cyclic_graph);
  }

  #[test]
  fn works_on_h_001() {
    let cyclic_graph = graph_from_file("h_001");
    let algorithm = SimpleHeuristic {
      graph: &cyclic_graph,
    };
    test_feedback_arc_set(algorithm, &cyclic_graph);
  }

  #[test]
  fn works_on_h_025() {
    let cyclic_graph = graph_from_file("h_025");
    let algorithm = SimpleHeuristic {
      graph: &cyclic_graph,
    };
    test_feedback_arc_set(algorithm, &cyclic_graph);
  }
}
