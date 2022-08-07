use crate::graph::hash_table::{Direction, HashTable};
use std::fmt::{Display, Formatter, Result};

// Quelle: Beispiel-Lösungen zu Übungsaufgaben
pub struct Dot<'a> {
  graph: &'a HashTable,
}

impl<'a> Dot<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl<'a> Display for Dot<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "digraph {{")?;
    for v in self.graph.vertices() {
      for e in self.graph.edges(*v, Direction::Outbound) {
        writeln!(f, "\t {} -> {};", e.0, e.1)?;
      }
    }
    writeln!(f, "}}")
  }
}

#[cfg(test)]
pub mod tests {
  use crate::graph::hash_table::HashTable;
  use crate::tools::dot::Dot;

  #[test]
  fn test_dot() {
    let graph = HashTable::complete(4);
    let dot: Dot = Dot::new(&graph);
    println!("{}", dot);
  }
}
