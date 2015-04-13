extern crate term;

use std::default::Default;
use std::fmt::{Display, Error, Formatter};

pub trait ToStyle {
    fn to_style(&self) -> Style;

    fn prepare(&self) -> Result<(), Error> {
        self.to_style().prepare()
    }

    fn cleanup(&self) -> Result<(), Error> {
        self.to_style().cleanup()
    }

    fn paint<T: Display>(&self, obj: T) -> Painted<T> {
        Painted { style: self.to_style() , obj: obj }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Normal,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn term_constant(&self) -> Option<term::color::Color> {
        match *self {
            Color::Normal  => None,
            Color::Black   => Some(term::color::BLACK),
            Color::Red     => Some(term::color::RED),
            Color::Green   => Some(term::color::GREEN),
            Color::Yellow  => Some(term::color::YELLOW),
            Color::Blue    => Some(term::color::BLUE),
            Color::Magenta => Some(term::color::MAGENTA),
            Color::Cyan    => Some(term::color::CYAN),
            Color::White   => Some(term::color::WHITE),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Normal
    }
}

impl ToStyle for Color {
    fn to_style(&self) -> Style {
        Style { fg: *self, bg: Color::default(), deco: Decoration::default() }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Decoration {
    Normal,
    Bold,
    Underline
}

impl Default for Decoration {
    fn default() -> Self {
        Decoration::Normal
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Style {
    fg: Color,
    bg: Color,
    deco: Decoration,
}

impl Style {
    pub fn prepare(&self) -> Result<(), Error> {
        let mut t = term::stdout().unwrap();

        match self.fg.term_constant() {
            None => {},
            Some(c) => { t.fg(c).unwrap(); },
        }
        match self.bg.term_constant() {
            None => {},
            Some(c) => { t.bg(c).unwrap(); },
        }

        Ok(())
    }

    pub fn cleanup(&self) -> Result<(), Error> {
        let mut t = term::stdout().unwrap();

        t.reset().unwrap();

        Ok(())
    }
}

impl ToStyle for Style {
    fn to_style(&self) -> Style {
        self.clone()
    }
}

pub struct Painted<T: Display> {
    style: Style,
    obj: T,
}

impl<T: Display> Display for Painted<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(self.style.prepare());
        try!(write!(f, "{}", self.obj));
        self.style.cleanup()
    }
}


#[test]
fn it_works() {
}
