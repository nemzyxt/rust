fn main() {
    let mut n = 1;

    loop {
        println!("The value of n is {}", n);
        n += 1;
        
        if n > 10 {
            break;
        }
    }
}