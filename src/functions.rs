// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("garett", "peter");

    // Bind functions values to variables
    let get_sum = add(5, 5);
    println!("Sum {}", get_sum);

    // closure - Record for storing a function with an environment.
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32 | n1 + n2 + n3;
    println!("C sum: {}", add_nums(3,3));

}

// This is a basic function with two strings.
// fn stands for Function
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// Add two numbers and return.
// -> is used to return data.
// Notice how there is no ;
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}