// Pointer references - Point to something in memory.

pub fn run() {
    // Primitive array
    // These are primitive values.
    let array1 = [1,2,3];
    let array2 = array1;


    // Array 2 points to array 1, and we print out both (the same) values using the debug thingy.
    println!("Values {:?}", (array1, array2));

    // No primitives - If you assign another variable to them, then the first one won't hold the data anymore.
    // I'll use & to point to the resource.
    // If I understand this correctly, & is used to point to data, or to "mean" the value of data.
    // Eg &array1 = [1,2,3] = [1,2,3] (?)
    println!("Value: {:?}", &array1);

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    // println!("Values: {:?}",(vec1, vec2)); // Won't compile.
    println!("Values: {:?}",(&vec1, vec2)); // &&&&





}