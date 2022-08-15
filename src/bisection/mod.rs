use crate::graph::hash_table::{Direction, HashTable, VertexId};
use rand::Rng;
use std::borrow::Borrow;
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

const ALPHA: f32 = 0.6;

pub struct StochasticEvolution<'a> {
  graph: &'a HashTable,
  current_bisection: (Vec<usize>, Vec<usize>),
  best_bisection: (Vec<usize>, Vec<usize>),
  vertices: Vec<VertexId>,
}

impl<'a> StochasticEvolution<'a> {
  pub fn new(graph: &'a HashTable) -> Self {
    Self {
      graph,
      current_bisection: initial_bisection(graph.vertices().len()),
      best_bisection: initial_bisection(graph.vertices().len()),
      vertices: graph.vertices(),
    }
  }

  pub fn bisection(&mut self) -> (HashSet<VertexId>, HashSet<VertexId>) {
    //Input Parameters:
    let initial_p = -1;
    let initial_r = 10;
    let initial_delta = 2;
    debug_assert!(initial_p <= 0);
    debug_assert!(initial_r > 1);
    debug_assert!(initial_delta > 0);

    let mut p = initial_p;
    let mut counter = 0;

    loop {
      let c_pre = self.cost();
      self.perturb(p);
      let c_post = self.cost();
      if c_post < c_pre {
        self.best_bisection = self.current_bisection.clone();
        counter -= initial_r;
      } else {
        counter += 1;
      }
      if c_post == c_pre {
        p -= initial_delta;
      } else {
        p = initial_p;
      }
      if counter > initial_r {
        break;
      }
    }

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

  fn perturb(&mut self, p: i32) {
    let mut s1 = vec![];
    let mut s2 = vec![];

    for i in 0..self.vertices.len() {
      if self.gain(i) > rand::thread_rng().gen_range(p..0) {
        self.move_vertex(i);
        if self.current_bisection.0.contains(&i) {
          s1.push(i);
        } else {
          s2.push(i);
        }
      }
    }

    let bisection;
    let mut stack;
    if self.current_bisection.0.len() > self.current_bisection.1.len() {
      bisection = self.current_bisection.0.clone();
      stack = s1;
    } else {
      bisection = self.current_bisection.1.clone();
      stack = s2;
    }

    while bisection.len() as f32 > (ALPHA * self.vertices.len() as f32) {
      let i = stack.pop().unwrap();
      self.move_vertex(i);
    }
  }

  // Number of Edges from Partition 2 to Partition 1
  fn cost(&mut self) -> usize {
    let mut cost = 0;
    for v_2_index in self.current_bisection.1.as_slice() {
      let v_2 = self.vertices[*v_2_index];
      for (_, neighbour) in self.graph.edges(v_2, Direction::Outbound) {
        // check if neighbor is in other partition
        if self.vertices.contains(&neighbour) {
          cost += 1;
        }
      }
    }

    cost
  }

  fn gain(&mut self, p0: usize) -> i32 {
    todo!()
  }

  fn move_vertex(&mut self, i: usize) {
    todo!()
  }
}

fn initial_bisection(vertices_count: usize) -> (Vec<usize>, Vec<usize>) {
  (
    (0..(vertices_count / 2)).collect(),
    ((vertices_count / 2)..vertices_count).collect(),
  )
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
