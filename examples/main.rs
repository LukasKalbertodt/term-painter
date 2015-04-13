extern crate term_painter;

use term_painter::ToStyle;
use term_painter::Color::*;
use std::default::Default;

fn main() {
  println!("{}\n{}\n{}\n{}\n{}",
    Red.bg(Green).bold().paint("Red-Green-Bold"),
    Blue.paint("Blue"),
    Blue.bold().paint("Blue"),
    Blue.bg(Magenta).paint("Blue"),
    Normal.underline().paint("Underline"));
}
