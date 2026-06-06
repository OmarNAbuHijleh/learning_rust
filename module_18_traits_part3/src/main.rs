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
 * The Copy Trait
 */

use std::fmt::{Debug, Display, Formatter, Result};
use std::fs; // filesystem module
use std::ops::Drop; // May be in the prelude but we'll do this anyway


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


    println!("The Copy Trait");

}
