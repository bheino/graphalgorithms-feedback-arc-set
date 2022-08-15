use crate::graph::hash_table::Edge;
use std::collections::HashSet;

pub trait FeedbackArcSet {
  fn feedback_arc_set(&self) -> HashSet<Edge>;
}

#[cfg(test)]
pub(crate) mod tests {
  macro_rules! fas_tests {
    (
      $algo: ident,
      [$($file_name:ident),*]
    ) => {
      fn test_feedback_arc_set(algorithm: &$algo, cyclic_graph: &HashTable) {
        let mut acyclic_graph = cyclic_graph.clone();
        let fas = algorithm.feedback_arc_set();
        fas.into_iter().for_each(|e| acyclic_graph.remove_edge(e));

        if acyclic_graph.is_cyclic() {
          let print_dot = |prefix, graph| {
            println!("{}", prefix);
            println!("{}", crate::tools::dot::Dot::new(graph));
          };

          print_dot("Cyclic Graph:", &cyclic_graph);
          print_dot("Acyclic Graph:", &acyclic_graph);

          panic!("Graph still has cycles!");
        }
      }

      $(
        paste::paste! {
          #[test]
          fn [<works_on_ $file_name>]() {
            let cyclic_graph = crate::tools::graphs::graph_from_file(stringify!($file_name));
            let algorithm = $algo {
              graph: &cyclic_graph,
            };

            test_feedback_arc_set(&algorithm, &cyclic_graph);
          }
        }
      )*

      #[test]
      fn works_on_wikipedia_scc() {
        let mut cyclic_graph = crate::tools::graphs::graph_from_wikipedia_scc();
        assert!(cyclic_graph.is_cyclic());

        let algorithm = $algo {
              graph: &cyclic_graph,
            };
        let fas = algorithm.feedback_arc_set();

        fas.into_iter().for_each(|e| cyclic_graph.remove_edge(e));

        assert!(!cyclic_graph.is_cyclic())
      }

      #[test]
      fn works_on_simple_clique() {
        let edges = [(0, 1), (1, 2), (2, 0)];
        let clique = HashTable::from_edges(&edges);
        let fas = $algo { graph: &clique }.feedback_arc_set();

        assert_eq!(fas.len(), 1);
        assert!(fas.is_subset(&HashSet::from(edges)));
      }

      #[test]
      fn works_on_multiple_cliques() {
        let clique = crate::tools::graphs::graph_with_multiple_cliques();
        let algorithm = $algo { graph: &clique };
        test_feedback_arc_set(&algorithm, &clique);
      }
    };
  }
  pub(crate) use fas_tests;
}
