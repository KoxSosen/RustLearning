pub fn run() {

    // Print a string
    println!("Hello from the print.rs file");

    // Formatting.. Any variable needs a formatter.
    println!("Number: {}", 1);

    // Hey! is the first value, 1 is the second.
    println!("{} <- First value, second one -> {}", "Hey! ", 1);

    // Order of variables. Brad is 0, test is 1, test is 2 again.
    println!("{0} yes {1} variable {2} ok", "Brad", "test", "test");

    // Looks sketchy but it isn't. We name the variables, then define them.
    // And not inlining smh
    println!("{first} this {tho}",
    first = "hi I'm first",
        tho = "alltho I'm not"
    );

    // Placeholder traits. If I understand this correctly, this is variable transforming?
    // As transforming I mean..Running
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug?
    // The {:?}, or :? means any data type?
    println!("{:?}", (12, true, "string"));
}