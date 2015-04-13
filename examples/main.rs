extern crate term_painter;

use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

#[derive(Debug)]
struct Foo {
    bar: String
}

fn main() {
  let x = Foo { bar: "huhu".to_string() };

  println!("{:?}", Plain.paint(&x));
  println!("{:?}", Plain.paint(x));

  // println!("{}\n{}\n{}\n{}\n{}",
  //   Red.bg(Green).bold().paint("Red-Green-Bold"),
  //   Blue.paint("Blue"),
  //   Blue.bold().paint("Blue"),
  //   Blue.bg(Magenta).paint("Blue"),
  //   Normal.underline().paint("Underline"));
}
