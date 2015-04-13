//! This is a crate for coloring and formatting terminal output. Simple
//! example:
//!
//! ```
//! extern crate term_painter;
//!
//! use term_painter::ToStyle;
//! use term_painter::Color::*;
//! use term_painter::Attr::*;
//!
//! fn main() {
//!     println!("{} or {} or {}",
//!         Red.paint("Red"),
//!         Bold.paint("Bold"),
//!         Red.bold().paint("Both!"));
//! }
//! ```
//!
//! How to use it
//! -------------
//! Formatting works in two steps mainly:
//! 1. Creating a style
//! 2. Use this style to "paint" something and reviece a `Painted` object
//!
//! 1. Creating a style
//! -------------------
//! To create a style a startpoint is needed: This can either be a startpoint
//! with an attached modifier (like `Red`: modifies the fg-color) or the
//! `Plain` startpoint, which does not modify anything.
//! After that the startpoint can be modified by modifiers like `bold()` or
//! `fg()`.
//!
//! ```
//! extern crate term_painter;
//!
//! use term_painter::ToStyle;
//! use term_painter::Color::*;
//! use term_painter::Attr::*;
//!
//! fn main() {
//!     let x = 5;
//!
//!     // These two are equivalent
//!     println!("{}", x);
//!     println!("{}", Plain.paint(&x));
//!
//!     // These two are equivalent, too
//!     println!("{}", Red.paint(&x));
//!     println!("{}", Plain.fg(Red).paint(&x));
//! }
//! ```
//!
//! So it looks something like this:
//!
//! `$start_point`  [`.modifier1(...)`]  [`.modifier2(...)`]  `.paint(...)`
//!
//!
//!
//!
//!
//!

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
    fn paint<T>(&self, obj: T) -> Painted<T> {
        Painted { style: self.clone().to_style() , obj: obj }
    }
}

/// Lists all possible Colors. It implements `ToStyle` so it's possible to call
/// `ToStyle`'s methods direclty on a `Color` variant like:
///
/// `println!("{}", Color::Red.bold().paint("Red and bold"));`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    fn term_constant(&self) -> Option<term::color::Color> {
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Attr {
    Plain,
    Bold,
    Underline,
}

impl ToStyle for Attr {
    /// Returns a Style with default values and the `self` attribute enabled.
    fn to_style(self) -> Style {
        let mut s = Style::default();
        match self {
            Attr::Plain => {},
            Attr::Bold => s.bold = true,
            Attr::Underline => s.underline = true,
        }
        s
    }
}

/// Saves all properties of a style. Implements `ToStyle`, so you can call
/// style modifiers on it.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
pub struct Painted<T> {
    style: Style,
    obj: T,
}

impl<T: Display> Display for Painted<T> {
    /// Implementation for `T: Display` -> to print with `{}`.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(self.style.prepare());
        try!(write!(f, "{}", self.obj));
        self.style.cleanup()
    }
}

impl<T: Debug> Debug for Painted<T> {
    /// Implementation for `T: Debug` -> to print with `{:?}`.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(self.style.prepare());
        try!(write!(f, "{:?}", self.obj));
        self.style.cleanup()
    }
}


// ----- Tests ------
#[cfg(test)]
mod test {
    use super::Color::*;
    use super::Attr::*;
    use super::ToStyle;

    #[test]
    fn modifier_order() {
        // The order of modifiers shouldn't play a role.
        assert_eq!(Plain.bold().fg(Red), Plain.fg(Red).bold());
        assert_eq!(Plain.bold().bg(Red), Plain.bg(Red).bold());
        assert_eq!(Plain.underline().fg(Red), Plain.fg(Red).underline());

        // The startpoints should have the same effect as the modifier.
        assert_eq!(Red.to_style(), Plain.fg(Red));
        assert_eq!(Bold.to_style(), Plain.bold());
    }

    #[test]
    fn modifier_override() {
        // The latter modifier should override the one before
        assert_eq!(Plain.fg(Red).fg(Blue), Plain.fg(Blue));
        assert_eq!(Plain.fg(Red).fg(Blue), Blue.to_style());
        assert_eq!(Red.fg(Blue), Plain.fg(Blue));
        assert_eq!(Red.fg(Blue), Blue.to_style());
    }
}
