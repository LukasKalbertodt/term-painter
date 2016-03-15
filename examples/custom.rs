extern crate term_painter;

use term_painter::ToStyle;
use term_painter::Color::Custom;
use term_painter::Attr::Plain;

fn main() {
    // print 16 colors each line
    for line in 0..16 {
        // foreground
        print!("FG:  ");
        for c in (0..16).map(|i| 16*line + i) {
            print!("{: <2x} ", Custom(c).paint(c));
        }
        println!("");

        // background
        print!("BG:  ");
        for c in (0..16).map(|i| 16*line + i) {
            print!("{: <2x} ", Plain.bg(Custom(c)).paint(c));
        }
        println!("");
    }
}
