/*
 * Associated Constants in a Trait
 * This is a constant that lives within a trait (fixed, and immutable)
 * We can override the default value in a struct implementation!!!!!
 */

 /*
 * Getters in Traits
 */

/*
 * Setters in Traits
 */

/*
 * Supertraits 1 (Trait Inheritance)
 A supertrait is a trait from which another trait inherits functionality. The parent is called "supertrait" and the child is called "subtrait"

 Anything implementing a subtrait must also implement the contents of the supertrait (unless there's a default implementation in the supertrait)
 */

/*
 * Traits with Generics
 *
 */

/*
 * Implementing the Display Trait on a Struct
 */

/*
 * Implementing the Display Trait on an Enum
 *
 */

/*
 * Implementing the Debug Trait

 */

/*
 * Formatter Methods
 This is another way we can deal with returning our display or debug
 */

/*
 * Implementing the Drop Trait
 Many rust types allocate memory on the heap and the owner is responsible for deallocating when going out of scope. This is the "Drop" trait that handles the memory deallocation

 For instance, say we have an operation that writes a file when creating a struct. When the variable goes out of scope, we may then want to delete the file that was written while deallocating the variable from memory.
 This would be an instance where we want to take the struct and implement the "drop" trait in a custom manner
 */

/*
 * The Clone Trait
 The clone trait indicates that we can make a duplicate of a type. It makes a full duplicated copy

 NOTE: Instead of going through and defining the clone implementation in the manner we did, we can simply derive the trait the way we do the debug trait!
 */

/*
 * Implementing the Copy Trait
 The copy trait is a subtrait of the clone supertrait. If a type chooses to implement the copy trait, we don't need to define any additional methods. If a type chooses to implement "Copy" it must implement the "Clone" supertrait.
  */

/*
 * Implementing the PartialEq trait for Structs
 This trait establishes equality between two values. It let's us apply operators like the "==" and "!="
 By default the trait assumes the values being compared are the same type, but it's actually more flexible than that. It' also included in the Rust prelude
 There's also the "eq" method, which does the same thing as the "==" operator. Implementing "eq" automatically establishes the functionality of "ne" (not equal)

 Just like with other traits, we can use the "derive" attribute. If we do, it'll then check all of the fields and if they are the same value!
 */

/*
 * Defining Equality for Different Types
 */

use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Drop; // May be in the prelude but we'll do this anyway
use std::clone::Clone; // This is in the rust prelude but we're including it anyway

// #[derive(PartialEq)]
struct Flight {
    origin: String,
    destination: String,
    time: String
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string()
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        // We're going to define a flight with the same origin and destination as being equal (so ignoring the time)
        self.origin == other.origin && self.destination == other.destination
    }
}



# [derive(Debug, Clone)] // Here, "Duration" is implementing the "Clone" supertrait
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32
}

impl Copy for Duration {}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self { // This is an example where the clone gives back a new data stored on the stack, because it's sub-components are stack data types
        Self {
            hours: hours,
            minutes: minutes,
            seconds: seconds
        }
    }
}

# [derive(Debug, Clone)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String
}

// NOTE: The below code is valid but not necessarily needed. We are using the "Clone" derviation to acheive the same result!
// impl Clone for Appointment {
//     fn clone(&self) -> Self {
//         Self {doctor: self.doctor.clone(), start_time: self.start_time.clone(), end_time: self.end_time.clone()}
//     }
// }

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        return Self { doctor: doctor.to_string(), start_time: start_time.to_string(), end_time: end_time.to_string() };
    }

    fn change_start_time(&mut self, new_time: &str) {
        self.start_time = new_time.to_string();
    }
}

enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "Red Delicious"),
            AppleType::GrannySmith => write!(f, "Granny Smith"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "AppleType::Red Delicious"),
            AppleType::GrannySmith => write!(f, "AppleType::Granny Smith"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Apple Kind: {}\nApple Price: ${:.2}", self.kind, self.price) // This is a macro that writes to the formatter
    }
}

impl Debug for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return f.debug_struct("** Apple ** ").field("Kind", &self.kind).field("Price", &self.price).finish(); // The formatter methods. This is the builder design pattern.
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        println!("Apple is being cleaned up");
    }
}

trait Investment<T> {
    fn get_amount(&self) -> T;

    fn double_amount(&mut self);
}

trait Taxable: Investment<f64> { // Investment is the supertrait and Taxable is the subtrait. We're also specifying the type of T to be f64 for anything that is taxable!
    const TAX_RATE: f64 = 0.25; // This is only accessible via the trait's methods

    fn tax_bill(&self) -> f64 {
        return self.get_amount() * Self::TAX_RATE;
    }
}

#[derive(Debug)]
struct Income {
    amount: f64
}

impl Investment<f64> for Income {
    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
    fn get_amount(&self) -> f64 {
        return self.amount;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    amount: f64
}

impl Investment<f64> for Bonus {
    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
    fn get_amount(&self) -> f64 {
        return self.amount;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5; // Override the default TAX_RATE
}

# [derive(Debug)]
struct QualityTime {
    amount: u32
}

impl Investment<u32> for QualityTime {
    fn double_amount(&mut self) {
        self.amount *= 2;
    }
    fn get_amount(&self) -> u32 {
        return self.amount;
    }
}

fn main() {
    println!("Associated Constants in a Trait");
    let mut income = Income { amount: 80000.00};
    println!("Tax Bill: ${:.2}", income.tax_bill());
    let mut bonus = Bonus { amount: 10000.00};
    println!("Tax Bill: ${:.2}", bonus.tax_bill());

    println!("Getters in Traits");
    println!("Setters in Traits");
    income.double_amount();
    println!("Tax Bill: ${:.2}", income.tax_bill());
    bonus.double_amount();
    println!("Tax Bill: ${:.2}", bonus.tax_bill());

    println!("{:?}", income);
    println!("{:?}", bonus);

    println!("Traits with Generics");
    let mut weekend = QualityTime{amount:10};
    weekend.double_amount();
    println!("{:?}", weekend);

    println!("Implementing the Display Trait on a Struct");
    let apple = Apple { kind: AppleType::GrannySmith, price: 1.00};
    println!("{}", apple); // This will fail unless the struct implements the Display trait

    println!("Implementing the Debug Trait on a Struct");
    println!("{:#?}", apple); // This will fail unless the struct implements the Debug trait


    println!("Formatter Methods");

    println!("Implementing the Drop Trait"); // NOTE: We can see that because we implemented a println as part of the drop statement that at the end of the program when the apple struct we created goes out of scope


    println!("The Clone Trait");
    let morning_appointment = Appointment::new("Dr. Alashwal", "9:00 AM", "10:00 AM");
    let mut cloned_appointment = morning_appointment.clone(); // NOTE the manner in which we created this clone implementation completely decouples the two - we can edit this version without touching the original


    println!("{:#?}", morning_appointment);
    println!("{:#?}", cloned_appointment);

    cloned_appointment.change_start_time("8:30");
    println!("After cloning and changing the times . . . ");
    println!("{:#?}", morning_appointment);
    println!("{:#?}", cloned_appointment);


    println!("Implementing the Copy Trait");
    let one_hour = Duration::new(1, 0, 0); // Hours minutes seconds
    // let another_hour = one_hour; // Here, ownership will move
    // println!("{one_hour}"); // This won't work because ownership has transferred

    let another_hour = one_hour.clone(); // Here, we don't have to worry about any issues with ownership moving. NOTE: We could also just implement the Copy trait and then whenever we use the "=" we'll automatically have movement!
    println!("{:?}", one_hour);

    let yet_another_hour = one_hour; // If we have the Copy trait implemented, problem solved automatically in situations that involve ownership
    println!("{:?}", one_hour);

    println!("Implementing the PartialEq trait for Structs");
    let flight1 = Flight::new("New York", "London", "10:00 AM");
    let flight2 = Flight::new("New York", "London", "10:00 PM");
    let flight3 = Flight::new("New York", "Los Angeles", "10:00 PM");
    // The above 2 flights should be equal because they have the same origin and destination, as we've defined
    println!("Equal flights --> {}", flight1==flight2);
    println!("Equal flights --> {}", flight1==flight3);
    println!("Equal flights --> {}", flight1.eq(&flight3)); // We can also use the "eq" method directly as shown
    println!("Equal flights --> {}", flight1!=flight3); // We can also use the "ne" method directly as shown
    println!("Equal flights --> {}", flight1.ne(&flight3)); // We can also use the "ne" method directly as shown
}
