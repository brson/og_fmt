#[macro_use]
extern crate og_fmt;

fn main() {
    let msg = fmt!("Original fmt! is the #{} fmt!", 1);
    println!("{}", msg);
}
