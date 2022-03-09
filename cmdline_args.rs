use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // print out the command line arguments
    for arg in args {
        println!("{}", arg);
    }
}