/*
    We use the impl keyword to define some functions for a struct
*/

struct Rectangle {
    length: u8,
    width: u8
}

// using impl keyword to define some functions for the struct Rectangle
impl Rectangle {
    fn print_description(&self) {
        println!("R => {} X {}", self.length, self.width);
    }

    fn is_square(&self) -> bool {
        self.length == self.width
    }

    fn calc_area(&self) -> u8 {
        return self.length * self.width;
    }

    fn calc_perimeter(&self) -> u8 {
        return 2 * (self.length + self.width);
    }
}

fn main() {
    let rect = Rectangle {length: 10, width: 5};

    // call function to print description of the rectangle 
    rect.print_description();

    // call the function to check whether or not our rectangle is a square
    println!("Is the rectangle a square ? {}", rect.is_square());

    // call function to calculat and return the area of the rectangle
    println!("Area of the rectangle : {}", rect.calc_area());

    // call function to calculate the perimeter of the rectangle
    println!("Perimeter of the rectangle : {}", rect.calc_perimeter());


}