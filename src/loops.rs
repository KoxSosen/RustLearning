// Loops - Itreate

pub fn run() {


    let mut count: i32 = 0;

    // Infinite loop if you remove the contuer
   // loop {
    //    count +=1; // Add one to the count
    //    println!("Number {}", count);
    // }


    // While Loop (FizzBuzz)
    // If it's dividable by 5 and 3 print FizzBuzz
    // If it's dividible by 5 bizz buzz
    // IF it's by 3 print fizz
    while count <= 100 { // If it's more than 100
        if count % 15 == 0 { // If the number is dividable by 15
            println!("FizzBuzz");
        } else if count % 3 == 0 { // If the number is dividable by 3
            println!("Fizz");
        } else if count % 5 == 0 { // If the number is dividable by 5
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        // Increment - Add one after an iteration
        count += 1;

    }

    // For range
    for x in 0..100 { // If it's more than 100
        if x % 15 == 0 { // If the number is dividable by 15
            println!("FizzBuzz");
        } else if x % 3 == 0 { // If the number is dividable by 3
            println!("Fizz");
        } else if x % 5 == 0 { // If the number is dividable by 5
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }



}