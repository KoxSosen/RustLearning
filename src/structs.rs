// Structs - Create custoooooom data types
// Similar to classes.


// Traditional struct. Simple trees!
struct Color {
    red: u8,
        green: u8,
            blue: u8
}

// Tuple Struct
struct Light(u8, u8, u8);

// Playground
struct Person {
    first_name: String,
    second_name: String,
}

// Wow something new! The heck happened here?
// So we implement a function for the Struct Person

// Implement the function
impl Person {
    // Create the func (A new person (Person::new()) and make it return both names to String.
    // It will return a person (->), with first_name, and second_name.
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            second_name: last.to_string()
        }
    }

    // Full name func
    // &self : reference to yourself (Person)
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.second_name)
    }

    // Set last name - mut
    fn set_last_name(&mut self, last: &str) {
        self.first_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.second_name)
    }


}


pub fn run() {

    // Define the values of the struct.
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    // Change value: Mutable
    color.red = 200;

    // Print the values of the struct. structname.member I guess.
    println!("Color: {} {} {}", color.red, color.green, color.blue);

    let mut light = Light(200,100,24);

    light.0 = 100;

    println!("Lights: {} {} {}", light.0, light.1, light.2);


    // Playground

    let mut p = Person::new("Marry", "Doe");
    println!("Person: {} {}", p.first_name, p.second_name);
    println!("Person full name: {}", p.full_name());

    p.set_last_name("Williams");
    println!("Person full name: {}", p.full_name());

    println!("Tuple stuff: {:?}", p.to_tuple());
}