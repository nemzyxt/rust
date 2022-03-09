// Print out the prime numbers from 1 to 100 

fn main() {
    for n in 1..101 {
        if is_prime(n) {
            println!("{} is Prime", n);
        } else {
            continue;
        }
    }
}

fn is_prime(n: u8) -> bool {
    let mut factors = 0;

    for i in 1..n + 1 {
        if n % i == 0 {
            factors += 1;
        }
    }

    if factors == 2 {
        return true;
    } else {
        return false;
    }
}