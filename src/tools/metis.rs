use std::fs::File;
use std::io;
use std::io::BufRead;

// Input format described here: https://pacechallenge.org/2022/tracks/
#[allow(dead_code)]
pub struct Metis {
  filename: String,
  edges: Vec<(usize, usize)>,
  edge_count: usize,
}

impl Metis {
  #[allow(dead_code)]
  pub fn new(file: &str) -> Self {
    Self {
      filename: file.to_string(),
      edges: vec![],
      edge_count: 0,
    }
  }

  // Based on: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
  // Returns an Iterator to the Reader of the lines of the file.
  #[allow(dead_code)]
  fn lines(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(&self.filename)?;
    Ok(io::BufReader::new(file).lines())
  }

  #[allow(dead_code)]
  pub fn parse(&mut self) {
    if let Ok(lines) = self.lines() {
      let mut idx = 0;
      for line in lines.flatten() {
        if idx != 0 {
          idx = self.parse_content_line(line, idx);
        } else {
          self.parse_header_line(line);
          idx += 1;
        }
      }
    }
  }

  fn parse_content_line(&mut self, line: String, idx: usize) -> usize {
    if line.starts_with('%') {
      return idx;
    }

    for edge in line.split_whitespace() {
      let target = edge.parse::<usize>().unwrap();
      self.edges.push((idx, target));
    }

    idx + 1
  }

  fn parse_header_line(&mut self, header: String) {
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() >= 2 {
      self.edge_count = parts[1].parse::<usize>().unwrap();
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::tools::metis::Metis;

  #[test]
  fn can_parse_e_001() {
    can_parse_metis_file("test/resources/e_001", 651);
  }

  #[test]
  fn can_parse_e_001_with_comments() {
    can_parse_metis_file("test/resources/e_001_with_comments", 651);
  }

  fn can_parse_metis_file(path: &str, expected_edge_count: usize) {
    let mut e_001 = Metis::new(path);
    e_001.parse();

    assert_eq!(e_001.edge_count, expected_edge_count);
    assert_eq!(e_001.edges.len(), e_001.edge_count);

    println!("{:?}", e_001.edges);
  }
}
