fn main() {
    let names = ("Nemuel", "Peris", "Jedidiah", "Derrick");

    println!("I am {} and she's called {}", names.0, names.1);
    println!("Our younger brothers are {} and {} respectively", names.2, names.3);

    // destructive assignment ... 
    let (a, b, c, d) = names;
    println!("a : {}", a);
    println!("b : {}", b);
    println!("c : {}", c);
    println!("d : {}", d);

    // to be able to modify values in the tuple, make sure to declare it as mutable
}