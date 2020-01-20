use gif::{Encoder, Frame, Repeat, SetParameter};
use ndarray::*;
use rand::prelude::random;
use std::borrow::Cow;
use std::fmt;
use std::fs;
use std::fs::DirBuilder;
use std::iter::FromIterator;
use std::path::*;

pub type Space = ArrayD<u8>; // Generic over multiple Dims

// Holds the Automata and Space
#[derive(Clone)]
pub struct Simulation {
  pub max_iterations: u32,
  pub iteration: u32,
  pub automata: Automata,
  pub space: Space,
}

#[derive(Clone)]
pub struct Automata {
  pub dimension: u32,
  pub neighbors: u32,
  pub rules: Rules,
}
// All Rules are 1D
pub type Rules = Array1<u8>;
pub trait TRules {
  fn to_str(&self, bin: bool) -> String;
}
impl TRules for Rules {
  fn to_str(&self, bin: bool) -> String {
    let mut v = Vec::<u8>::new();
    let mut acc = 0u8;
    let mut s = String::from("");
    for (idx, e) in self.indexed_iter() {
      let bit_idx = idx % 8;
      acc += e << bit_idx;
      if idx > 0 && bit_idx == 7 {
        v.push(acc);
        // println!("!!!!acc: {} {} {} {} {:?}", e, idx, bit_idx, acc, v);
        acc = 0;
      } else {
        //println!("acc: {} {} {} {} {:?}", e, idx, bit_idx, acc, v);
      }
    }
    for e in &v {
      if bin {
        s.push_str(&format!("{:08b}", e));
      } else {
        s.push_str(&format!("{:08}", e));
      }
    }
    format!("{}", s)
  }
}

// Struct used to contain the properties of interest for a Rule.  Generally,
// these have to do with the number of "alive" rules.
#[derive(Debug)]
pub struct InterestingRules {
  alive_ratio: f32,
}
impl fmt::Display for InterestingRules {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.alive_ratio)
  }
}
// Trait for producing Interesting structs about Rules.
trait IsInterestingRules {
  fn get_interesting(&self) -> InterestingRules;
}

impl IsInterestingRules for Rules {
  fn get_interesting(&self) -> InterestingRules {
    let alive = self.iter().filter(|e| **e > 0).count() as f32;
    let total = self.len() as f32;
    /* println!("alive, total, alive_ratio: {:?}, {}, {}, {}",
    self, alive, total, alive/total);*/
    InterestingRules {
      alive_ratio: alive / total,
    }
  }
}

// Trait for producing Interesting structs about Automata.
pub trait IsInterestingSimulation {
  fn get_interesting(&self) -> InterestingSimulation;
}

#[derive(Debug)]
pub struct InterestingSimulation {
  alive_ratio: f32,
  neighbor_ratio: f32,
  lateral_neighbor_ratio: f32,
  overall: f32,
}

impl IsInterestingSimulation for Simulation {
  fn get_interesting(&self) -> InterestingSimulation {
    InterestingSimulation {
      alive_ratio: self.space.iter().filter(|e| **e > 0).count() as f32 / self.space.len() as f32,
      lateral_neighbor_ratio: 0.0,
      neighbor_ratio: 0.0,
      overall: 0.0,
    }
  }
}

impl Simulation {
  pub fn is_complete(&self) -> bool {
    self.iteration + 1 == self.max_iterations
  }
  pub fn path_exists(&self) -> bool {
    let exists = self.path().as_path().exists();
    if exists {
      println!("!! Path exists: {:?}.  Skipping...\n", self.path());
    }
    exists
  }
  pub fn path(&self) -> PathBuf {
    return Path::new(&format!(
      "./output/{}/{}/{}:IR:{}.gif",
      self.automata.dimension,
      self.automata.neighbors,
      self.automata.rules.to_str(true),
      self.automata.rules.get_interesting()
    ))
    .to_path_buf();
  }
  fn iterate_1d(&mut self) {
    let dim0 = self.space.dim()[0] as u32;
    for i in 0..dim0 {
      // memos
      let nei = self.automata.neighbors as u32;
      let idim0 = i + dim0;
      // Start accumulation with 2**neighbors
      let mut acc: usize = self.space[[self.iteration as usize, i as usize]] as usize
                            * 2usize.pow(nei);
      let _ac = self.space[[self.iteration as usize, i as usize]] as usize;
      for idx in 0..nei {
        let l = (idim0 - 1) % dim0;
        let r = (idim0 + 1) % dim0;
        // neighbors==1 && idx==0 -> 1; neighbors==2 && idx==0 -> 2; neighbors==2 && idx==1 -> 1
        acc += self.space[[self.iteration as usize, l as usize]] as usize
                * 2usize.pow(nei + idx as u32 - 1u32);
        // neighbors==1 && idx==0 -> 4; neighbors==2 && idx==0 -> 4; neighbors==2 && idx==1 -> 8
        acc += self.space[[self.iteration as usize, r as usize]] as usize
                * 2usize.pow(idx as u32 + 2u32);
         /*if self.iteration < 2 {
           print!("({},{},{}) {} {} {} -> {} ", l, i, r,
           self.space[[self.iteration as usize, l as usize]],
           _ac,
           self.space[[self.iteration as usize, r as usize]],
           acc);
        }*/
      }
      let v = self.automata.rules[[acc]];
      /*if self.iteration < 2 {
        print!("-> {}; ", v);
      }*/
      // set the value in the *next* plane to the decoded value
      self.space[[1 + self.iteration as usize, i as usize]] = v;
    }
    /*if self.iteration < 2 {
      println!("");
    }*/
    self.iteration += 1;
  }
  pub fn iterate_all(&mut self) {
    while !self.is_complete() {
      self.iterate_1d();
    }
  }
  pub fn initialize_space(&mut self) {
    let mut s = self.space.view_mut();
    // Place down a consistent "predicate" pattern.
    // Unsafe-ish (panic) since no range checking is done.
    // 1110010-1-0110011
    let middle = s.dim()[0] / 2;
    // s[[0, middle - 7]] = 1;
    // s[[0, middle - 6]] = 1;
    // s[[0, middle - 5]] = 1;
    // s[[0, middle - 4]] = 0;
    // s[[0, middle - 3]] = 0;
    // s[[0, middle - 2]] = 1;
    // s[[0, middle - 1]] = 0;
    s[[0, middle]] = 1;
    // s[[0, middle + 1]] = 0;
    // s[[0, middle + 2]] = 1;
    // s[[0, middle + 3]] = 1;
    // s[[0, middle + 4]] = 0;
    // s[[0, middle + 5]] = 0;
    // s[[0, middle + 6]] = 1;
    // s[[0, middle + 7]] = 1;
  }
  pub fn randomize_space(&mut self) {
    let mut s = self.space.view_mut();
    // 2D automata?
    if s.axes().count() == 3usize {
      // 2D
      for x in 0..s.dim()[1] {
        for y in 0..s.dim()[1] {
          s[[0, x, y]] = random::<bool>() as u8;
        }
      }
    } else {
      // 1D
      for i in 0..s.dim()[0] {
        s[[0, i]] = random::<bool>() as u8;
      }
    }
  }

  pub fn write_image(&self) {
    let __p = self.path();
    let _p = __p.parent().unwrap();
    let p = Path::new("./").join(&_p);
    let f = Path::new("./").join(&__p);
    let sp = self.space.clone();
    DirBuilder::new()
      .recursive(true)
      .create(&p)
      .expect("Couldn't create the directory.");
    let mut image = fs::File::create(f.clone()).unwrap();
    let color_map = &[0, 0, 0, 0xFF, 0xFF, 0xFF];
    let mut encoder = Encoder::new(
      &mut image,
      sp.dim()[0] as u16,
      sp.dim()[1] as u16,
      color_map,
    ).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    // Handle 1D and 2D automata.  Default to 1D and
    // end after 1 frame.
    let mut end = 1usize;
    let mut fw = sp.dim()[0] as u16;
    let mut fh = sp.dim()[1] as u16;
    if sp.axes().count() == 3usize {
      // We're dealing with a 2D automata over a
      // 3D space so end at the last frame.
      end = sp.dim()[0];
      fw = sp.dim()[1] as u16;
      fh = sp.dim()[2] as u16;
    }
    for state in 0..end {
      println!("writing: {:?}\n", f);
      let mut frame = Frame::default();
      let mut sl = sp.view();
      frame.width = fw;
      frame.height = fh;
      if end == 1 {
        sl.slice_collapse(s![0.., 0..].as_ref());
      } else {
        sl.slice_collapse(s![state, 0.., 0..].as_ref());
      }
      let iter = sl.iter();
      let arr = Array::from_iter(iter).map(|e| **e);
      frame.buffer = Cow::Borrowed(arr.as_slice().unwrap());
      encoder.write_frame(&frame).unwrap();
    }
  }
}
