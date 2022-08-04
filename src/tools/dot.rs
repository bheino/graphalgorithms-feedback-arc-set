use crate::graph::hash_table::{Direction, HashTable, VertexId};
use std::fmt::{Display, Formatter, Result};

// Quelle: Beispiel-Lösungen zu Übungsaufgaben
pub struct Dot<'a> {
  vertex_colors: Vec<String>,
  graph: &'a HashTable,
}

impl<'a> Dot<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self {
      vertex_colors: vec!["black".to_string(); graph.order() as usize],
      graph,
    }
  }

  pub fn set_color(&mut self, u: VertexId, color: String) {
    self.vertex_colors[u as usize] = color;
  }
}

impl<'a> Display for Dot<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "Digraph {{")?;
    for u in 0..self.graph.order() {
      writeln!(f, "\t {} [color={}];", u, self.vertex_colors[u as usize])?;
    }
    for v in self.graph.vertices() {
      for e in self.graph.edges(v, Direction::Outbound) {
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
