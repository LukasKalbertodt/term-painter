# Coloring terminal ouput
[![Build Status](https://travis-ci.org/LukasKalbertodt/term-painter.svg?branch=master)](https://travis-ci.org/LukasKalbertodt/term-painter)
[![crates.io version](https://img.shields.io/crates/v/term-painter.svg)](https://crates.io/crates/term-painter)

`term-painter` is a Rust library for coloring and formatting terminal output. It provides easy ways to format various things and uses the crate `rust-lang/term` to do the actual formatting. Example:

``` Rust
println!("{} | {} | {} | {} | {}",
    Red.bg(Green).bold().paint("Red-Green-Bold"),
    Blue.paint("Blue"),
    Blue.bold().paint("BlueBold"),
    Blue.bg(Magenta).paint("BlueMagentaBG"),
    Plain.underline().paint("Underline"));

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

It's easy to use and integrates well with `println!`/`print!`. The main design
goal was to make it simple. This has one performance disadvantage: It will often reset the terminal style after each printing operation. But performance isn't usually hugely important when printing on the terminal, so simplicity was more important for the design of this library.

More examples [here (`examples/main.rs`)](https://github.com/LukasKalbertodt/term-painter/blob/master/examples/main.rs) or in the [**Documentation**](https://lukaskalbertodt.github.io/term-painter/term_painter/).

## Usage
Just add `extern crate term_painter;` in your crate root and the dependency in
your `Cargo.toml` file:

```
[dependencies]
term-painter = "*"
```

## Collaboration
Yes please! If you find a bug, have any feature request or anything else: Please open a issue or create a pull request.

## Thanks
I've got some design ideas from [`rust-ansi-term`](https://github.com/ogham/rust-ansi-term). I decided to make my own crate though, since my goals were too different from `ansi-term`.
