/*
 * Nested Functions
 * A closure is a function without a name (like a lambda in python)
 *
 * they are helpful when we want to declare a one off procedure. It comes from a functional programming paradigm - where you can deal with functions as types.
 *
 * One disadvantage of functions is that they cannot access data outside of their scope.
 */

/*
 * Intro to Closures
 *
 * Closures can capture values from outside their scope. The syntax for a closure is two vertical pipes followed by curly braces, like so:
 * ||{}
 *
 * We define our parameters and annotate their types in the pipes and the procedure to be followed in the curly braces.
 */

/*
 * Closure Shortcuts
 *
 */

/*
 * The Fn Trait Hierarchy
 * The Fn trait is one of 3 traits that indicate a callable type (an invokable procedure). Rust uses the call operator - which is the pair of parenthasis "()". These traits indicate a type that can be called:
 * FnOnce: Closure captures values by "move" (transferring ownership). Closure will be invoked once
 * FnMut: Captures values by "mutable reference". Closure can be invoked multiple times
 * Fn: Closure captures values by "immutable reference" (read-only) or doesn't capture anything at all. Closure can be invoked multiple times.
 *
 * FnMut is a subtrait of FnOnce and Fn is a subtrait of FnMut
 * Fn -> FnMut -> FnOnce
 *
 * These traits are additive. This means we can pass a closure into an argument.
 */

/*
 * Closures that Capture Immutable References
 * A closure can capture a value from the scope it's defined in. A closure can capture values in 3 ways:
 * 1.) It can take ownership
 * 2.) Borrow an immutable reference
 * 3.) Borrow a mutable reference
 *
 * These 3 ways all correspond to the three closure traits of "FnOnce", "FnMut", "Fn"
 */

/*
 * Closures that Capture Mutable References
*/


fn main() {
    println!("Intro to Closures");

    let multiplier = 5;

    let multiply_by = || -> i32 {
        return 5;
    }; // For an inline closure declaration like this, we're unable to declare a type and assign it explicitly. Note that "Fn" is a trait - both closures and functions implement this trait.

    let multiply_value2 = |value: i32| -> i32 {
        return value * multiplier;
    };

    println!("{}", multiply_by()); // This will return "5"
    println!("{}", multiply_value2(5)); // This will return "25"

    let product = |a: i32, b: i32| {
        return a * b;
    };

    println!("{}", product(5, 8));

    println!("Closure Shortcuts");
    // We can omit the "return" and the ";"
    let multiply_by = || -> i32 {
        5
    };
    // We can also omit the "-> i32". If we do, the return value is inferred by the function body
    let multiply_by = || {5};
    // We can also omit the "{}"
    let multiply_by = || 5;


    // When we want to customize the return type, we can tell it what type without using "-> i32"
    let multiply_by = || 5 as u8;

    let multiply_by3 = |value: i32| value * multiplier;

    // If we remove the data type in the input - the return data type is unknown. The compiler can later infer the data type by seeing how the closure is used in it's very first invocation! - This is NOT like a generic!!
    let multiply_by3 = |value| value * multiplier;
    println!("{}", multiply_by3(3));
    // If we want to specify what data type it should infer
    // println!("{}", multiply_by3(3 as u8));


    println!("The Fn Trait Hierarchy");
    println!("Closures that Capture Immutable References");
    let multiplier = 5;
    let multiply_by = |value| value * multiplier; // Here, the closure doesn't take ownership of anything, so it's implementing the "Fn" trait.
    println!("{}", multiply_by(3_u8));
    let numbers = vec![4, 8, 15, 16, 28, 32];
    println!("{:?}", numbers);
    let print_numbers = || {println!("{:?}", numbers)}; // Here, the closure just uses an immutable reference, so it gets the "Fn" trait
    print_numbers();
    print_numbers();
    print_numbers();
    print_numbers();


    println!("Closures that Capture Mutable References");
}
