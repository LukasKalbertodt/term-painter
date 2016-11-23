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
//!         Red.bold().paint("Both!")
//!     );
//! }
//! ```
//!
//! This crate uses [`rust-lang/term`][term] to do the formatting. Of course,
//! you can use `term` directly, but it's kinda clumsy. Hence this library.
//!
//! [term]: https://crates.io/crates/term
//!
//!
//! How to use it
//! -------------
//! Formatting is done in two steps:
//!
//! 1. Creating a style
//! 2. Use this style to "paint" something and get a `Painted` object
//!
//!
//! 1. Creating a style
//! -------------------
//! To create a style a startpoint is needed: This can either be a startpoint
//! with an attached modifier (like `Red`: modifies the fg-color) or the
//! `Plain` startpoint, which does not modify anything.
//! After that, the startpoint can be modified with modifiers like `bold()` or
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
//!     // These two are equivalent: nothing is formatted/painted
//!     println!("{} | {}", x, Plain.paint(x));
//!
//!     // These two are equivalent, too
//!     println!("{} | {}", Red.paint(x), Plain.fg(Red).paint(x));
//! }
//! ```
//! You can chain as many modifiers as you want. Every modifier overrides
//! preceding modifier:
//!
//! ```
//! # use term_painter::Attr::*;
//! # use term_painter::Color::*;
//! # use term_painter::ToStyle;
//!
//! // blue, not red
//! println!("{}", Plain.fg(Red).fg(Blue).paint("Apple"));
//! ```
//!
//! 2. Use the style
//! ----------------
//! After building the style, you can use it in two different ways.
//!
//! One way is to call `paint` to use it on some object.
//! `paint` will return the wrapper object `Painted` that holds your object and
//! the specified style. `Painted` implements any formatting trait (like
//! `Display` and `Debug`) if and only if the type of the given Object, `T`,
//! does. So a `Painted` object can be printed via `println!` or similar macros.
//! When it gets printed, it will apply the given style before printing the
//! object of type `T` and will reset the style after printing.
//!
//! `Note`: `paint` will consume the passed object. This is no problem when
//! passing constant literals (like `paint("cheesecake")`) or types that are
//! `Copy`. Otherwise it could be confusing because just printing should not
//! consume a variable. To prevent consuming, just pass a reference to the
//! object (with `&`). Example:
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
//!     let copy = 27;  // i32 *is* Copy
//!
//!     println!("{}", Plain.paint(&non_copy));
//!     println!("{}", Plain.paint(&copy));
//!     // non_copy is still usable here...
//!     // copy is still usable here...
//!
//!     println!("{}", Plain.paint(non_copy));
//!     println!("{}", Plain.paint(copy));
//!     // non_copy was moved into `paint`, so it not usable anymore...
//!     // copy is still usable here...
//! }
//! ```
//!
//! Another way is to call `with`. `with` takes another function (usually a
//! closure) and everything that is printed within that closure is formatted
//! with the given style. Specifically, `with()` sets the given style,
//! calls the given function and resets the style afterwards. It can be
//! chained and used together with `paint()`. Inner calls will overwrite
//! outer calls of `with`.
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
//! `use` statements:
//!
//! - `use term_painter::Color::*;`
//! - `use term_painter::Attr::*;`
//!
//! Please note that global state is changed when printing a `Painted`
//! object. This means that some state is set before and reset after printing.
//! This means that, for example, using this library in `format!` or `write!`
//! won't work. The color formatting is not stored in the resulting string.
//! Although Unix terminals do modify color and formatting by printing special
//! control characters, Windows and others do not. This library uses the
//! plattform independent library `term`, thus saving formatted text in a
//! string not possible. This was a design choice.
//!
//! This crate also assumes that the terminal state is not altered by anything
//! else. Calling `term` function directly might result in strange behaviour.
//! This is due to the fact that one can not read the current terminal state.
//! In order to work like this, this crate needs to track terminal state
//! itself. However, there shouldn't be any problems when the terminal state
//! is completely reset in between using those two different methods.
//!
//! Another possible source of confusion might be multithreading. Terminal
//! state and handles are hold in thread local variables. If two terminal
//! handles would reference the same physical terminal, those two threads could
//! interfere with each other. I have not tested it, though. Usually, you don't
//! want to print to the same Terminal in two threads simultanously anyway.
//!
//! Functions of `term` sometimes return a `Result` that is `Err` when the
//! function fails to set the state. However, this crate silently ignores those
//! failures. To check the capabilities of the terminal, use `term` directly.
//!

extern crate term;

use std::default::Default;
use std::fmt;
use std::cell::RefCell;


/// Everything that can be seen as part of a style. This is the core of this
/// crate. All functions ("style modifier") consume self and return a modified
/// version of the style.
pub trait ToStyle : Sized {
    fn to_style(self) -> Style;

    /// Convenience method for modifying the style before it's returned.
    fn to_mapped_style<F>(self, func: F) -> Style
        where F: FnOnce(&mut Style)
    {
        let mut s = self.to_style();
        func(&mut s);
        s
    }

    /// Sets the foreground (text) color.
    fn fg(self, c: Color) -> Style {
        self.to_mapped_style(|s| s.fg = c)
    }

    /// Sets the background color.
    fn bg(self, c: Color) -> Style {
        self.to_mapped_style(|s| s.bg = c)
    }

    /// Makes the text bold.
    fn bold(self) -> Style {
        self.to_mapped_style(|s| s.set_bold(Some(true)))
    }

    /// Dim mode.
    fn dim(self) -> Style {
        self.to_mapped_style(|s| s.set_dim(Some(true)))
    }

    /// Underlines the text.
    fn underline(self) -> Style {
        self.to_mapped_style(|s| s.set_underline(Some(true)))
    }

    /// Removes underline-attribute.
    fn not_underline(self) -> Style {
        self.to_mapped_style(|s| s.set_underline(Some(false)))
    }

    /// Underlines the text.
    fn blink(self) -> Style {
        self.to_mapped_style(|s| s.set_blink(Some(true)))
    }

    /// Underlines the text.
    fn reverse(self) -> Style {
        self.to_mapped_style(|s| s.set_reverse(Some(true)))
    }

    /// Secure mode.
    fn secure(self) -> Style {
        self.to_mapped_style(|s| s.set_secure(Some(true)))
    }

    /// Wraps the style specified in `self` and something of arbitrary type
    /// into a `Painted`. When `Painted` is printed it will print the arbitrary
    /// something with the given style.
    fn paint<T>(&self, obj: T) -> Painted<T>
        where Self: Clone
    {
        Painted {
            style: self.clone().to_style(),
            obj: obj,
        }
    }

    /// Executes the given function, applying the style information before
    /// calling it and resetting after it finished.
    fn with<F, R>(&self, f: F) -> R
        where F: FnOnce() -> R,
              Self: Clone
    {
        // Shorthand for the new style and the style that was active before
        let new = self.clone().to_style();
        let before = CURR_STYLE.with(|curr| curr.borrow().clone());

        // Apply the new style and setting the merged style as CURR_STYLE
        let _ = new.apply();
        CURR_STYLE.with(|curr| *curr.borrow_mut() = before.and(new));

        let out = f();

        // Revert to the style that was active before and set it as current
        let _ = before.revert_to();
        CURR_STYLE.with(|curr| *curr.borrow_mut() = before);

        out
    }
}

/// Lists all possible Colors. It implements `ToStyle` so it's possible to call
/// `ToStyle`'s methods directly on a `Color` variant like:
///
/// ```
/// # use term_painter::{Attr, Color, ToStyle};
///
/// println!("{}", Color::Red.bold().paint("Red and bold"));
/// ```
///
/// It is not guaranteed that the local terminal supports all of those colors.
/// As already mentioned in the module documentation, you should use `term`
/// directly to check the terminal's capabilities.
///
/// **Note**: Using `Color::NotSet` will *not* reset the color to the default
/// terminal color.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    NotSet,
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
    Custom(u16),
}

impl Color {
    /// Returns the associated constant from `term::color::Color`.
    fn term_constant(&self) -> Option<term::color::Color> {
        match *self {
            Color::NotSet        => None,
            Color::Black         => Some(term::color::BLACK),
            Color::Red           => Some(term::color::RED),
            Color::Green         => Some(term::color::GREEN),
            Color::Yellow        => Some(term::color::YELLOW),
            Color::Blue          => Some(term::color::BLUE),
            Color::Magenta       => Some(term::color::MAGENTA),
            Color::Cyan          => Some(term::color::CYAN),
            Color::White         => Some(term::color::WHITE),
            Color::BrightBlack   => Some(term::color::BRIGHT_BLACK),
            Color::BrightRed     => Some(term::color::BRIGHT_RED),
            Color::BrightGreen   => Some(term::color::BRIGHT_GREEN),
            Color::BrightYellow  => Some(term::color::BRIGHT_YELLOW),
            Color::BrightBlue    => Some(term::color::BRIGHT_BLUE),
            Color::BrightMagenta => Some(term::color::BRIGHT_MAGENTA),
            Color::BrightCyan    => Some(term::color::BRIGHT_CYAN),
            Color::BrightWhite   => Some(term::color::BRIGHT_WHITE),
            Color::Custom(c)     => Some(c)
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::NotSet
    }
}

impl ToStyle for Color {
    /// Returns a Style with default values and the `self` color as foreground
    /// color.
    fn to_style(self) -> Style {
        Style {
            fg: self,
            .. Style::default()
        }
    }
}

/// Lists possible attributes. It implements `ToStyle` so it's possible to call
/// `ToStyle`'s methods directly on a `Attr` variant like:
///
/// ```
/// # use term_painter::{Attr, Color, ToStyle};
///
/// println!("{}", Attr::Bold.fg(Color::Red).paint("Red and bold"));
/// ```
///
/// It is not guaranteed that the local terminal supports all of those
/// formatting options. As already mentioned in the module documentation, you
/// should use `term` directly to check the terminal's capabilities.
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
            Attr::Bold => s.set_bold(Some(true)),
            Attr::Dim => s.set_dim(Some(true)),
            Attr::Underline => s.set_underline(Some(true)),
            Attr::Blink => s.set_blink(Some(true)),
            Attr::Reverse => s.set_reverse(Some(true)),
            Attr::Secure => s.set_secure(Some(true)),
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
    // Each attribute was `Option<bool>` once. To reduce struct size, the
    // Option type is simulated with 2 bits for each attribute. The first
    // attribute in the name uses the MSBs, the last attribute the LSBs.
    // 00 => None, 10 => Some(false), 11 => Some(true)
    bold_dim_underline_blink: u8,
    reverse_secure: u8,
}


impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Color::default(),
            bg: Color::default(),
            bold_dim_underline_blink: 0,
            reverse_secure: 0,
        }
    }
}

thread_local!(
    static TERM: RefCell<Option<Box<term::StdoutTerminal>>> = RefCell::new(term::stdout())
);
thread_local!(
    static CURR_STYLE: RefCell<Style> = RefCell::new(Style::default())
);

// Macro to generate getter and setter for all attributes. This hides almost
// all bit magic in here.
macro_rules! gen_getter {
    ($getter:ident, $setter:ident, $var:ident, $pos:expr) => {
        pub fn $getter(&self) -> Option<bool> {
            // shift important bits to the right and mask them
            match (self.$var >> ($pos * 2)) & 0b11 {
                0b10 => Some(false),
                0b11 => Some(true),
                _ => None,
            }
        }

        pub fn $setter(&mut self, v: Option<bool>) {
            match v {
                None => {
                    // Set important bits to 00
                    self.$var &= !(0b11 << ($pos*2));
                },
                Some(false) => {
                    // Set important bits to 10
                    self.$var &= !(0b01 << ($pos*2));
                    self.$var |= 0b10 << ($pos*2);
                },
                Some(true) => {
                    // Set important bits to 11
                    self.$var |= 0b11 << ($pos*2);
                },
            }
        }
    }
}

impl Style {
    // Generate a bunch of getters and setters to hide bit fiddling.
    gen_getter!(get_bold,       set_bold,       bold_dim_underline_blink, 3);
    gen_getter!(get_dim,        set_dim,        bold_dim_underline_blink, 2);
    gen_getter!(get_underline,  set_underline,  bold_dim_underline_blink, 1);
    gen_getter!(get_blink,      set_blink,      bold_dim_underline_blink, 0);
    gen_getter!(get_reverse,    set_reverse,    reverse_secure, 3);
    gen_getter!(get_secure,     set_secure,     reverse_secure, 2);


    fn apply(&self) -> Result<(), fmt::Error> {
        // Like `try!`, but converts `term`-Error into `fmt::Error`
        macro_rules! try_term {
            ($e:expr) => { try!($e.map_err(|_| fmt::Error)) }
        }

        TERM.with(|term_opt| {
            let mut tmut = term_opt.borrow_mut();
            let mut t = match tmut.as_mut() {
                None => return Err(fmt::Error),
                Some(t) => t,
            };

            // Apply colors if set.
            if let Some(c) = self.fg.term_constant() {
                try_term!(t.fg(c));
            }
            if let Some(c) = self.bg.term_constant() {
                try_term!(t.bg(c));
            }

            // For all attributes: Apply, when set.
            if let Some(true) = self.get_bold() {
                try_term!(t.attr(term::Attr::Bold));
            }
            if let Some(true) = self.get_dim() {
                try_term!(t.attr(term::Attr::Dim));
            }
            if let Some(u) = self.get_underline() {
                try_term!(t.attr(term::Attr::Underline(u)));
            }
            if let Some(true) = self.get_blink() {
                try_term!(t.attr(term::Attr::Blink));
            }
            if let Some(true) = self.get_reverse() {
                try_term!(t.attr(term::Attr::Reverse))
            }
            if let Some(true) = self.get_secure() {
                try_term!(t.attr(term::Attr::Secure))
            }

            Ok(())
        })
    }

    /// `o` overrides values of `self`.
    fn and(&self, o: Style) -> Style {
        // Some shortcuts for bitfields.
        let ax = self.bold_dim_underline_blink;
        let ay = o.bold_dim_underline_blink;
        let bx = self.reverse_secure;
        let by = o.reverse_secure;

        // The following is equivalent to
        //     `s.set_attr(o.get_attr().and(self.get_attr()));`
        // for every attribute.
        //
        // But we can do better with some bit operations.
        // There are two bits for each attribute: The setbit and valuebit.
        // The resulting setbit is just an bitwise OR of both input setbits.
        // The resulting valuebit is either the one of y (if y's set bit is
        // set) or the one of x (otherwise).
        let az = ((ax | ay) & 0b10101010) | (((ay >> 1) & ay | !(ay >> 1) & ax) & 0b01010101);
        let bz = ((bx | by) & 0b10101010) | (((by >> 1) & by | !(by >> 1) & bx) & 0b01010101);

        Style {
            fg: if o.fg == Color::NotSet { self.fg } else { o.fg },
            bg: if o.bg == Color::NotSet { self.bg } else { o.bg },
            bold_dim_underline_blink: az,
            reverse_secure: bz,
        }
    }

    /// Resets the whole terminal and applies this style.
    fn revert_to(&self) -> Result<(), fmt::Error> {
        try!(TERM.with(|term_opt| {
            let mut tmut = term_opt.borrow_mut();
            tmut.as_mut()
                .and_then(|t| t.reset().ok())
                .ok_or(fmt::Error)
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

/// Wraps an object of type `T` and a style. When attempting to print it, the
/// given style is applied before printing and reset afterwards.
/// All formatting traits (`Display`, `Debug`, ...) that are implemented
/// for `T` are also implemented the wrapper type `Painted<T>`.
pub struct Painted<T> {
    style: Style,
    obj: T,
}

macro_rules! impl_format {
    ($symbol:expr, $fmt:ident) => {
        impl<T: fmt::$fmt> fmt::$fmt for Painted<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                self.style.with(|| fmt::$fmt::fmt(&self.obj, f))
            }
        }
    }
}

impl_format!("{}", Display);
impl_format!("{:?}", Debug);
impl_format!("{:o}", Octal);
impl_format!("{:x}", LowerHex);
impl_format!("{:X}", UpperHex);
impl_format!("{:p}", Pointer);
impl_format!("{:b}", Binary);
impl_format!("{:e}", LowerExp);
impl_format!("{:E}", UpperExp);


// ----- Tests ------
#[cfg(test)]
mod test {
    use super::Color::*;
    use super::Attr::*;
    use super::{ToStyle, Style};

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

    #[test]
    fn style_and() {
        let s1 = Style::default().bold().not_underline();
        let s2 = Style::default().underline();
        let s3 = Style::default().bold();

        let r1 = Style::default().bold().underline();
        let r2 = Style::default().bold().not_underline();

        assert_eq!(s2.and(s1), r2);
        assert_eq!(s2.and(s1).and(s3), r2);
        assert_eq!(s2.and(s3), r1);
    }
}
