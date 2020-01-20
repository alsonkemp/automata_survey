use automata_survey;

fn main() {
    // For a 1D automata with 1 neighbor, 256 images should be produced.
    // So 1024 runs will probably produce those. 
    // For a 1D automata with 1 neighbor, 2^512 images should be produced.
    // Nothing in the universe will ever produce those... 
    for n in 0..1024 {
      println!("Run #: {}", n);
      // automata_survey::automata::automata_1d::run_random_automata(1);
      automata_survey::automata::automata_2d::run_random_automata(1);
    }
}
