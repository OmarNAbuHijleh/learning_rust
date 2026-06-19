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
 * A function cannot return a reference to a value created inside of its body or return a reference to an owned parameter because either of those situations would create a dangling reference
*/

/*
 * References as Function Parameters
 * The lifetime of the referent being passed into a function must last until the function call if we want to return a reference to that data from a function. See "select_first_two_elements_3" for more.
 */

/*
 * Intro to Generic Lifetimes
 * When we say generic, we mean non-specific. When we use "concrete" - we mean actual or non-realized. A generic is a placeholder for a future type. Generics add flexibility by not hardcoding an exact type.
 * Code can use a variety of types in the place of a generic.
 *
 * There are generic lifetimes and concrete lifetimes:
 * A concrete lifetime is an area of the code that a value exists in a program (the time it lives in its memory address)
 * A generic lifetime is more abstract. It is a hypothetical lifetime, a non-specific lifetime, a future lifetime that can vary.
 * We can annotate generic lifetimes in our code. This enables functions that are flexible enough to handle varying lifetimes
 *
 * Lifetime Annotations:
 * A lifetime annotation is a name or a label for a lifetime
 * Lifetime annotations don't change the reference's lifetime. They don't affect the logic in any way
 * A lifetime annotation is a piece of metadata that we provide to the borrow checker so that it can validate that references are valid.
 */

/*
 * Lifetimes and Referents
 * To reiterate from the last lesson, the lifetime syntax doesn't create a coupling between the input reference and the return reference. It creates a coupling between the return reference and the referent (the original data source)
 */

/*
 * Lifetime Elision Rules Part 1
 *
 * Elision: The act of omitting something or leaving it out
 *
 * This refers to omitting generic lifetime annotations in situations where the borrow checker can infer the lifetime relationships automatically.
 *
 * There are 3 rules:
 * 1.) The compiler assigns a lifetime to each parameter that is a REFERENCE
 * 2.) If there is one reference parameter and the return value is a reference, the borrow checker will infer that their lifetimes are related
 *
 */

/*
 * Multiple Parameters Part 1
 */

// This function has no issues
fn create() -> i32 {
    let age = 100;
    return age;
}

// // This function doesn't work because the created variable's reference lasts longer than the lifetime of the refered variable. This issue applies to a function parameter as well
// fn create2() -> &i32 {
//     let age = 100;
//     return &age;
// }

// fn create_slice(items: Vec<i32>) -> &[i32] {
//     &items // notice how this return doesn't work
// }

// // Whenever we invoke this function, because i32 implements the copy trait Rust will copy the value we pass in for the function invocation, and the lifetime of that copy will end - leading to a dangling reference
// // We can, however, return the actual value
// fn create_number_reference(number: i32) -> &i32 {
//     return &number;
// }

fn select_first_two_elements(items: &Vec<String>) {
    let selected_items = &items[..2];
    println!("{:?}", selected_items);
}

// Changing the function to take a reference to a collection of Strings -> This makes it so we can pass in a more dynamic collection (A reference to a vector of strings or a reference to an array of strings)
// Either will work because rust will use deref coersion to coerce either type to a some slice of a collection of strings
fn select_first_two_elements_2(items: &[String]) {
    let selected_items = &items[..2];
    println!("{:?}", selected_items);
}

// NOTE: This will work. Why? Because the lifetime of the variable being passed in to the function extends beyond this function call. Therefore, a pointer to that original variable will still be acceptable as a return (i.e. we're won't have a dangling pointer)
fn select_first_two_elements_3(items: &[String]) -> &[String] {
    &items[..2]
}

// NOTE: Generic lifetimes example. The " ' " (also known as a "tick" symbol) informs the compiler that we're providing an annotation (a generic lifetime). The letter "a" is arbitrary and it's really up to the developer what they decide to provide
// You can have multiple lifetime annotations. So you might have <'a, 'b> for instance. " 'a " is the full lifetime name. We can't just use "a" by itself when referencing the liftime.
// What this is saying: For some generic, hypothetical lifetime 'a, the returned reference must live within the lifetime of the referent that "items" is a reference to because we've also marked "items" with 'a.
// "Whatever the original source of the argument is, the return value must live within it's lifetime"
fn select_first_two_elements_4<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

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

    println!("References as Function Parameters");
    let cities = vec![String::from("Chicago"), String::from("Los Angeles"), String::from("New York")];
    select_first_two_elements(&cities);

    let coffees = [
        String::from("Latte"),
        String::from("Mocha"),
        String::from("Hot Chocolate"),
    ];

    select_first_two_elements_2(&cities);
    select_first_two_elements_2(&coffees);

    let output = select_first_two_elements_3(&coffees);
    println!("{:?}", output);

    println!("Intro to Generic Lifetimes");
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona")
    ];
    let sliced_result = select_first_two_elements_4(&cities);
    // drop(cities); // We're not permitted to do this because sliced_result must live within the lifetime of "cities"
    println!("{:?}", sliced_result);


    println!("Lifetimes and Referents");
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona")
    ];
    let sliced_result = {
        let cities_reference = &cities;
        select_first_two_elements_4(cities_reference) // The lifetime of the return value is coupled to the lifetime of the referent that was passed in (i.e. the original "cities" variable)
    };
    println!("{sliced_result:?}");

    println!("Lifetime Elision Rules Part 1");
    println!("Multiple Paremeters Part 1");

}
