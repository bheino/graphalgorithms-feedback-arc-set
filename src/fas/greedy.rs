/*
Procedure GR
(G: DiGraph; var s: VertexSequence);
s1 <- ∅; s2 <- ∅;
while G != ∅ do
    {while G contains a sink do
        {choose a sink u; s2 <- us2; G <- G - u);
    while G contains a source do
        {choose a source u; s1 <- s1u; G <- G - u};
    if G != ∅ then
    {choose a vertex u for which 6(u) is a
    maximum; s1 <- s1u; G <- G - u}};
s <- s1s2.
*/

use std::collections::HashSet;

use crate::graph::hash_table::HashTable;

use super::feedback_arc_set::FeedbackArcSet;

pub struct GreedyHeuristic<'a> {
  graph: &'a HashTable,
}

impl<'a> GreedyHeuristic<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }
}

impl FeedbackArcSet for GreedyHeuristic<'_> {
  fn feedback_arc_set(&self) -> std::collections::HashSet<crate::graph::hash_table::Edge> {
    let graph = self.graph.clone();

    // let s1 = vec![];
    // let s2 = vec![];

    while graph.vertices().len() > 0 {}

    HashSet::new()
  }
}
