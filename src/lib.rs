extern crate term;

use std::default::Default;
use std::fmt::{Display, Debug, Error, Formatter};

/// Everything that can be seen as part of a style. This is the core of this
/// crate. All functions ("style modifier") consume self and return a modified
/// version of the style.
pub trait ToStyle : Clone {
    fn to_style(self) -> Style;

    /// Sets the foreground (text) color.
    fn fg(self, c: Color) -> Style {
        let mut s = self.to_style();
        s.fg = c;
        s
    }

    /// Sets the background color.
    fn bg(self, c: Color) -> Style {
        let mut s = self.to_style();
        s.bg = c;
        s
    }

    /// Makes the text bold.
    fn bold(self) -> Style {
        let mut s = self.to_style();
        s.bold = true;
        s
    }

    /// Underlines the text.
    fn underline(self) -> Style {
        let mut s = self.to_style();
        s.underline = true;
        s
    }

    /// Wraps the style specified in `self` and something of arbitrary type
    /// into a `Painted`. When `Painted` is printed it will print the arbitrary
    /// something with the given style. `T` needs to implement
    /// `std::fmt::Display` or `std::fmt::Debug`.
    fn paint<'a, T: 'a + ?Sized>(&self, obj: &'a T) -> Painted<'a, T> {
        Painted { style: self.clone().to_style() , obj: obj }
    }
}

/// Lists all possible Colors. It implements `ToStyle` so it's possible to call
/// `ToStyle`'s methods direclty on a `Color` variant like:
///
/// `println!("{}", Color::Red.bold().paint("Red and bold"));`
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
    /// Returns the associated constant from `term::color::Color`.
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
    /// Returns a Style with default values and the `self` color as foreground
    /// color.
    fn to_style(self) -> Style {
        let mut s = Style::default();
        s.fg = self;
        s
    }
}

/// Lists possible attributes. It implements `ToStyle` so it's possible to call
/// `ToStyle`'s methods directly on a `Attr` variant like:
///
/// `println!("{}", Attr::Bold.fg(Color::Red).paint("Red and bold"));`
#[derive(Debug, Copy, Clone)]
pub enum Attr {
    Bold,
    Underline,
}

impl ToStyle for Attr {
    /// Returns a Style with default values and the `self` attribute enabled.
    fn to_style(self) -> Style {
        let mut s = Style::default();
        match self {
            Attr::Bold => s.bold = true,
            Attr::Underline => s.underline = true,
        }
        s
    }
}

/// Saves all properties of a style. Implements `ToStyle`, so you can call
/// style modifiers on it.
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
    fn prepare(&self) -> Result<(), Error> {
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

    fn cleanup(&self) -> Result<(), Error> {
        let mut t = term::stdout().unwrap();

        t.reset().unwrap();

        Ok(())
    }
}

impl ToStyle for Style {
    /// Dummy implementation that just returns `self`.
    fn to_style(self) -> Style {
        self
    }
}

/// Saves a style and a reference to something that will be printed in that
/// style. That something of type `T` needs to implement either
/// `std::fmt::Debug` or `std::fmt::Display`
pub struct Painted<'a, T: 'a + ?Sized> {
    style: Style,
    obj: &'a T,
}

impl<'a, T: Display + 'a + ?Sized> Display for Painted<'a, T> {
    /// Implementation for `T: Display` -> to print with `{}`.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(self.style.prepare());
        try!(write!(f, "{}", self.obj));
        self.style.cleanup()
    }
}

impl<'a, T: Debug + 'a + ?Sized> Debug for Painted<'a, T> {
    /// Implementation for `T: Debug` -> to print with `{:?}`.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(self.style.prepare());
        try!(write!(f, "{:?}", self.obj));
        self.style.cleanup()
    }
}
