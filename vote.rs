// Tell the user whether or not they are eligible to vote based on their age

fn main() {
    println!("Welcome ... 1\n");

    let age = -5;
    
    if age < 0 {
        println!("YOu are not even born");
    } else if age < 18 {
        println!("YOu are too young!");
    } else {
        println!("You are eligible to vote!");
    }
}