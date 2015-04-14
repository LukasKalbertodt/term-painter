extern crate term_painter;

use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

#[derive(Debug)]
struct Foo {
    bar: String
}

fn main() {
    for _ in 0..10 {
        all_styles();
    }

  // let x = Foo { bar: "huhu".to_string() };

  // println!("{:?}", Plain.paint(&x));
  // println!("{:?}", Plain.paint(x));

  // println!("{}\n{}\n{}\n{}\n{}",
  //   Red.bg(Green).bold().paint("Red-Green-Bold"),
  //   Blue.paint("Blue"),
  //   Blue.bold().paint("Blue"),
  //   Blue.bg(Magenta).paint("Blue"),
  //   Normal.underline().paint("Underline"));
}

fn all_styles() {
    let colors =
        [Normal, Black, Red, Green, Yellow, Blue, Magenta, Cyan, White];

    // Normal test
    for c in &colors { print!("{:?} ", c.paint(c)); }
    println!("    (fg)");
    for c in &colors { print!("{:?} ", Plain.bg(*c).paint(c)); }
    println!("    (bg)");

    // Bold text
    for c in &colors { print!("{:?} ", c.bold().paint(c)); }
    println!("    (bold fg)");
    for c in &colors { print!("{:?} ", Bold.bg(*c).paint(c)); }
    println!("    (bold bg)");

    // Underlined text
    for c in &colors { print!("{:?} ", c.underline().paint(c)); }
    println!("    (underline fg)");
    for c in &colors { print!("{:?} ", Underline.bg(*c).paint(c)); }
    println!("    (underline bg)");

    // Underlined and bold text
    for c in &colors { print!("{:?} ", c.underline().bold().paint(c)); }
    println!("    (underline bold fg)");
    for c in &colors { print!("{:?} ", Underline.bg(*c).bold().paint(c)); }
    println!("    (underline bold bg)");
}
