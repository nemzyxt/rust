fn main() {
    let x = 10;

    {
        let y = 20;
        // both x and y are accessible in this scope
        println!("The value of x is {} and the value of y is {}", x, y);
    }

    // x is accessible here but y cannot be accessed here !
    println!("The value of x is {}", x); // success

    // the below code generates an error
    // println!("The value of y is {}", y); 

}