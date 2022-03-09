fn main() {
    let mut str1 = String::from("Hello friend, I am Nemuel .");
    println!("{}", str1);

    // length of string 
    println!("Length of string : {}", str1.len());

    // split by whitespaces
    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    // check whether string is empty 
    println!("Is string empty ? {}", str1.is_empty());

    // to append another string to the end of an existing one
    str1.push_str("It's a pleasure meeting you !");

    println!("New String : {}", str1);

    // check whether it contains a certain word
    println!("Does the string contain 'Nemuel' ? {}", str1.contains("Nemuel"));
}