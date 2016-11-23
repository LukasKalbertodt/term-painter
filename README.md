# Coloring terminal ouput
[![Build Status](https://img.shields.io/travis/LukasKalbertodt/term-painter/master.svg)](https://travis-ci.org/LukasKalbertodt/term-painter)
[![crates.io version](https://img.shields.io/crates/v/term-painter.svg)](https://crates.io/crates/term-painter)
[![GitHub license](https://img.shields.io/github/license/LukasKalbertodt/term-painter.svg)]()

[**Documentation**](https://lukaskalbertodt.github.io/term-painter/term_painter/)

`term-painter` is a cross-platform Rust library for coloring and formatting terminal output.
It provides easy ways to format various things and uses the crate [`rust-lang/term`][term] to do the actual formatting.
Example:

``` Rust
println!("{} | {} | {} | {} | {}",
    Red.bg(Green).bold().paint("Red-Green-Bold"),
    Blue.paint("Blue"),
    Blue.bold().paint("BlueBold"),
    Blue.bg(Magenta).paint("BlueMagentaBG"),
    Plain.underline().paint("Underline")
);

Red.with(|| {
    print!("JustRed");
    Bold.with(|| {
        print!(" BoldRed {} BoldRed ", Underline.paint("Underline"));
    });
    print!("JustRed ");

    print!("{}", Blue.paint("Blue (overwrite) "));
    Green.with(|| {
        println!("Green (overwrite)");
    });
});
```

![alt text](https://raw.githubusercontent.com/LukasKalbertodt/term-painter/master/media/readme_example.png "Result of code snippet above")

It's easy to use and integrates well with `println!`/`print!`.
The main design goal was to make it simple.
This has one performance disadvantage: It will often reset the terminal style after each printing operation.
But performance isn't usually hugely important when printing on the terminal, so simplicity was more important for the design of this library.

More examples [here (`examples/main.rs`)](https://github.com/LukasKalbertodt/term-painter/blob/master/examples/main.rs) or in the [**Documentation**](https://lukaskalbertodt.github.io/term-painter/term_painter/).

## Cross Platform

This crate uses [`rust-lang/term`][term] internally.
`term` supports all (or rather: many) platforms, hence `term-painter` does, too.

*How does it work?* In order to work, this crate depends on a specific way how `println!` and friends evaluate their arguments (which is the common-sense way).
There are no guarantees about argument evaluation, but currently it works.
And honestly, it's unlikely that this way of evaluation ever changes.
But, for example, if `println!()` would always call `format!()` first and print the resulting `String`, it wouldn't work.

[term]: https://crates.io/crates/term

## Usage
Just add `extern crate term_painter;` in your crate root and the dependency in
your `Cargo.toml` file:

```
[dependencies]
term-painter = "0.2"
```

## Collaboration
Yes please! If you find a bug, want to request a feature or anything else: Please open an issue or create a pull request.

## Thanks
I've got some design ideas from [`rust-ansi-term`](https://github.com/ogham/rust-ansi-term).
I decided to make my own crate though, since my goals were too different from `ansi-term` (specifically: `ansi-term` does not work everywhere).
