


# Rust Standard Library Cookbook
This is the code repository for [Rust Standard Library Cookbook](https://www.packtpub.com/application-development/rust-standard-library-cookbook?utm_source=github&utm_medium=repository&utm_campaign=9781788623926), published by [Packt](https://www.packtpub.com/?utm_source=github). It contains all the supporting project files necessary to work through the book from start to finish.
## About the Book
Mozilla’s Rust is gaining much attention with amazing features and a powerful library. This book will take you through varied recipes to teach you how to leverage the Standard library to implement efficient solutions.

## Instructions and Navigation
All of the code is organized into folders. Each folder starts with a number followed by the application name. For example, Chapter02.

All the code files are present in their respective code folders.

The code will look like the following:
```
let s = "Hello".to_string();
println!("s: {}", s);
let s = String::from("Hello");
println!("s: {}", s);
```

This book has been written with and tested for the Rust versions rustc 1.24.1 and rustc 1.26.0-nightly; however, Rust's strong backward compatibility should make it possible for you to use any newer versions for all chapters except the last. Chapter 10, Using Experimental Nightly Features, is working with cutting-edge technology that is expected to improve through ground-breaking changes.

To download the newest Rust version, visit https://rustup.rs/, where you will be able to download a Rust installer for your operating system. It's okay to leave it at the standard settings. Make sure to call rustup default nightly before starting Chapter 10, Using Experimental Nightly Features. Don't worry, you'll be reminded again when it's time.

## Building the code with a modern Rust toolchain

The repository has been updated to build with modern stable Rust (Rust 2024 edition, tested on rustc 1.97.1). The following changes were applied to keep the original examples compiling:

- Each chapter now declares `edition = "2024"` in its `Cargo.toml`.
- Deprecated APIs have been replaced with their current equivalents (for example, `trim_left`/`trim_right` → `trim_start`/`trim_end`, `compare_and_swap` → `compare_exchange`, `Box<Error>` → `Box<dyn Error>`, bare trait objects → `dyn` trait objects).
- Old crate versions have been bumped where necessary (`serde`, `reqwest`, `csv`, `lazy_static`, `bitflags`, `regex`, and others).
- Chapter 9 now uses `reqwest` with the `rustls-tls` backend to avoid depending on an OpenSSL version that is no longer available on many systems.
- Chapter 10 no longer requires nightly. The unstable `Generator`/`yield` example was rewritten as a stable state machine, the benchmark harness was replaced with a simple timing `main`, and stabilized `#![feature(...)]` attributes were removed.

An active internet connection is required for many recipes, as we will work intensively with crates. These are Rust's way of distributing libraries over the internet, and they are hosted at https://crates.io/.

You might wonder why a book about Rust's standard library, or std for short, uses so much code from outside the std. That's because Rust, in contrast to most other system languages, was designed with strong dependency management in mind from the beginning. It's so easy to pull crates into your code that a lot of specific functionality has been outsourced to officially recommended crates. This helps the core standard library that is distributed with Rust to stay simple and very stable.

The most official group of crates after the std is the nursery (https://github.com/rust-lang-nursery?language=rust). These crates are the standard for many operations and are nearly stable or generic enough to be in the std.

If we can't find a crate for a recipe in the nursery, we look at the crates of the Rust core team members (https://github.com/orgs/rust-lang/people), who put a lot of effort into providing functionality that is missing from the standard library. These crates are not in the nursery because they are usually specific enough that it is not worth allocating too many resources to actively maintaining them.

All the code in this book has been formatted with the newest rustfmt (rustfmt-nightly v0.4.0), which you can optionally download using rustup component add rustfmt-preview and run with cargo fmt. The code on GitHub (https://github.com/jnferner/rust-standard-library-cookbook) is going to be actively maintained and consequently formatted using a newer version of rustfmt, if available. In some cases, this means that the source code line markings can become outdated. It should not be hard to find the code, however, as this shift is usually no greater than two or three lines.

All code has also been checked by Rust's official linter, clippy (https://github.com/rust-lang-nursery/rust-clippy), using version 0.0.187. If you want, you can install it with cargo +nightly install clippy and run it with cargo +nightly clippy. The newest version tends to break quite often though, so don't be surprised if it doesn't work outright.

Some clippy and rustc warnings have been left in the code intentionally. Most of these are either dead code, which happens when we assign a value to a variable to illustrate a concept and then don't need to use the variable anymore, or usage of placeholder names such as foo, bar, or baz, which are used when the exact purpose of a variable is irrelevant to the recipe.

## Related Products
* [Rust High Performance](https://www.packtpub.com/application-development/rust-high-performance?utm_source=github&utm_medium=repository&utm_campaign=9781788399487)

* [Rust Programming By Example](https://www.packtpub.com/application-development/rust-programming-example?utm_source=github&utm_medium=repository&utm_campaign=9781788390637)

* [Learning Rust](https://www.packtpub.com/application-development/learning-rust?utm_source=github&utm_medium=repository&utm_campaign=9781785884306)

### Download a free PDF

 <i>If you have already purchased a print or Kindle version of this book, you can get a DRM-free PDF version at no cost.<br>Simply click on the link to claim your free PDF.</i>
<p align="center"> <a href="https://packt.link/free-ebook/9781788623926">https://packt.link/free-ebook/9781788623926 </a> </p>
