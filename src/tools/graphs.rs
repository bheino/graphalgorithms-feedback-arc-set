use crate::graph::hash_table::HashTable;
use crate::tools::metis::Metis;

pub fn graph_from_file(filename: &str) -> HashTable {
  let mut path = "test/resources/".to_owned();
  if filename.starts_with("h_") {
    path += "heuristic/";
  } else if filename.starts_with("e_") {
    path += "exact/";
  }
  path += filename;

  let mut parser = Metis::new(path.as_str());
  parser.parse();

  HashTable::from_vertices_and_edges(parser.vertices(), parser.edges())
}

pub fn graph_with_multiple_cliques() -> HashTable {
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
  HashTable::from_edges(&edges)
}

pub fn graph_with_simple_clique() -> HashTable {
  let edges = [(0, 1), (1, 2), (2, 0)];
  HashTable::from_edges(&edges)
}
