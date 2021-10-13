// Immutable vars, using let to create variables

pub fn run() {

    // Create var
    let name= "Brad";
    let mut age = 38; // Ths is mutable - You can assign or change the variable again.

    age = 20; // - We get a warning. Because we haven't used the old variable yet.

    // let age = 37; - This is innmutable. You can't change it.
    // Reassign
    // age = 38; - This won't compile. You can't assign twice to an immutable variable.
    // See how we already defined the age variable.


    println!("My name is {}, and I'm {}", name, age);

    // Constants

    const ID: i32 = 001; // When defining a constant, you must specify the data type.
    println!("ID: {}", ID);

    // Assign multiple vars
    let (test, testwo) = ("yo", 10);
    println!("{} is {}", test, testwo);

}