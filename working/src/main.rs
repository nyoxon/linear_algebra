#![allow(dead_code, unused_imports, unused_variables, unused_mut, 
non_snake_case)]

extern crate vectors;
extern crate matrices;
extern crate complex;

use vectors::*;
use matrices::*;
use complex::*;
use rand::Rng;

fn random_matrix(m: usize, n: usize) -> Matrix {
  let mut rng = rand::rng();
  let mut matrix = Matrix::zero(m, n);

  for i in 1..=m {
    for j in 1..=n {
      let random_number = rng.random_range(-10.0..10.0);
      matrix.change_element(i, j, random_number);
    }
  }

  matrix
}

fn main() {
  let k = Complex::new(5.0, -10.0);
  let t = Complex::new(2.0, 1.0);
  println!("{}", &k + &t);
  println!("{}", &k - &t);
  println!("{}", &k * &t);
  println!("{}", &k / &t);
  println!("{}", t.pow_complex(&k));
}   
