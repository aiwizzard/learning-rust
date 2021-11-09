// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is block-scoped lanquage

pub fn run() {
    let name = "Ajmal";
    let mut age = 23;
    println!("My name is {} and I am {}", name, age);
    age = 24;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Ajmal", 24);
    println!("{} is {}", my_name, my_age);
}
