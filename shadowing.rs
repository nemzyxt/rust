fn main() {
    let x = 10;

    {
        let x = 20;

        println!("The value of x is {}", x);
    }

    println!("The value of x is {}", x);

    let x = "Hello friend, how are you doing today ?";
    println!("x : {}", x);

    let x = 'P';
    println!("The value of x is {}", x);
}