// Primitive str = Immutable fixed lenght saved in memory
// String = Growable heap allocated data structure - Use when you need to modify string data.


pub fn run() {

    // This is Primitive - you can't modify it
    let hi = "Hello";

    // This is a String - Made it mutable
    let mut hello = String::from("hi");

    println!("Primitive &str: {}, data structure String: {}", hi , hello);

    // Get length: This works with both strings.
    println!("Length of hi: {0}, Lenght of hello: {1}", hi.len(), hello.len());

    // Add to string.
    hello.push_str(" there");
    // Add a char
    hello.push('p');

    // Byte capacity
    println!("Capacity: {}", hello.capacity());

    // Isempty
    println!("Is empty {}", hello.is_empty());

    // Does it contain x?
    let x: &str = "do you contain me?";
    println!("{}", hello.contains(x));

    // Replace
    println!("Replace {}", hello.replace('p', " not a P anymore!")); // Replacing char with string - You can do this.

    // Loop thru string by whitespace
    for word in hello.split_whitespace() {
        println!("Whitespace: {}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len()); // left == right. a, b = a == b. Passes if it's true, exits if it's false.
    assert_eq!(10, s.capacity()); // We set the capacity to 10 before.

    println!("{}", hello);

}