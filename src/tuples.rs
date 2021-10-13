// Tuples - Interesting data type. Haven't used tuples at all before.
// It groups together values of different types. !! Max 12 elements !!

pub fn run() {

    // This is a tuple.
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    // Calling tuples
    println!("{} - First tuple part, {} - is second value, this is the 3rd, which is a i8 {}", person.0, person.1, person.2);

}