// Vectors - Resizeable arrays


pub fn run() {

    // Instead of this, which is a mutable array:
    // let mut vektor: [i32; 5] = [1,2,3,4,5];
    // We do:
    // Variable - Data type - Vector elements
    let mut vektor: Vec<i32> = vec![1,2,3,4,5];

    // Special stuff why you should use a Vector
    // Add to the Vector
    vektor.push(6);

    println!("Vec lenght before pop: {}", vektor.len());

    // Pop off the last value
    // Remove last value smh
    vektor.pop();

    println!("Vec lenght after: {}", vektor.len());

    // Loop thru vector values
    for x in vektor.iter() {
        println!("Vec elements interated thru: {}", x);
    }

    // Loop & mutate

    for y in vektor.iter_mut() {
        *y *= 2; // Multiply by two. Similar to map.
    }
    println!("2 - Numbers Vec: {:?}", vektor);
}