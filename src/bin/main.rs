use automata_survey;

fn main() {
    // For a 1D automata with 1 neighbor, 256 images should be produced.
    // So 1024 runs will probably produce those. 
    for n in 0..24 {
      println!("\nRun #: {}", n);
      automata_survey::automata::automata_1d::run_random_automata(1);
    }
}
