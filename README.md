# `og_fmt` — Will the real `format!` please stand up?

## **O.G. Rust Series™ — Rust as it really was**

### 💯**%** **True Rust** &middot; Accept No Substitutes 💯**%**

`og_fmt` brings back the legendary `fmt!` macro! [Removed from Rust in
2013](https://github.com/rust-lang/rust/pull/8637#issuecomment-23191833),
`fmt!` is a more primal interpretation of the Rust string formatting
paradigm. Some say that the renaming of `fmt!` to `format!` was the
single largest mistake in the history of Rust. We'll never know what
might have been, but we can still recapture a bit of that original
formatting feeling.

Behold! `fmt!`

### Example

```rust
#[macro_use]
extern crate og_fmt;

fn main() {
    let msg = fmt!("Original fmt! is the #{} fmt!", 1);
    println!("{}", msg);
}
```

## About O.G. Rust Series™

_O.G. Rust Series™ — Rust as it really was_, restores Rust to its
original design, one crate at a time. O.G. Rust Series™ crates are
derived from the original source code of historical revisions of the
Rust compiler, and provide the most authentic Rust experience
available today.

Enjoy other fine crates in O.G Rust Series™ and share your
favorites #OGRust

## License

MIT / Apache-2.0

Rust Libs Team, you villains — bring O.G. `fmt!` back home to std.
