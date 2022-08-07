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
  use crate::fas::simple_heuristic::SimpleHeuristic;
  use crate::graph::hash_table::{Edge, HashTable};
  use crate::tools::cycle::CycleDetection;
  use crate::tools::dot::Dot;
  use crate::tools::metis::graph_from_file;
  use std::collections::HashSet;
  use std::ops::Range;

  #[test]
  fn works_on_simple_clique() {
    let edges = [(1, 2), (2, 3), (3, 1)];
    let clique = HashTable::from_edges(3, &edges);

    let fas = SimpleHeuristic { graph: &clique }.feedback_arc_set();

    assert_eq!(fas.len(), 1);
    assert!(fas.is_subset(&HashSet::from(edges)));
  }

  #[test]
  fn works_on_multiple_cliques() {
    let edges = [
      (0, 1),
      (0, 7),
      (1, 2),
      (1, 3),
      (2, 4),
      (2, 5),
      (2, 6),
      (3, 7),
      (6, 8),
      (6, 9),
      (7, 9),
      (5, 10),
      (8, 10),
      (9, 10),
      (4, 11),
      (4, 12),
      (12, 11),
      (10, 13),
      (11, 13),
      (10, 14),
      (14, 15),
      (14, 16),
      (16, 15),
      (16, 17),
      (17, 18),
      (12, 18),
      // Ab hier kommen Zyklen rein
      (13, 2),
      (7, 1),
      (6, 7),
      (15, 10),
      (15, 13),
    ];
    let cyclic_graph = HashTable::from_edges(19, &edges);

    test_feedback_arc_set(&cyclic_graph, 4..(5 * 4), false, true);
  }

  #[test]
  fn works_on_h_001() {
    let cyclic_graph = graph_from_file("test/resources/heuristic/h_001");
    test_feedback_arc_set(&cyclic_graph, 143..(5 * 143), false, false);
  }

  #[test]
  fn works_on_h_025() {
    let cyclic_graph = graph_from_file("test/resources/heuristic/h_025");
    test_feedback_arc_set(&cyclic_graph, 1574..(5 * 1574), false, false);
  }

  fn test_feedback_arc_set(
    cyclic_graph: &HashTable,
    expected_set_range: Range<usize>,
    should_print_edges: bool,
    should_print_dot: bool,
  ) {
    let is_cyclic = |graph: &HashTable| -> bool { CycleDetection::new(graph).is_cyclic() };
    assert!(is_cyclic(cyclic_graph));
    let algorithm = SimpleHeuristic {
      graph: cyclic_graph,
    };

    if should_print_dot {
      let print_dot = |prefix, graph| {
        println!("{}", prefix);
        println!("{}", Dot::new(graph));
      };
      print_dot("Cyclic Graph:", cyclic_graph)
    };

    let removable_edges = algorithm.feedback_arc_set();
    if should_print_edges {
      let print_edges = |edges: &HashSet<Edge>| {
        println!("Edges to be removed:");
        edges
          .iter()
          .for_each(|(source, target)| println!("\t{:?} -> {:?}", source, target));
        println!();
      };
      print_edges(&removable_edges);
    }

    let remove_edges = |cyclic_graph: &HashTable, edges: &HashSet<Edge>| {
      let mut acyclic_graph = cyclic_graph.clone();
      for edge in edges {
        acyclic_graph.remove_edge(*edge);
      }
      acyclic_graph
    };

    let acyclic_graph = remove_edges(cyclic_graph, &removable_edges);
    if should_print_dot {
      let print_dot = |prefix, graph| {
        println!("{}", prefix);
        println!("{}", Dot::new(graph));
      };
      print_dot("Acyclic Graph:", &acyclic_graph);
    }

    assert!(expected_set_range.contains(&removable_edges.len()));
    // TODO assert!(!is_cyclic(&acyclic_graph));
  }
}
