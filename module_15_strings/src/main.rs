/*
   Review of Strings
   A string is a datatype representing a piece of text or a sequence of characters.

   usually in two forms - &str and String. &str is a hardcoded piece of text embedded into the binary executable. We can't modify it, just refer to it.
   String is editable and is a heap-allocated string.

   Recall that strings can't have individual indices accessed in Rust. This is because although one english character corresponds to one byte in memory, that's not necessarily
   the case for emojis. The range syntax is the only way to obtain the individual characters
*/

/*
    Concatenation
    We can add content to the end of a string. For this, we need to be using a heap string
    We can push a character with the ".push()" or a full string with ".push_str()". We can also concatenate using the "+" operator
    Using the "+" operator will take ownership of the first string and move it to the new variable!
*/

/*
    The format! Macro
    This is similar to println but it returns a formatted string. It easily interprets multiple values and returns it
*/

/*
    Common String Methods (trim, casing, replace, split)
*/

/*
    Collecting User Input with read_line Method

*/

/*
    Project
    Define a `make_money` function that accepts a mutable
    String reference. The function should concatenate
    the characters "$$$" to the end of the String.
    Invoke the function in `main`.

    Define a `trim_and_capitalize` function that accepts
    a string slice. It should return a String with
    all whitespace removed and all characters in uppercase.
    Invoke the function in `main`.

    Define an `elements` function that accepts a string
    slice. It should split the string by all occurrences
    of the `!` symbol and return a vector of the string
    slices. Invoke the function in `main`.

    Example:
    elements("Gold!Silver!Platinum")
    => Vector of ["Gold", "Silver", "Platinum"]

    Define a `get_identity` function. The function should
    ask the user for their first and last name in TWO
    steps (i.e., collect user input twice). Make sure
    to communicate the instructions to the user.
    For each Result enum you receive, call the `expect`
    method and provide a custom error message (like
    "Failed to collect first name"). Return a String
    with the first and last names combined. Invoke
    the `get_identity` function in `main`, and output the
    returned String.

    Example:
    fn main() {
      let name = get_identity();
       println!("{name}"); // Bill Murray
    }
*/

use std::io;

fn main() {
    let pirate = "Bloodhook"; // This is a hard-coded string
    let sailor = String::from(pirate); // This is a heap-allocated string we can play around with. It is editable and follows borrowing rules

    let bad_guy = pirate.to_string(); // &str implements the to_string() trait and therefore is able to be used in this fashion

    println!("{pirate} and {sailor} and {bad_guy}");

    let first_initial = &pirate[0..1]; // get character - the second index is non-inclusive
    // Using the push functions
    let mut full_name = String::from("Sylvester");
    let last_name = "Stallone";
    full_name.push(' '); // This is pushing a character
    full_name.push_str(&last_name); // This is pushing a full string. This also changes our heap reference to a stack reference
    println!("{full_name}");
    println!("{last_name}");
    // Using the + operator
    let mut first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    // let full_name = first_name + &last_name; // This performs the "add" method in the background, which takes ownership of "first_name" and gives it to "full_name"
    // println!("{first_name}");
    // println!("{last_name}");

    let icon = format!("{first_name} {last_name}"); // NOTE this doesn't take away our ownership, it automatically borrows behind the scenes. 
    let icon = format!("{} {}", first_name, last_name); // Another acceptable way
    let icon = format!("{0} {1} {0}", first_name, last_name); // Another acceptable way
    println!("{icon}");
    println!("{first_name}");
    println!("{last_name}");

    let mut music_genres = "    Rock, Metal, Country, Rap      ";
    println!("{}", music_genres.trim());
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("a", "@"));

    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genres);

    let mut name = String::new();
    println!("What is your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to collect input"); // This accounts for errors if there's an issue
    // match io::stdin().read_line(&mut name) {
    //     // We can also handle the error using a match statement
    //     Ok(_) => println!("Hello, {}", name.trim()),
    //     Err(message) => println!("There was an error: {message}"),
    // }
    println!("Hello, {name}");

    println!("Starting Project Content");
    let mut make_money_str = String::from("It's time to make some ");
    make_money(&mut make_money_str);
    println!("{}", make_money_str);

    let input_slice = "Omar Abu-Hijleh";
    let first_name = &input_slice[0..5];
    println!(
        "{}'s first name is {}",
        input_slice,
        trim_and_capitalize(first_name)
    );

    println!("{:?}", elements("Gold!Silver!Platinum"));

    println!("Your name is: {}", get_identity());
}

fn make_money(input_str: &mut String) {
    input_str.push_str("$$$");
}

fn trim_and_capitalize(input_string_slice: &str) -> String {
    return String::from(input_string_slice.trim()).to_uppercase();
}

fn elements(input_string_slice: &str) -> Vec<&str> {
    return input_string_slice.split("!").collect();
}

fn get_identity() -> String {
    let mut first_name = String::new();
    println!("Input your first name: ");
    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to get first name");

    let mut last_name = String::new();
    println!("Input your last name: ");
    io::stdin()
        .read_line(&mut last_name)
        .expect("Failed to get last_name");

    return format!("{} {}", first_name.trim(), last_name.trim());
}
