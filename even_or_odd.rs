// Loop through numbers from 1 to 100. and print whether or not they are even or odd ...

fn main() {
    for n in 1..101 {
        if is_even(n) {
            println!("{} is even", n);
        } else {
            println!("{} is odd", n);
        }
    }
}

fn is_even(n: u8) -> bool {
    return n % 2 == 0;
}