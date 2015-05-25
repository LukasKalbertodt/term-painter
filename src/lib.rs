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
//! This crate uses `rust-lang/term` to do the formatting. You can of course
//! use `term` directly, but it's kinda clumsy. Hence this library.
//!
//!
//! How to use it
//! -------------
//! Formatting works in two steps mainly:
//!
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
//!    // These two are equivalent
//!    println!("{} | {}", x, Plain.paint(x));
//!
//!    // These two are equivalent, too
//!    println!("{} | {}", Red.paint(x), Plain.fg(Red).paint(x));
//! }
//! ```
//! You can chain as many modifier as you want. Every modifier overrides
//! preceding modifier:
//!
//! `println("{}", Plain.fg(Red).fg(Blue).paint("Apple")); // blue, not red`
//!
//! 2. Use the style
//! ----------------
//! After building the style, you can use it in two different ways.
//!
//! One way is to call `paint` to use it on some object.
//! `paint` will return the wrapper object `Painted` that holds your object and
//! the specified style. `Painted` implements `Display` and/or `Debug` if the
//! type of the given Object, `T`, does. So the `Painted` object can be printed
//! via `println!` or similar macros. When it gets printed, it will apply the
//! given style before printing the object of type `T` and will reset the style
//! after printing.
//!
//! `Note`: `paint` will consume the passed object. This is no problem when
//! passing constant literals (like `paint("cheesecake")`) or types that are
//! `Copy`. Otherwise it could be confusing because just printing should not
//! consume a variable. To prevent consuming, just pass a borrow to the object
//! (with `&`). Example:
//!
//! ```
//! extern crate term_painter;
//!
//! use term_painter::ToStyle;
//! use term_painter::Color::*;
//! use term_painter::Attr::*;
//!
//! fn main() {
//!     let non_copy = "cake".to_string();  // String is *not* Copy
//!     let copy = 27;  // usize/isize *is* Copy
//!
//!     println!("{}", Plain.paint(&non_copy));
//!     println!("{}", Plain.paint(&copy));
//!     // non_copy is still usable here...
//!     // copy is still usable here...
//!
//!     println!("{}", Plain.paint(non_copy));
//!     println!("{}", Plain.paint(copy));
//!     // non_copy was moved into paint, so it not usable anymore...
//!     // copy is still usable here...
//! }
//! ```
//!
//! Another way is to call `with`. `with` takes another function (usually a
//! closure) and everything that is printed within that closure is formatted
//! with the given style. It can be chained and used together with `paint`.
//! Inner calls will overwrite outer calls of `with`.
//!
//! ```
//! extern crate term_painter;
//!
//! use term_painter::ToStyle;
//! use term_painter::Color::*;
//! use term_painter::Attr::*;
//!
//! fn main() {
//!     Red.with(|| {
//!         print!("JustRed");
//!         Bold.with(|| {
//!             print!(" BoldRed {} BoldRed ", Underline.paint("Underline"));
//!         });
//!         print!("JustRed ");
//!
//!          print!("{}", Blue.paint("Blue (overwrite) "));
//!          Green.with(|| {
//!              println!("Green (overwrite)");
//!          });
//!     });
//! }
//! ```
//!
//! Some Notes
//! ----------
//! If you don't want to pollute your namespace with `Color` and `Attr` names,
//! you can use a more qualified name (`Color::Red.paint(..)`) and remove these
//! `use` statements: `use term_painter::Color::*;` and
//! `use term_painter::Attr::*;`.
//!
//! And please note that global state is changed when printing a `Painted`
//! object. This means that some state is set before and reset after printing.
//! This means that, for example, using this library in `format!` or `write!`
//! won't work. The color formatting is not stored in the resulting string.
//! Although Unix terminals do modify color and formatting by printing special
//! control characters, Windows and others do not. And since this library uses
//! the plattform independent library `term`. This was a design choice.
//!
//! This crate also assumes that the terminal state is not altered by anything
//! else. Calling `term` function directly might result in strange behaviour.
//! This is due to the fact that one can not read the current terminal state.
//! In order to work like this, this crate needs to track terminal state
//! itself. However, there shouldn't be any problems when the terminal state
//! is completely reset in between using those two methods.
//!
//! Another possible source of confusion might be multithreading. Terminal
//! state and handles are hold in thread local variables. If two terminal
//! handles would reference the same physical terminal, those two threads could
//! interfere with each other. I have not tested it though.
//!
//! Functions of `term` sometimes return a `Result` that is `Err` when the
//! function fails to set state. However, this crate silently ignores those
//! failures. To check the capabilities of the terminal, use `term` directly.
//!
//!

extern crate term;

use std::default::Default;
use std::fmt::{Display, Debug, Error, Formatter};
use std::cell::RefCell;


/// Everything that can be seen as part of a style. This is the core of this
/// crate. All functions ("style modifier") consume self and return a modified
/// version of the style.
pub trait ToStyle : Sized {
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
        s.bold = Some(true);
        s
    }

    /// Dim mode.
    fn dim(self) -> Style {
        let mut s = self.to_style();
        s.dim = Some(true);
        s
    }

    /// Underlines the text.
    fn underline(self) -> Style {
        let mut s = self.to_style();
        s.underline = Some(true);
        s
    }

    /// Underlines the text.
    fn blink(self) -> Style {
        let mut s = self.to_style();
        s.blink = Some(true);
        s
    }

    /// Underlines the text.
    fn reverse(self) -> Style {
        let mut s = self.to_style();
        s.reverse = Some(true);
        s
    }

    /// Secure mode.
    fn secure(self) -> Style {
        let mut s = self.to_style();
        s.secure = Some(true);
        s
    }

    /// Wraps the style specified in `self` and something of arbitrary type
    /// into a `Painted`. When `Painted` is printed it will print the arbitrary
    /// something with the given style. `T` needs to implement
    /// `std::fmt::Display` or `std::fmt::Debug`.
    fn paint<T>(&self, obj: T) -> Painted<T>
        where Self: Clone {
        Painted { style: self.clone().to_style(), obj: obj }
    }

    /// Executes the given function, applying the style information before
    /// calling it and resetting after it finished.
    #[allow(unused_must_use)]
    fn with<F, R>(&self, f: F) -> R
        where F: FnOnce() -> R, Self: Clone {
        // Shorthand for the new style and the style that was active before
        let new = self.clone().to_style();
        let before = CURR_STYLE.with(|curr| curr.borrow().clone());

        // Apply the new style and setting the merged style as CURR_STYLE
        new.apply();
        CURR_STYLE.with(|curr| *curr.borrow_mut() = before.and(new));

        let out = f();

        // Revert to the style that was active before and set it as current
        before.revert_to();
        CURR_STYLE.with(|curr| *curr.borrow_mut() = before);

        out
    }
}

/// Lists all possible Colors. It implements `ToStyle` so it's possible to call
/// `ToStyle`'s methods directly on a `Color` variant like:
///
/// `println!("{}", Color::Red.bold().paint("Red and bold"));`
///
/// Note: Using `Color::Normal` will *not* reset the color to the default
/// terminal color.
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
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
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
            Color::BrightBlack   => Some(term::color::BRIGHT_BLACK),
            Color::BrightRed     => Some(term::color::BRIGHT_RED),
            Color::BrightGreen   => Some(term::color::BRIGHT_GREEN),
            Color::BrightYellow  => Some(term::color::BRIGHT_YELLOW),
            Color::BrightBlue    => Some(term::color::BRIGHT_BLUE),
            Color::BrightMagenta => Some(term::color::BRIGHT_MAGENTA),
            Color::BrightCyan    => Some(term::color::BRIGHT_CYAN),
            Color::BrightWhite   => Some(term::color::BRIGHT_WHITE),
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
///
/// For more information about enum variants, see `term::Attr` Documentation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Attr {
    /// Just default style
    Plain,
    Bold,
    Dim,
    Underline,
    Blink,
    Reverse,
    Secure,
}

impl ToStyle for Attr {
    /// Returns a Style with default values and the `self` attribute enabled.
    fn to_style(self) -> Style {
        let mut s = Style::default();
        match self {
            Attr::Plain => {},
            Attr::Bold => s.bold = Some(true),
            Attr::Dim => s.dim = Some(true),
            Attr::Underline => s.underline = Some(true),
            Attr::Blink => s.blink = Some(true),
            Attr::Reverse => s.reverse = Some(true),
            Attr::Secure => s.secure = Some(true),
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
    pub bold: Option<bool>,
    pub dim: Option<bool>,
    pub underline: Option<bool>,
    pub blink: Option<bool>,
    pub reverse: Option<bool>,
    pub secure: Option<bool>,
}


impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Color::default(),
            bg: Color::default(),
            bold: None,
            dim: None,
            underline: None,
            blink: None,
            reverse: None,
            secure: None,
        }
    }
}

thread_local!(static TERM: RefCell<Option<Box<term::StdoutTerminal>>>
    = RefCell::new(term::stdout()));
thread_local!(static CURR_STYLE: RefCell<Style>
    = RefCell::new(Style::default()));


impl Style {
    fn apply(&self) -> Result<(), Error> {
        macro_rules! try_term {
            ($e:expr) => ({
                match $e {
                    Ok(true) => {},
                    _ => { return Err(Error); },
                }
            })
        }

        TERM.with(|term_opt| {
            let mut tmut = term_opt.borrow_mut();
            let mut t = match tmut.as_mut() {
                None => return Err(Error),
                Some(t) => t,
            };

            match self.fg.term_constant() {
                None => {},
                Some(c) => { try_term!(t.fg(c)); },
            }
            match self.bg.term_constant() {
                None => {},
                Some(c) => { try_term!(t.bg(c)); },
            }
            if self.bold.unwrap_or(false) {
                try_term!(t.attr(term::Attr::Bold))
            }
            match self.underline {
                Some(u) => try_term!(t.attr(term::Attr::Underline(u))),
                None => {},
            }
            if self.blink.unwrap_or(false) {
                try_term!(t.attr(term::Attr::Blink))
            }
            if self.reverse.unwrap_or(false) {
                try_term!(t.attr(term::Attr::Reverse))
            }
            if self.secure.unwrap_or(false) {
                try_term!(t.attr(term::Attr::Secure))
            }

            Ok(())
        })
    }

    // `o` overrides values of `self`
    fn and(&self, o: Style) -> Style {
        Style {
            fg: if o.fg == Color::Normal { self.fg } else { o.fg },
            bg: if o.bg == Color::Normal { self.bg } else { o.bg },
            bold: o.bold.or(self.bold),
            dim: o.dim.or(self.dim),
            underline: o.underline.or(self.underline),
            blink: o.blink.or(self.blink),
            reverse: o.reverse.or(self.reverse),
            secure: o.secure.or(self.secure),
        }
    }

    fn revert_to(&self) -> Result<(), Error> {
        try!(TERM.with(|term_opt| {
            let mut tmut = term_opt.borrow_mut();
            match tmut.as_mut() {
                None => Err(Error),
                Some(t) => match t.reset() {
                    Ok(..) => Ok(()),
                    Err(..) => Err(Error),
                },
            }
        }));
        self.apply()
    }
}

impl ToStyle for Style {
    /// Dummy implementation that just returns `self`.
    fn to_style(self) -> Style {
        self
    }
}

/// Saves a style and a reference to something that will be printed in that
/// style. That something of type `T` needs to implement at least one of
/// `std::fmt::Debug` and `std::fmt::Display`.
pub struct Painted<T> {
    style: Style,
    obj: T,
}

impl<T: Display> Display for Painted<T> {
    /// Implementation for `T: Display` -> to print with `{}`.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.style.with(|| write!(f, "{}", self.obj))
    }
}

impl<T: Debug> Debug for Painted<T> {
    /// Implementation for `T: Debug` -> to print with `{:?}`.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.style.with(|| write!(f, "{:?}", self.obj))
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
