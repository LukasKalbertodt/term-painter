# Coloring terminal ouput
`term-painter` is a Rust library for coloring and formatting terminal output. It provides easy ways to format various things and uses the crate `rust-lang/term` to do the actual formatting. Example:

``` Rust
println!("{}\n{}\n{}\n{}\n{}",
        Blue.paint("Blue"),
        Blue.bold().paint("Blue-Bold"),
        Blue.bold().bg(Green).paint("Blue-Green-Bold"),
        Plain.underline().paint("Just Underline"));
```

It's easy to use and integrates well with `print!` and friends. The main design
goal was to make it simple. This has one performance disadvantage: It will reset the terminal style after each printing operation. But performance isn't usually hugly important when printing on the terminal, so simplicity is more important.

More examples [here (`examples/main.rs`)](https://github.com/LukasKalbertodt/term-painter/blob/master/examples/main.rs) or in the [**Documentation**](https://lukaskalbertodt.github.io/term-painter/term_painter/).

## Collaboration
Yes please! If you find a bug, have any feature request or anything else: Please open a issue or create a pull request.

## Thanks
I've got some design ideas from [`rust-ansi-term`](https://github.com/ogham/rust-ansi-term). I decided to make my own crate though, since my goals were too different from `ansi-term`.
