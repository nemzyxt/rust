// reading from a file in Rust

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("msg.txt").expect("Could not open the file");

    let mut file_content = String::new();

    file.read_to_string(&mut file_content)
        .expect("Could not read from the file!");

    // print out the content
    println!("File Content : {}", file_content);
}