fn main() {
    let mut x = 10;

    // immutable reference, can be more than one
    // let xr = &x; 

    // mutable reference, can only be one
    let nem = &mut x;
    println!("The value of x is {}", nem);

    // change the value of x
    *nem += 5; // value of x is now 15
    println!("The new value of x is {}", x);

    /*
     We can only have either one or more immutable references or just one mutable
     reference to a variable in our program !
    */
}