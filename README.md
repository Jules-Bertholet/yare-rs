# Rust bindings for [Yare.io](https://yare.io/) bots

This crate uses [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) to expose the objects available to [Yare.io](https://yare.io/) bots to Rust.
It's meant to be used with [`yare-rust-template`](https://github.com/Jules-Bertholet/yare-rust-template),
which contains custom build scripts necessary to make `wasm-bindgen` work with Yare.

This crate is documented with [Rustdoc](https://jules-bertholet.github.io/yare-rust/yare/).

## Usage notes

The methods and structs this crate provides map pretty directly to what's available in JS.
This means they won't always be idiomatic Rust. For example, [`Deref`]-based inheritance is ued extensively;
this is [an antipattern](https://github.com/rust-unofficial/patterns/blob/master/anti_patterns/deref.md) for idiomatic Rust
but it's also the best/only way to represent JS inheritance hierarchies, and it's what `wasm-bindgen` uses.

Passing values between WebAssembly and JS is slow, especially when those values aren't numbers.
Generally, any method in this crate that returns a value involves such a transfer of data
(functions that return static references don't).
So be careful, and only retrieve the information you need.

For the reasons mentioned in the previous paragraphs, you may want to create you own structs and data structures to store the information you need.
