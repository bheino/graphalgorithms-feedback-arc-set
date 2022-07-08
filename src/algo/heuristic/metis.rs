use anyhow::{anyhow, Result};
use petgraph::{
  graph::{DefaultIx, IndexType, NodeIndex},
  stable_graph::StableDiGraph,
};

use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::Path,
};

#[derive(Debug, Default, Clone)]
pub struct Metis<N = u32, Ix: IndexType = DefaultIx> {
  vertex_count: N,
  edge_count: N,
  identifier: N,
  edges: Vec<(Ix, Ix)>,
}

impl<N: Default, Ix: IndexType + Into<NodeIndex>> Into<StableDiGraph<N, ()>> for Metis<Ix> {
  fn into(self) -> StableDiGraph<N, ()> {
    StableDiGraph::<N, ()>::from_edges(self.edges.as_slice())
  }
}

impl Metis {
  pub fn from_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    let mut metis = Self::default();
    BufReader::new(File::open(&path)?)
      .lines()
      .filter_map(|l| l.ok())
      .filter(|l| !(l.is_empty() || l.starts_with("%")))
      .skip(1)
      .enumerate()
      .for_each(|(idx, targets)| {
        targets.split_whitespace().for_each(|target| {
          metis.edges.push((
            IndexType::new(idx),
            IndexType::new(target.parse::<usize>().unwrap()),
          ))
        })
      });

    let first_line = BufReader::new(File::open(path)?)
      .lines()
      .filter_map(|l| l.ok())
      .filter(|l| !(l.is_empty() || l.starts_with("%")))
      .take(1)
      .collect::<String>();

    let first_line = first_line.split_whitespace().collect::<Vec<_>>();
    metis.vertex_count = first_line
      .get(0)
      .ok_or(anyhow!("Could not get vertex_count!"))?
      .parse()?;

    metis.edge_count = first_line
      .get(1)
      .ok_or(anyhow!("Could not get edge_count!"))?
      .parse()?;

    metis.identifier = first_line
      .get(2)
      .ok_or(anyhow!("Could not get identifier!"))?
      .parse()?;

    Ok(metis)
  }
}

#[cfg(test)]
mod test {
  use super::Metis;
  use anyhow::Result;
  use petgraph::stable_graph::StableDiGraph;

  #[test]
  fn from_file() -> Result<()> {
    let path = "test/resources/exact/e_001_with_comments";
    let metis = Metis::from_file(path)?;

    let graph: StableDiGraph<i32, ()> = metis.clone().into();

    assert_eq!(metis.vertex_count, 512);
    assert_eq!(metis.edge_count, 651);
    assert_eq!(metis.identifier, 0);
    assert_eq!(metis.edges.len(), 651);

    Ok(())
  }
}
