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
 * We use the "+" operator when we want to specify multiple trait bounds for a type
  */

/*
 * where Clauses
 * This is another syntax option we have when we want to implement one or more trait
 *
 */

/*
 * Traits as Function Return Values
 We can annotate the return type of a function to be one of a specific trait
 */

/*
 * Trait Bounds to Conditionally Implement Traits
 * We're going to expand the Hotel struct to accept a generic type T, which we'll also assign as the type for the existing name field
*/

/*
 * A Preview of Trait Objects
 A trait object is an instance of a type that implements a particular trait whose methods will be accessed at runtime using a feature called dynamic dispatch.
 */
/*
 * Trait Must be In Scope to Use it's Definitions
 When it comes to traits, we can't invoke them unless they are defined within the current scope! This is the case for any methods that are the result of a trait implementation.

 Traits can mandate methods on a type and associated functions as well
 */
use std::collections::HashMap;
use std::fmt::Display;

trait Accommodation {
    fn get_description(&self) -> String; // This says that we call this on a variable that implements the get_description method and takes a reference. It returns a string
    fn book(&mut self, name: &str, nights: u32); // A mutable reference to an instance
    fn get_description2(&self) -> String { // The default implementation - this runs unless "get_description2" is implemented by the type
        return String::from("A wonderful place to stay");
    }
}

trait Rating {
    fn get_rating(&self) -> u32 {
        return 0;
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

impl Rating for Hotel {
    fn get_rating(&self) -> u32 {
        return 5;
    }
}

// We're defining a second hotel struct that uses a generic type T - this is so that we can learn how to use trait bounds to conditionally implement methods
#[derive(Debug)]
struct Hotel_2<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel_2<T> { // This form of declaration means that everything defined in this scope works for any generic input of type T. We use the trait bound Display to ensure that T has a display method. This is so there are no conflicts with the Display trait when we try printing! The problem here is that we limit our constructor to the display trait as well. We can easily fix this by using 2 different impl blocks, one for the generic type T and one for the trait bound Display.
    fn new(name: T) -> Self {
        Self {
            name: name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel_2<T> { // This form of declaration means that everything defined in this scope works for any generic input of type T. We use the trait bound Display to ensure that T has a display method. This is so there are no conflicts with the Display trait when we try printing! The problem here is that we limit our constructor to the display trait as well. We can easily fix this by using 2 different impl blocks, one for the generic type T and one for the trait bound Display.
    fn summarize(&self) -> String {
        return format!("{}: {}", self.name, self.get_description());
    }
}


impl<T: Display> Accommodation for Hotel_2<T> {
    fn get_description(&self) -> String {
        return format!("{} is the pinnacle of luxury.", self.name);
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}


impl<T> Rating for Hotel_2<T> {
    fn get_rating(&self) -> u32 {
        return 5;
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

impl Rating for AirBnB {
    fn get_rating(&self) -> u32 {
        return 3;
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
fn mix_and_max(first: &mut (impl Accommodation + Rating), second: &mut impl Accommodation, guest: &str) { // The first parameter must implement both Accomodation and Rating traits
    first.book(guest, 1);
    second.book(guest, 1);
}

// Removes the issue from above, now first and second can be of different types as long as they both implement the Accommodation trait
fn mix_and_max2<T: Accommodation + Rating, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

fn mix_and_max3<T, U>(first: &mut T, second: &mut U, guest: &str)
where // We are using the "where" clause here!
    T: Accommodation + Rating,
    U: Accommodation
{
    first.book(guest, 1);
    second.book(guest, 1);
}

fn choose_best_place_to_stay() -> impl Accommodation {
    return Hotel::new("The Marriott");
}

fn choose_best_place_to_stay2() -> impl Accommodation + Rating { // Implements both traits
    return Hotel::new("The Marriott");
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

    println!("Multiple Trait Bounds");

    println!("Traits as Function Return Values");
    let mut result = choose_best_place_to_stay(); // Note that the result is something that implements the Accommodation trait, not a concrete type. This causes issues with the compiler. The solution is to have "choose_best_place_to_stay" return something that implements both traits, so that the compiler knows it implements both traits and can call methods from both traits on it!
    let mut result2 = choose_best_place_to_stay2();
    // mix_and_max(&mut result, &mut air_bnb, "Sindbad"); // The compiler knows where giving it a type that implement Accommodation, but it doesn't know if it implements Rating as well!
    mix_and_max(&mut result2, &mut air_bnb, "Sindbad");

    println!("Trait Bounds to Conditionally Implement Methods");
    let hotel_2_struct = Hotel_2::new(5); // This is a valid hotel. We're passing in the number 5, and Hotel_2 takes a generic
    println!("{}", hotel_2_struct.summarize()); // Since integers implement the Display trait, this is valid code we can call
    // a hotel that doesn't implement the display trait
    let hotel_2_no_display = Hotel_2::new(AirBnB::new("Fayrouz")); // Here we see that we can pass in an AirBnB struct to Hotel_2, but it doesn't implement the Display trait, so we can't call summarize on it
    // let hotel_2_no_display_summarized = hotel_2_no_display.summarize(); // This will not compile because the AirBnB struct does not implement the Display trait

    println!("A Preview of Trait Objects");
    let hotel = Hotel::new("The Luxe");
    let airbnb = AirBnB::new("Peter");
    // Dynamic dispatch example below. Static dispatch is the default, but dynamic dispatch allows us to call methods on trait objects at runtime. Dynamic dispatch is slower but more flexible
    // we're using the "dyn" keyword to indicate we're using a trait object. Also, the "&" indicates we're using references in memory!
    let stays: Vec<&dyn Accommodation> = vec![&hotel, &airbnb]; // This yields an error because Rust expects all elements in a vector to have the same type. Rust makes the inference based on the first element of the vector
    println!("{}", stays[0].get_description()); // This is an example of where the actual dynamic dispatch happens
    println!("{}", stays[1].get_description());

    println!("Trait Must be In Scope to Use it's Definitions");

}
