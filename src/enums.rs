// Enums Definite values

enum Directons {
    // Variants
    Up, // A value inside Directons
    Down,
    Left,
    Right
}

// A new function. d: is Directions
fn move_around(d: Directons) {
    // Preform action depending on info

    // Java switch - Similar to that.
    // In case d is matched then
    match d {

        // If we're moving up then do xyz
        // => is used to say "do xyz"
        // Func => Stuff to do
        Directons::Up => println!("Up"),
        Directons::Down => println!("Down"),
        Directons::Left => println!("Left"),
        Directons::Right => println!("Right")

    }
}


pub fn run() {

    // Create objects and define their type by using the Enum.
    let upmovingguy = Directons::Up;
    let downmovinguy = Directons::Down;
    let rightmovinguy = Directons::Right;
    let leftmovingguy = Directons::Left;

    // Call the function we created earlier with the objects.
    // As per they have a variable type deined from the enum, the function will call
    // the corresponding method.
    move_around(upmovingguy);
    move_around(downmovinguy);
    move_around(rightmovinguy);
    move_around(leftmovingguy);

}