extern crate term_painter;

use term_painter as painter;
use std::default::Default;

fn main() {
  let s = painter::Style::default();

  println!("Hello World {}", s.paint("huhu"));
}
