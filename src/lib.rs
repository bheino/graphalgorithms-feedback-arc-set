#![allow(dead_code)]

pub mod bisection;
pub mod fas;
pub mod ordering;
pub mod tools;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
