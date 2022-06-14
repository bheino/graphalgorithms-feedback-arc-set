use std::fs::File;
use std::io;
use std::io::BufRead;

// Input format described here: https://pacechallenge.org/2022/tracks/
#[allow(dead_code)]
pub struct Metis {
  filename: String,
  edges: Vec<(usize, usize)>,
}

impl Metis {
  #[allow(dead_code)]
  pub fn new(file: &str) -> Self {
    Self {
      filename: file.to_string(),
      edges: vec![],
    }
  }

  // Based on: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
  // Returns an Iterator to the Reader of the lines of the file.
  #[allow(dead_code)]
  fn lines(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(&self.filename)?;
    // TODO Error handling?
    Ok(io::BufReader::new(file).lines())
  }

  #[allow(dead_code)]
  pub fn parse(&mut self) {
    // TODO Error handling?
    if let Ok(mut lines) = self.lines() {
      lines.next(); // Ignore first line
                    // TODO Get Indices (= Node numbers from 1..n) right because idx is currently incremented on comment lines too!
      for (idx, line) in lines.enumerate() {
        // TODO Error handling?
        if let Ok(content) = line {
          self.parse_line(content, idx);
        }
      }
    }
  }

  fn parse_line(&mut self, line: String, source: usize) {
    if line.is_empty() || line.starts_with('%') {
      return;
    }

    for edge in line.split_ascii_whitespace() {
      let target = edge.parse::<usize>().unwrap();
      self.edges.push((source, target));
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::tools::metis::Metis;

  #[test]
  fn can_parse_e_001() {
    let mut e_001 = Metis::new("test/resources/e_001");
    e_001.parse();

    // TODO Implement first-line parsing of edge count, so the test can verify correctly
    assert_eq!(e_001.edges.len(), 651);

    println!("{:?}", e_001.edges);
  }
}
