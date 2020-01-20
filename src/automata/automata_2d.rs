use crate::automata::types::*;
use ndarray::*;
use rand::prelude::random;

fn conways_rules() -> Array1<u8> {
  // len 2^9 for neighbors == 1
  let mut rules = Array1::<u8>::zeros(Ix1(512));
  for (i, e) in rules.indexed_iter_mut() {
    let alive = i & 16 > 1;
    let cnt = i.count_ones();
    // Remember: cnt includes the *alive* cell so alive || (2+1) or (3+1)
    if (!alive && cnt == 3) || (alive && (cnt == 3 || cnt == 4)) {
      *e = 1;
    }
  };
  rules
}
fn new_with_rules<'a>(neighbors: u32, space_size: u32,
                             max_iterations: u32, rules : Array1<u8>) -> Simulation {
  return Simulation {
    iteration: 0,
    max_iterations: max_iterations,
    automata: Automata {
      dimension: 2,
      neighbors,
      rules: rules
    },
    space: ArrayD::<u8>::zeros(IxDyn(&[max_iterations as usize,
                                       space_size as usize,
                                       space_size as usize])).clone()
  };
}

fn new_with_random_rules<'a>(neighbors: u32, space_size: u32,
                             max_iterations: u32) -> Simulation {
  let randoms = (0..512)
    .map(|_| if random::<u8>() < 64 { 1 } else { 0 })
    .collect::<Array1<u8>>();
  new_with_rules(neighbors, space_size, max_iterations, randoms)
}

pub fn run_random_automata(n: u32) {
  /*let _rules = conways_rules();
  let mut _s = new_with_rules(n, 100, 100, _rules);
  println!("Rules (Conway) = {}", _s.automata.rules.to_str(false));
  println!("Interesting (Conway): {:?}", _s.automata.rules.get_interesting());
  _s.randomize_space();
  _s.iterate_all();
  _s.write_image();
  */
  let mut s = new_with_random_rules(n, 100, 100);
  if !s.path_exists() {
    s.randomize_space();
    println!("Rules (Random) = {}", s.automata.rules.to_str(false));
    println!("Interesting (Random): {:?}", s.automata.rules.get_interesting());
    s.iterate_all();
    s.write_image();
  }
}