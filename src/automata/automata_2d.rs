/* 1D first...
use crate::automata::types::*;
use ndarray::prelude::*;
use rand::prelude::random;

fn new(neighbors: u8, rules: &'static Rules, space_size: usize, max_iterations: usize) -> &Simulation {
  &Simulation {
    iteration: 0,
    max_iterations: max_iterations,
    automata: Automata {
      dimension: 2,
      neighbors,
      rules: &rules,
    },
    space: ArrayD::<u8>::zeros(IxDyn(&[max_iterations, space_size, space_size]))
  }
}

fn new_with_random_rules(neighbors: u8, space_size: usize, max_iterations: usize) -> &'static Simulation {
  let randoms: Vec<u8> = (0..(2u32.pow(1 + 2 * neighbors as u32)))
    .map(|_| random::<u8>())
    .collect();
  let rules = Array1::<u8>::from(randoms);
  &Simulation {
    iteration: 0,
    max_iterations: max_iterations,
    automata: Automata {
      dimension: 1,
      neighbors,
      rules: &rules,
    },
    space: ArrayD::<u8>::zeros(IxDyn(&[max_iterations, space_size, space_size]))
  }
}

fn run_random_automata() {
  let mut s : &Simulation = new_with_random_rules(1, 100, 100);
  s.randomize_space();
  println!("Rules = {}", s.automata.rules.to_str());
  s.write_image();
}
*/
