#![allow(unused_variables)] // If we do this, we're applying the compiler directive to the entire file!

//Constant definitions
const TAX_RATE: f64 = 0.3; //Note that constants REQUIRE a specific type annotation
type Meters = i32; // we're assigning an alias to a type
const TOUCHDOWN_POINTS: i32 = 6;

#[allow(unused_variables)] //This compiler directive will prevent warnings for unused variables across the entire function definition!
fn main() {
    // 
    let apples = 50;
    let oranges = 14 + 6; 
    let fruits = apples + oranges;

    println!("My Garden has {fruits} fruits"); //Can do this
    println!("My Garden has {} fruits", fruits); //Can do this
    println!("My Garden has {} fruits, of which {} are oranges", fruits, oranges); //Can do this
    println!("My Garden has {0} fruits, of which {1} are oranges", fruits, oranges); //Can do this
    println!("My Garden has {0} fruits, of which {0} are oranges {1}", fruits, oranges); //Can do this, where digits represent numerical arguments passed



    // Variables are immutable by default in Rust

    //To make an immutable variable:
    let mut name = "Omar"; // The "mut" keyword is how we create a mutable variable! We can't change the variable's type though!
    println!("{name}");
    name = "Ali";
    println!("{name}");

    //Variable shadowing - this is when we re-declare a variable to assign a new value to it
    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345; // We have to re-initialize entirely to give it the new value with the new type!
    let mut grams_of_protein = 100;
    grams_of_protein = 105;
    println!("{grams_of_protein}");

    //A scope is the boundary or region of code where a name is valid
    {
        let random_var = 32;
        println!("{random_var}"); //this works
    }
    // println!("{random_var}"); // This doesn't work

    //Constants - a value assigned to a variable. That variable's value can never change!
    /*What is the difference between a constant and an immutable variable? Well a constant can be declared at any level whereas a variable
        is only ever limited to a scope. A constant can be re-used across multiple functions!

        ANOTHER MAJOR DETAIL - A constant's value is known at compile time whereas a variable's value is known at runtime!
     */
    println!("The tax rate is {TAX_RATE}");


    /*
        A type alias is an alternate name that we can assign to an existing type.
        The benefit is that we can provide additional context on what a type represents
    */
    let mile_race_length: Meters = 1600; // The "meters" type we created above our main program is something we can reference, letting Rust know that it's an i32 by default
    let two_mile_race_length: Meters = 3200;
    println!("A mile race is {mile_race_length} meters long and a 2 mile race is {two_mile_race_length} meters long");



    /*
        Compiler Directives:
            A compiler directive is an annotation that tells the compiler how to parse the source code
            We can apply this to individual lines, functions, or entire Rust files. We write the directive above the line that we're using
    */
    #[allow(unused_variables)] //This is an example of a compiler directive. Notice that the unused "mile_race_length" variable warning is surpressed now!
    let mile_race_length: Meters = 1600; // The "meters" type we created above our main program is something we can reference, letting Rust know that it's an i32 by default
    let two_mile_race_length: Meters = 3200;


    // Mini project:
    let season: &str = "Summer";
    let mut points_scored: i32 = 28;
    points_scored = 35;
    let event_time = "06:00";
    let event_time = 6;
    println!("Project printout");
    println!("{season}, {points_scored}, {event_time}");
    println!("{0}, {1}, {2}", season, points_scored, event_time);

    let _favorite_beverage: &str = "Water"; //Can silence compiler warnings about unused variables like this!



}
