// Ints: The number of bits they take in memory.
// u8, u8 etc
// Floats: f32
// Boolean (bool)
// Characters (char)
// Tuples ?
// Arrays - Vectors

// Not required to set the data type, the compiler can set it, but it's a good practice to set it.
// It meas it's a statically typed lang, I or whatever


pub fn run() {

    // This is i32
    let x = 1;

    // f64
    let y = 2.5;

    // Manually set it
    let u: i64 = 4727428942743232;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max f64: {}", std::i64::MAX);

    // Boolean
    let hi = true;

    println!("{:?}", (x,y,u,hi));

    // If we set specify the data type we can get a true or false out of this
    // We don't have to specify, it works by itself tho
    let is_greater: bool = 10 > 5;

    println!("{}", is_greater);

    // char
    let a1 = 'a'; // Notice that '' is char, "" is string.
    let face: char = '\u{1F600}';

    println!("{0}, {1}", a1, face);

}