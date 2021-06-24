/*
Michael Scherrer

Special thanks to the Rust programing language book

(for reading stdin over all lines)
https://stackoverflow.com/questions/30186037/how-can-i-read-a-single-line-from-stdin/30186553#30186553

*/
use std::io::{self, BufRead};
mod lib;

fn main() {

    let mut contents = String::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        contents.push_str(&line.unwrap());
        contents.push_str(" ");
    }

    println!("total dom letters: {}",
    lib::dom::get_dom_letters(&contents, false));
}
