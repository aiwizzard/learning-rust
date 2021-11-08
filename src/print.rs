pub fn run() {
    // Print to console
    println!("Hello from print.rs");

    // Printing a number
    println!("Number: {}", 1);

    // Basic formatting
    println!("{} is from {}", "ajmal", "vadakara");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} loves to {2}",
        "Ajmal", "Vadakara", "code"
    );

    // Named arguments
    println!(
        "{name} loves {activity}",
        name = "Ajmal",
        activity = "Coding"
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    println!("Binary: {0:b}, Hex: {0:x}, Octal: {0:o}", 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "ajmal"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
