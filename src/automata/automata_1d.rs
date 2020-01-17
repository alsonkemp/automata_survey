use crate::automata::types::*;
use ndarray::*;
use rand::prelude::random;

fn new_with_random_rules<'a>(
  neighbors: u32,
  space_size: u32,
  max_iterations: u32,
) -> Simulation {
  let randoms = (0..(2u32.pow(1 + 2 * neighbors)))
    .map(|_| random::<u8>() & 1)
    .collect::<Array1<u8>>();
  return Simulation {
    iteration: 0,
    max_iterations: max_iterations,
    automata: Automata {
      dimension: 1,
      neighbors,
      rules: Array1::<u8>::from(randoms),
    },
    space: ArrayD::<u8>::zeros(IxDyn(&[max_iterations as usize, space_size as usize])).clone(),
  };
}

pub fn run_random_automata(n: u32) {
  let mut s = new_with_random_rules(n, 100, 100);
  if s.path_exists() {
    println!("!! Path exists: {:?}.  Skipping...", s.path());
  } else {
    // s.randomize_space();
    s.initialize_space();
    println!("Rules = {}", s.automata.rules.to_str(true));
    s.iterate_all();
    s.write_image();
  }
}
