// Conditionals - Used to check the condition of something then act based on the result
// Like if x is true then do y, else z.

pub fn run() {

    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // if/else
    if age >= 21 && check_id || knows_person_of_age { // If age is equal or more than 21, and it's checked, then do this
        println!("What do u want?");
    } else if age < 21 && check_id { // Else if the age is less then 21, and it's checked
        println!("Get out");
    } else { // Otherwise
        println!("Gibe id");
    }

    // Shorthand If
    let is_of_age = if  age >= 21 { true } else { false }; // Inline
    println!("Is of age: {}", is_of_age) // If age is bigger or equal to 21 then true, else false.

}