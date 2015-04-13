extern crate term_painter;

use term_painter::{Color, ToStyle};
use std::default::Default;

fn main() {
  println!("Hello World {}", Color::Red.paint("huhu"));
}
