use crate::graph::hash_table::{HashTable, VertexId};
use std::collections::HashSet;

mod stochastic_evolution;

/*
SE is an process that iteratively improves an initial solution through a sequence of local
perturbations called moves. [...] The overall SE algorithm
for GB can now be described as follows (Saab, 2001):

(V1 , V2 ) := a random initial bisection of G(V, E);
(B1 , B2 ) := (V1 , V2 ); {save best bisection}
p := p0 ; {initial value for parameter p}
set value for iteration control parameter R;
counter := 0;
repeat
    Cpre := cost(V1 , V2 );
    perturb(V, V1 , V2 , p);
    Cpost := cost(V1 , V2 );
    if Cpost < Cpre then
        (B1 , B2 ):= (V1 , V2 ); {save best bisection}
        counter := counter - R {allow for more iterations}
    else
        counter := counter + 1;
    if Cpost = Cpre then
        p := p − δ {decrease p to allow for more movements of vertices in perturb}
    else
        p := p0 {restore original value of p}
until counter > R
return (B1 , B2 ).
 */
pub struct StochasticEvolution<'a> {
  graph: &'a HashTable,
}

impl<'a> StochasticEvolution<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self { graph }
  }

  pub fn bisection(&self) -> (HashSet<VertexId>, HashSet<VertexId>) {
    self.dummy_result()
  }

  fn dummy_result(&self) -> (HashSet<VertexId>, HashSet<VertexId>) {
    let vertices = self.graph.vertices();
    (
      HashSet::from_iter(vertices[0..(vertices.len() / 2)].iter().cloned()),
      HashSet::from_iter(
        vertices[(vertices.len() / 2)..vertices.len()]
          .iter()
          .cloned(),
      ),
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::bisection::StochasticEvolution;
  use crate::tools::graphs::graph_from_wikipedia_scc;
  use std::collections::HashSet;

  #[test]
  fn works_on_wikipedia_scc() {
    let graph = graph_from_wikipedia_scc();
    let (partition_1, partition_2) = StochasticEvolution::new(&graph).bisection();

    let graph_vertices = HashSet::from_iter(graph.vertices().into_iter());
    assert_eq!(graph_vertices.len(), partition_1.len() + partition_2.len());
    assert!(partition_1.is_subset(&graph_vertices));
    assert!(partition_2.is_subset(&graph_vertices));
    assert!(partition_1.is_disjoint(&partition_2));
  }
}
