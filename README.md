# Coloring terminal ouput
[![Build Status](https://img.shields.io/travis/LukasKalbertodt/term-painter/master.svg)](https://travis-ci.org/LukasKalbertodt/term-painter)
[![crates.io version](https://img.shields.io/crates/v/term-painter.svg)](https://crates.io/crates/term-painter)
[![GitHub license](https://img.shields.io/github/license/LukasKalbertodt/term-painter.svg)]()

[**Documentation**](https://docs.rs/term-painter/)

`term-painter` is a cross-platform (i.e. also non-ANSI terminals) Rust library for coloring and formatting terminal output.
It provides easy ways to format various things and uses the crate [`rust-lang/term`][term] to do the actual formatting. **Please read ["When (not) to use this crate"](#when-not-to-use-this-crate)**.
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

## When (not) to use this crate

There are more terminal color crates than stars in the observable universe, therefore it's a valid question to ask "which one is best"? Unfortunately, there is no clear answer, I think. 

**_Don't_ use this crate, if:**
- you want full power of what happens (consider using `rust-lang/term` instead), *or*
- you want to print from multiple threads (consider using [`termcolor`](https://crates.io/crates/termcolor) instead), *or*
- you want to color/format text you print on something else than stdout (however, `term-painter` might add support for stderr in the future)
- you want an actively developed crate (see ["Status of this crate"](#status-of-this-crate))
- you want to use a crate with a fancy name (`term-painter` is such a boring name :unamused:)

**You _probably shouldn't_ use this crate, if:**
- you don't need to support non-ANSI terminals (Only supporting ANSI-formatting gives the author of the lib greater flexibility in designing the API, thus potentially making it easier and more powerful. See [section "Cross Platform"](#cross-platform). Consider using [`ansi-term`](https://crates.io/crates/ansi_term), [`colored`](https://crates.io/crates/colored), [`yansi`](https://crates.io/crates/yansi), ... instead.), *or*
- you expect a time-proven library

**Use this crate, if:**
- you need support for non-ANSI terminals, *and*
- you are developing a non-mission critical project

## Cross Platform

This crate uses [`rust-lang/term`][term] internally.
`term` supports all (or rather: many) platforms, hence `term-painter` does, too.

*How does it work?* In order to work, this crate depends on a specific way how `println!` and friends evaluate their arguments (which is the common-sense way).
There are no guarantees about argument evaluation, but currently it works.
And honestly, it's unlikely that this way of evaluation ever changes.
But, for example, if `println!()` would always call `format!()` first and print the resulting `String`, it wouldn't work.

To give a simplified explanation of the terminal-world: there are ANSI-terminal and non-ANSI Terminals. ANSI-formatting works by inserting special control characters into the string. Thus you can easily store the formatted string in a `String` to print it later. Non-ANSI terminals work differently, and the most commonly used one is `cmd` which is part of Windows 7/10. Formatting for `cmd` works by calling methods of the winapi before and after printing. Thus you *cannot* easily store a formatted string. Apart from that, AFAIK there are not that many developers mainly using `cmd` -- most Windows developer use IDEs alone or another terminal for Windows (there are plenty).

In summary: most terminals support ANSI-coloring, non-ANSI-terminals make the world more complicated.

[term]: https://crates.io/crates/term

## Status of this crate

This crate is not actively developed anymore. The API is likely to stay as it is. I doubt this crate will reach its 1.0 milestone. This doesn't mean that this crate doesn't work! You can still use it, if it fits your needs.

## Thanks
I've got some design ideas from [`rust-ansi-term`](https://github.com/ogham/rust-ansi-term).
I decided to make my own crate though, since my goals were too different from `ansi-term` (specifically: `ansi-term` does not work everywhere).

---

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
