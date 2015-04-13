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

    fn fg(&self, c: Color) -> Style {
        let mut s = self.to_style();
        s.fg = c;
        s
    }

    fn bg(&self, c: Color) -> Style {
        let mut s = self.to_style();
        s.bg = c;
        s
    }

    fn bold(&self) -> Style {
        let mut s = self.to_style();
        s.bold = true;
        s
    }

    fn underline(&self) -> Style {
        let mut s = self.to_style();
        s.underline = true;
        s
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
        let mut s = Style::default();
        s.fg = *self;
        s
    }
}

// #[derive(Debug, Copy, Clone)]
// pub enum Attr {
//     Normal,
//     Bold,
//     Underline,
//     Blink,
//     Standout,
//     Reverse,
//     Secure,
// }

// impl Attr {
//     pub fn term_constant(&self) -> Option<term::Attr> {
//         match *self {
//             Attr::Normal      => None,
//             Attr::Bold        => Some(term::Attr::Bold),
//             Attr::Underline   => Some(term::Attr::Underline(true)),
//             Attr::Blink       => Some(term::Attr::Blink),
//             Attr::Standout    => Some(term::Attr::Standout(true)),
//             Attr::Reverse     => Some(term::Attr::Reverse),
//             Attr::Secure      => Some(term::Attr::Secure),
//         }
//     }
// }

// impl Default for Attr {
//     fn default() -> Self {
//         Attr::Normal
//     }
// }

#[derive(Debug, Copy, Clone)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub underline: bool,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Color::default(),
            bg: Color::default(),
            bold: false,
            underline: false,
        }
    }
}

impl Style {
    pub fn prepare(&self) -> Result<(), Error> {
        macro_rules! try_term {
            ($e:expr) => ({
                match $e {
                    Ok(true) => {},
                    _ => { return Err(Error); },
                }
            })
        }

        let mut t = term::stdout().unwrap();

        match self.fg.term_constant() {
            None => {},
            Some(c) => { try_term!(t.fg(c)); },
        }
        match self.bg.term_constant() {
            None => {},
            Some(c) => { try_term!(t.bg(c)); },
        }
        if self.bold { try_term!(t.attr(term::Attr::Bold)) }
        if self.underline { try_term!(t.attr(term::Attr::Underline(true))) }

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
