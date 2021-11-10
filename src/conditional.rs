// Conditionals - Used to check the condition of something
// and act on the result

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // If else
    if age >= 18 && check_id || knows_person_of_age {
        println!("Police: You can go");
    } else if age < 18 && check_id {
        println!("Police: You are not allowed to drive.");
    } else {
        println!("Police: Show me your Id");
    }

    // Shorthand if
    let is_of_age: bool = if age >= 18 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}
