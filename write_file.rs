// writing to a file in Rust

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("hello.txt").expect("Could not open the file");

    file.write_all(b"Good morning bro ! Hope you are enjoying the world of Rust")
        .expect("Could not write to the file");
}