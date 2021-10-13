// Arrays are fixed lists where elements are the same data types
// array = a,b,c,d

// Import mem lib
use std::mem;

pub fn run() {


    // Mutable Array list. - Using []'s. First we define the data type, then the number of elements.
    // The elements must be the same data type, because we already defined them in the first scope.
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    // Get an element !! Same with any other lang: 0 is actually the first one !!
    println!("Single val {}", numbers[3]);

    // Re-assign an element.
    numbers[2] = 20;
    println!("Changed: {:?}", numbers);

    // Get the whole array length - The number of elements basically
    println!("Array lenght: {}", numbers.len());

    // Memory amount - Arrays stack allocated
    println!("Array occupies {0} bytes", mem::size_of_val(&numbers));

    // Get slices of array - The first two as an example. Tricky
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

}