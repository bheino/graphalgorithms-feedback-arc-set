#![allow(dead_code)]

pub mod algo;
pub mod feedback_arc_set;
pub mod tools;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
