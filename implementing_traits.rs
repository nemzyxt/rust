struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("Hello friend, my name is {} and I am {}", self.name, self.age);
    }
}

fn main() {
    let p1 = Person {name: "Nemuel".to_string(), age: 18};

    println!("{}", p1.to_string());
}