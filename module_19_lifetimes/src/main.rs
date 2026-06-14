/*
 * Concrete Lifetimes for Values
 *
 * Lifetimes are one of Rust's most difficult features. The term refers to different ideas
 *
 * A lifetime refers to how long something is alive (useful, valid, usable, etc.)
 *
 * the lifetime of a value refers to how long it is valid within the code (valid = capable of being utilized)
 *
 * A value's lifetime is the time during which it exists at a particular memory address!!!!
 *
 * Lifetimes are often connected to scopes, but a lifetime can end before a scope ends
 */

/*
 * Concrete Lifetimes for References
 */
/*
 * Non Lexical Lifetimes
 * This is a feature that was added over time to help developers. It wasn't in rust originally
 *
 * Lexical => Lasting until the end of the block
 * Non-Lexical => Not lasting until the end of the block
 * If the borrow checker can validate that there is no use of a reference beyond a certain point in the code, then it treats the end of the reference's lifetime as the last place it is used rather than the end of the actual scope or block
 */

/*
 * Invalid Lifetimes
 * The program will not compile if the borrow checker sees an invalid lifetime. That can be caused by a dangling reference - a reference to data that no longer exists
 */

/*
 * Functions Cannot Return References to Owned Values or Parameters
*/

fn main() {
    println!("Concrete Lifetimes for values");
    let a = 1; // NOTE: The lifetime of this variable begins when it is declared. It's lifetime ends when it goes out of scope. the lifetime of the "a" variable is a CONCRETE lifetime. It's very clear where it starts and ends. It's the exact same every time the code executes! Lifetimes become more complex when we're passing references or variables from one function to the next

    // rust supports nested scopes through the use of {} --> This creates it's own scope
    {
        let b = 2; // The lifetime of this variable starts at this line and ends at the next line.
    }
    // NOTE: When Rust transfers ownership of one variable to another, that's when the lifetime of that variable ends! This is an example of the lifetime ending before the scope ends
    let b2 = String::from("Hello");
    let b3 = b2; // The lifetime of b2 ends here since there is a change in ownership

    println!("Concrete Lifetimes for References");
    // NOTE: The reference can't outlive the original value! It must point to a valid value, or else we're going to have dangling pointers
    // The borrow checker is the part of the Rust compiler that validates that all borrows (references) are valid
    let dog = String::from("Watson"); // The lifetime of the string starts at this line and ends when it goes out of scope
    let my_pet = &dog; // The reference is "my_pet" and the referent/lender is "dog". NOTE: This variable's lifetime must be contained within the lifetime of "dog", becaus it's borrowing from "dog"!
    // NOTE: The "my_pet" lifetime ends before the dog because rust cleans these in order

    println!("Non-Lexical Lifetimes");
    println!("{my_pet}"); // This is the last place the "my_pet" reference is used, so the lifetime of the reference ends here instead of the end of the scope

    println!("Invalid Lifetimes");
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona")
    ];
    let favorite_cities = &cities[..2]; // a slice referencing the first two indices of the cities vector
    // let cities2 = cities; // This will fail because we're trying to move ownership in the middle of a borrow, which would otherwise create a dangling reference. The lifetime of the borrow ends later!
    //drop(cities); // NOTE: If we do this, then favorite_cities becomes a dangling reference
    println!("{:?}", favorite_cities);

    // let some_cities = {
    //     let cities1 = vec![
    //     String::from("London"),
    //     String::from("New York"),
    //     String::from("Barcelona")
    //     ];

    //     &cities1[..2] // a slice referencing the first two indices of the cities vector is being returned
    // };
    // println!("{:?}", some_cities);


    println!("Functions Cannot Return References to Owned Values or Parameters");
}
