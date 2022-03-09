fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];

    // print it out
    println!("Vector : {:?}", nums); // 1, 2, 3, 4, 5

    // append a new item to our vector
    nums.push(6);
    println!("Added the number 6 to the vector");

    println!("Vector : {:?}", nums); // 1, 2, 3, 4, 5, 6

    // remove item at index 3 (4)
    nums.remove(3);
    println!("Removed item at index 3");

    println!("Vector : {:?}", nums); // 1, 2, 3, 5, 6
}