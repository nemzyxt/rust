struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let c = Color {red: 255, green: 70, blue: 40};

    print_color(&c);
}

fn print_color(c: &Color) {
    println!("R: {}, G: {}, B: {}", c.red, c.green, c.blue);
}