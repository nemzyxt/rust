struct Color (u8, u8, u8);

fn main() {
    println!("Welcome ...");

    let red = Color (255, 0, 0);
    println!("{}, {}, {}", red.0, red.1, red.2);
}