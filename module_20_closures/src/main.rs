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

/*
 * Closures with Ownership
 * We can move ownership of a value to a closure by returning that value in the closure logic. The value must be heap-allocated
 */

/*
 * The move Keyword
 * This is a special keyword that moves the ownership of a value to a closure. This is particularly necessary when dealing with multi-threaded programs
 */

/*
 * The unwrap_or_else Method
 * This is similar to unwrap(), but instead of passing in a value we can pass in a closure for the "else" component. The definition forces a trait bound of "FnOnce". If we have a "Some" option, then we get the value contained. If it's a "None", then we invoke the closure
 */

/*
 * Defining a Method that Accepts a Closure
 * We can use trait bounds and pass in a closure to a function in rust
 *
 * The String.retain method - this is an iterator method that goes through each character in a string and applying a closure to it to either keep it or discard it
 */

/*
 * Implementing our own FnMut trait method
 */

/*
 * The Fn Trait
 * This is a subtrait of the FnMut trait. It is the strictest trait and can be invoked multiple times
 */

/*
 * Passing in a Function to Fn Trait Parameter
 * The Fn Trait isn't mandating a closure, but something that can be executed with parenthasis. A function could work too
 */


use std::io::stdin;

fn execute_thrice<F>(procedure: F) where F: Fn() {
    procedure();
    procedure();
    procedure();
}

#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32
}

#[derive(Debug)]
struct Map<'a> {
    locations: &'a [Location]
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where F: FnMut(&Location)
    {
        let final_index = self.locations.len() - 1;
        let mut current_index = 0;
        while current_index <= final_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

 struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock<F>(self, procedure: F)  -> Option<String>
    where F: FnOnce() -> String  // We're setting the closure as a type constraint for the generic. Note that We've also explicitly defined that the return value is supposed to be a string for the input closure
    {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

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
    let mut numbers = vec![4, 8, 15, 16, 28, 32];
    let mut add_number = || {numbers.push(100)}; // NOTE: Invoking a closure is a mutable operation when the closure has a mutable reference. This is because the "state" of the mutable reference is changing, and so the closure is also changing (or re-declaring itself) on each call!

    // println!("{:?}", numbers); // NOTE: This causes a problem because we later call the "add_number" closure. When we have a mutable reference, we can not have other references! The "println" macro is borrowing another reference to the "numbers" variable, which does not work in our case.
    add_number(); // This adds 100 as an element to the end of the "numbers". We now have an "FnMut", indicating that the closure captures a mutable reference
    add_number();

    println!("{:?}", numbers);


    println!("Closures with Ownership");
    let number = 13;
    let capture_number = || number; // This is returning something that implements the copy trait, so there won't be a transfer in ownership for the value "13"
    let a = capture_number();
    let b = capture_number();

    let first_name = String::from("Alice");
    let capture_string = || first_name; // This changes the ownership
    // println!("{first_name}"); // This gives an error because the string is now part of the "capture_string" closure
    let new_owner = capture_string(); // Movement of ownership. We can no longer use the "capture_string" closure again either, because capture_string is no longer the owner and therefore cannot pass the ownership
    println!("{new_owner}");
    // let new_owner_2 = capture_string(); // Doesn't work because the closure is only an "FnOnce", and the "capture_string" moves after the first use


    println!("The \"move\" keyword");
    let first_name = String::from("Alice");
    let last_name = String::from("Wonder");
    let capture_string = move || { // The "move" keyword here forces the ownership of the value of first_name into the closure
        println!("{first_name}");
    };
    capture_string();
    capture_string();
    capture_string();
    // println!("{first_name}"); // This fails because ownership of the variable was moved to the closure

    println!("The unwrap_or_else Method");
    let option = Some("Salami");
    let food = option.unwrap_or_else(|| "Pizza");
    println!("{food}");

    let option = None;
    let food = option.unwrap_or_else(|| "Pizza");
    println!("{food}");


    println!("Defining a Method that Accepts a Closure");
    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold")
    };

    let mut string_value = String::new();
    stdin().read_line(&mut string_value);
    string_value = string_value.trim().to_string();
    let hack = || string_value; // NOTE: We can also make it so that the closure invocation performs the string reading and then we can invoke the trait more than once.
    let extraction = vault.unlock(hack);
    println!("{:?}", extraction);

    println!("The String.retain Method");
    // This implements the FnMut trait
    let mut game_console = String::from("PlayStation");
    let mut deleted_characters = String::new();

    let closure = |character| {
        let is_not_a = character != 'a';
        if is_not_a {
            true
        } else {
            deleted_characters.push(character);
            false
        }
    };
    game_console.retain(closure); // This will remove all the 'a' characters
    println!("{game_console}");
    println!("{deleted_characters}");

    println!("Implementing our own FnMut trait method");
    let locations = [
        Location {
            name: String::from("Enchanted Forest"),
            treasures: 5
        },
        Location {
            name: String::from("Mystic Mountain"),
            treasures: 10
        },
    ];

    let map = Map {
        locations: &locations
    };
    let mut total_treasures = 0;
    map.explore(|location| {
        total_treasures += location.treasures;
    });
    println!("Total Treasures: {total_treasures}");

    println!("The Fn Trait");
    let closure = || println!("I'm the boss here!");
    execute_thrice(closure);

    // This won't work
    // let bosses = vec!["Boris"];
    // let closure = || {
    //     let employees = bosses;
    // };
    // execute_thrice(closure);

    println!("Passing in a Function to Fn Trait Parameter");
    fn bake_cake() {
        println!("Hello Chocolate");
    }

    execute_thrice(bake_cake);

    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new); // We're passing in the new vector creation and the result is a collection
    println!("{:?}", collection);
}
