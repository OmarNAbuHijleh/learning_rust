/*
 * Intro to Traits
 * A trait is a distinguishing quality or characteristic. It serves to identify something.
 *
 * Different things can have the same trait even if they are different entities.
 *
 * A trait is a contract in rust that describes functionality that a type should have
 * We use the word "implement" to describe when a type honors a trait's requirements
 * A trait definition declares the method(s) that a type implementing that trait must have
 * - Method name, parameter types, and return type
 *
 * For example: the "Display" and "Debug" traits require a type to define methods for presenting itself a string
 * The clone trait requires a type to define a "clone" method for creating a duplicate of itself
 *
 * Once we have defined a trait, we can implement it on structs and enums. The type promises to honor the trait's requirements. Mulitple types can implement the same trait, and a type can implement
 * multiple traits.
 */

/*
 * Defining a Trait
 use the "trait" keyword and define functions
 */

/*
 * Implementing a Trait for Structs
 */

/*
 * Default Implementations
 */
use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String; // This says that we call this on a variable that implements the get_description method and takes a reference. It returns a string
    fn book(&mut self, name: &str, nights: u32); // A mutable reference to an instance

}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        return format!("{} is the pinnacle of luxury.", self.name);
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

# [derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)> // String is the guest name and u32 is the number of nights they are staying
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: Vec::<(String, u32)>::new(),
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        return format!("{} is a popular AirBnB host.", self.host);
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

fn main() {
    println!("Implementing a trait for structs");

    let mut air_bnb = AirBnB::new("Omar's AirBnB");
    let mut hotel = Hotel::new("Abu-Hijleh Inn & Suits");

    println!("{}", air_bnb.get_description());
    println!("{}", hotel.get_description());

    air_bnb.book("Jasmine", 7);
    hotel.book("Aladdin", 8);

    println!("{:?}", air_bnb);
    println!("{:?}", hotel);

    println!("Default Implementations");
}
