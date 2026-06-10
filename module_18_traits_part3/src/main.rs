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
 We can define equality between two different types
 */

/*
 * Implementing the PartialEq Trait for Enums
 *
 * Two enum results are equal when implementing the PartialEq trait when they have the same enum value and associated data value!
 */

/*
 * Implementing the Eq Trait
 * there's an additional trait called the "eq" trait
 * The eq trait is a subtrait of the PartialEq super trait
 * The eq trait doesn't require any methods. It declares that three additional principles will apply:
 * 1.) reflexive: a == a;
 * 2.) symmetric: a == b implies b == a (required by PartialEq as well);
 * 3.) transitive: a == b and b == c implies a == c (required by PartialEq as well)
 *
 * the reason the rust team declared 2 separate traits is because there are exceptions a type will implement only the partialEq. For example f32 and f64.
 * NaN is techincally a float of type f64, for example. And an NaN cannot equal an NaN
 */

/*
 * Implementing the PartialOrd trait
 We can impliement this trait to indicate that a type can be ordered/sorted
 It's included in the rust prelude. We can also use it to define logic for different types, not just same type scenarios
 This is a sub trait of the PartialEq supertrait --> If a type chooses to implement PartialOrd it must also implement PartialEq
 */

/*
 * Associated Types
 This is a placeholder is like a generic in spirit in the sense that it's a placeholder for a future concrete type but it's coupled to a trait definition

 Whenever we use a trait with an associated type, we need to provide a concrete type for that associated type
 */


use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Drop; // May be in the prelude but we'll do this anyway
use std::clone::Clone; // This is in the rust prelude but we're including it anyway
use std::cmp::Ordering; // cmp is the "compare" module
use std::ops::Add; // ops is the "operations" library

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    // This function is operating on a Generic that implements the Add trait, where the Add trait returns the generic type T as it's associated output type
    return a+b;  // This will not compile because the compiler cannot guarantee that a and b are types that can be added together, unless we add a trait constraint for the generic to implement the Add trait! On it's own, the Add trait is insufficient though, because the definition of the trait indicates that it returns a Self::Output type - and so it needs to know what the return type will be!
}
#[derive(Debug)]
struct Lunch {
    cost: f64,
}

impl Add for Lunch {
    type Output = f64; //Our associated type
    // An alternative implementation if we try to change the output to be a lunch struct with the new total cost instead
    // type Output = Lunch;
    // fn add(self, rhs: Self) -> Self {
    //     Self {
    //         cost: self.cost + rhs.cost
    //     }
    // }
    fn add(self, rhs: Self) -> Self::Output { // Doing things this ways sets the Add implementation to return a f64 if we add two lunch structs together
        self.cost + rhs.cost
    }
}


struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Instead of the below code, we could just do self.salary.partial_cmp(&other.salary) because u32 data types implement the PartialCmp trait!
        if self.salary == other.salary {
            Some(Ordering::Equal)
        } else if self.salary < other.salary {
            Some(Ordering::Less)
        } else if self.salary > other.salary {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

#[derive(PartialEq)]
enum Musician {
    SingerSongWriter(String),
    Band(u32),
}
use Musician::{Band, SingerSongWriter}; // This is just so we can reference the enum values directly instead of using the Musician prefix

// impl PartialEq for Musician {
//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             SingerSongWriter(name) => match other {
//                 SingerSongWriter(other_name) => name == other_name,
//                 Band(_) => false
//             },
//             Band(members) => match other {
//                 SingerSongWriter(_) => false,
//                 Band(other_members) => members == other_members,
//             },
//         }
//     }
// }


struct BusTrip {
    origin: String,
    destination: String,
    time: String
}

impl BusTrip{
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string()
        }
    }
}

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

impl PartialEq<BusTrip> for Flight { // This is the definition of partial eq between a flight and a bus trip!
        fn eq(&self, other: &BusTrip) -> bool {
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

    println!("Defining Equality for Different Types");
    let bus_trip = BusTrip::new("New York", "London", "10:30 AM");
    println!("Flight and Bus Trip are equal? --> {}", flight1.eq(&bus_trip)); // We can do this because we defined the equality for flights and bus trips by having the same origin and destination!
    // NOTE: we can't do bus_trip.eq(&flight1) because we never defined it as a BusTrip function implementation of the trait

    println!("Implementing the PartialEq Trait for Enums");
    let rustin_beiber = SingerSongWriter("Rustin".to_string());
    let rustin_timberlake = SingerSongWriter("Rustin".to_string());
    let holly = SingerSongWriter("Holly".to_string());

    let backstreet_boys = Band(32);
    let jane_street = Band(32);
    let one_direction = Band(5);

    println!("Rustin Bieber and Holly --> {}", rustin_beiber.eq(&holly));
    println!("Rustin Beiber and Rustin Timberlake --> {}", rustin_beiber.eq(&rustin_timberlake));
    println!("Backstreet Boys and Jane Street --> {}", backstreet_boys.eq(&jane_street));
    println!("One Direction and Backstreet Boys --> {}", one_direction.eq(&backstreet_boys));

    println!("Implementing the PartialOrd Trait");
    let long_commute_job = Job{salary: 100000, commute_time: 2};
    let short_commute_job = Job{salary: 75000, commute_time: 2};

    println!("{}", long_commute_job > short_commute_job);
    println!("{}", long_commute_job == short_commute_job);
    println!("{}", long_commute_job < short_commute_job); // You can also use methods --> lt, le, gt, ge

    println!("Associated Types");
    let lunch1 = Lunch{cost: 10.0};
    let lunch2 = Lunch{cost: 5.0};
    println!("{}", lunch1 + lunch2); // this now works for our custom type

    println!("{}", add_two_numbers(1.5, 2.4));
    println!("{}", add_two_numbers(1, 2));


}
