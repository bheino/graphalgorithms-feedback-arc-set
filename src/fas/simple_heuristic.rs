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

  pub fn feedback_arc_set(&self) -> HashSet<Edge> {
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

    fas
  }
}

#[cfg(test)]
mod tests {
  use crate::fas::simple_heuristic::SimpleHeuristic;
  use crate::graph::hash_table::HashTable;
  use std::collections::HashSet;

  #[test]
  fn deterministic_on_simple_clique() {
    let edges = [(1, 2), (2, 3), (3, 1)];
    let clique = HashTable::from_edges(&edges);

    let fas = SimpleHeuristic { graph: &clique }.feedback_arc_set();

    assert_eq!(fas.len(), 1);
    assert!(fas.is_subset(&HashSet::from(edges)));
  }
}
