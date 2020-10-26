fn main() {
    
    struct Dog {
        name: String,
        age: u32
    }

    let kiki=Dog { name: String::from("Kiki"), age: 5};

    println!("{} is {} years old",kiki.name, kiki.age)

}
