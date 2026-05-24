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
 You can define the default implementation of a trait method - the implementation we write is the one that will be implemented UNLESS the type implements the method itself. This makes the default a fallback
 */

/*
 * Calling Trait Method from Another Method
 * We can call trait methods from within other methods
 */

/*
 * Traits for Function Parameter Constraints
 We can use a trait to add constraints to a function parameter.
 */

/*
 * Trait Bound Syntax
 * The design of taking something that implements the trait as a parameter should remind us of another topic we covered - generics. They solve similar problems.
 * With generics, we define a generic type that is a stand-in for a future concrete type and then we can use the generic within our function declaration to annotate the type of parameters and return values to be of that type
 *
 * The syntax we currently have is called syntactic sugar - which means it's a shortcut for a longer, alternative declaration form that uses generics. That alternative declaration form is called a trait bound.
 * The trait bound syntax uses a generic but limits the generic type to only types that implement a trait.
 */

/*
 * Multiple Trait Bounds
 */

use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String; // This says that we call this on a variable that implements the get_description method and takes a reference. It returns a string
    fn book(&mut self, name: &str, nights: u32); // A mutable reference to an instance
    fn get_description2(&self) -> String { // The default implementation - this runs unless "get_description2" is implemented by the type
        return String::from("A wonderful place to stay");
    }
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

    fn summarize(&self) -> String {
        return format!("{}: {}", self.name, self.get_description());
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

fn book_for_five_nights(entity: &mut impl Accommodation, guest: &str) { // Takes a reference to any type that has implements the accommodation trait!
    println!("{}", entity.get_description());
    entity.book(guest, 5);
}

// Reminder - the <T> is a generic type parameter. The ": Accommodation" is the trait bound, limiting us to only use types that implement the Accommodation trait.
fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    println!("{}", entity.get_description());
    entity.book(guest, 1);
}

// If we do this, both have to be of trait Accommodation but they don't have to be the same type. if we switched to using the trait bound syntax, we have to use the same type for both!!!!
fn mix_and_max(first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

// Removes the issue from above, now first and second can be of different types as long as they both implement the Accommodation trait
fn mix_and_max2<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
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
    println!("{}", air_bnb.get_description2());
    println!("{}", hotel.get_description2());

    println!("Calling Trait Method from Another Method");
    println!("{}", hotel.summarize());

    println!("Traits for Funciton Parameter Constraints");
    book_for_five_nights(&mut hotel, "Omar");
    book_for_five_nights(&mut air_bnb, "Ali");
    println!("{:?}", hotel);
    println!("{:?}", air_bnb);

    println!("Trait Bound Syntax");
    book_for_one_night(&mut hotel, "Inam"); // Notice how this runs in the same manner as before, but it uses the trait bound syntax to limit the type to only those that implement the Accommodation trait
    book_for_one_night(&mut air_bnb, "Eman");
    println!("{:?}", hotel);
    println!("{:?}", air_bnb);

    println!("Multiple Trait Bounds")

}
