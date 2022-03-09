fn main() {
    let numbers = 1..11; 

    for n in numbers {
        println!("n is {}", n);
    }

    let names = vec!["Nemuel", "Peris", "Krypto"];
    
    for (index, name) in names.iter().enumerate() {
        println!("The index is {} and the name is {}", index, name);
    }
}