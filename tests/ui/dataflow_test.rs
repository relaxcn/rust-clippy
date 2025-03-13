#![warn(clippy::dataflow_test)]

use std::io::{self, Write};

fn foo() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();  // input is marked as tainted

    let input2 = &input; // data flow tracking - not support now.

    // Violation
    let file = std::fs::File::open(input2).unwrap();
}

fn main() {

}
