fn main() {
    /*
        The Option Enum

        This models a scenario where the type can be a valid value or nothing at all. The option enum is the Rust solution for this problem. It is something that a program automatically has access to in Rust

        Option::None - Represents the absence of a value.
        Option::Some(T) - Represents the presence of a value of type T.


        NOTE: Rust implements the copy trait on the Option enum, so when you assign an option to another variable, it is copied and not moved. This is different from other types in Rust, which are moved by default.
    */

    let a = Option::Some(5); // Sets the option to i32
    let b = Option::Some("Hello"); // Sets the option to &str
    let c: Option<i16> = Option::Some(5); // Sets the option to i16 (This is how you explicitly set the type of an option)
    let c: Option<i16> = Option::<i16>::Some(5); // Another version of the above one, but with the turbofish operator

    let d: Option<&str> = Option::None; // Sets the option to None, but also indicates that we would have a &str type if there was a value

    // Example 
    let musical_instruments = [String::from("Guitar"), String::from("Piano"), String::from("Drums")]; 
    let first_instrument = musical_instruments.get(0); // This returns an Option<&String> - this is a safe way to get the value at the index location
    println!("The first instrument is {:?}", first_instrument); // Output: Some("Guitar")

    
    let invalid_instrument = musical_instruments.get(100); // This returns an Option<&String> - this is a safe way to get the value at the index location
    println!("The invalid instrument is {:?}", invalid_instrument); // Output: Some("Guitar")

    /*
        The Unwrap and Expect Methods
        The unwrap method is used to extract the value from an Option enum. If the option is None, it will cause the program to panic and terminate.

        The unwrap method is not the most idiomatic way to handle option enums in Rust, but it is useful for quick prototyping and testing.

        The expect method is similar to unwrap, but it allows you to provide a custom error message that will be displayed if the option is None.
    */
    let valid_instrument = first_instrument.unwrap(); // This will successfully extract the value "Guitar"
    println!("The valid instrument is {}", valid_instrument);
    // let invalid_instrument_unwrap = invalid_instrument.unwrap(); // This will cause the program to panic because the option is None
    // let invalid_instrument_expect = invalid_instrument.expect("No instrument found at this index!"); // This will also cause the program to panic, but with a custom error message


    /*
        The Match Keyword with the Option Enum 
        This forces us to declare a match arm for each possible variant of the Option Enum
    */
    safe_get_user_instrument(&musical_instruments, 1); // Valid index
    safe_get_user_instrument(&musical_instruments, 10); // Invalid index


    /*
        Returning an Option Enum from a Function
    */
    let available_item = is_item_in_stock(true, true);
    // let available_item = is_item_in_stock(true, false);
    // let available_item = is_item_in_stock(false, false);
    match available_item {
        Option::Some(true) => println!("The item is in stock."),
        Option::Some(false) => println!("The item is out of stock."),
        Option::None => println!("The item does not exist in the catalog."),
    }

    /*
        Top Level Option Variants

        The Rust prelude is a collection of named constructs that are available automatically in every program (i.e. types, functions, macros, and the Option enum)

        Because they're automatically included, we can actually remove the Option:: prefix when using the Option enum and its variants.
    */

    /*
        The unwrap_or Method 
        This is similar to unwrap, but it mandates an argument as a fallback value. 
    */
    let present_value: Option<i32> = Some(10);
    let missing_value: Option<i32> = None;

    let unwrapped_present = present_value.unwrap_or(0); // This will return 10
    let unwrapped_missing = missing_value.unwrap_or(0); // This will return 0

    println!("Unwrapped present value: {}", unwrapped_present);
    println!("Unwrapped missing value: {}", unwrapped_missing);

    /*
        Building Option from Scratch

        We're going to create our own enum called "MyOption"
    */
    let my_value = MyOption::Some(42); 
    let my_no_value = MyOption::None;
    
    let unwrapped_my_value = my_value.unwrap(); // This will return 42
    println!("Unwrapped MyOption value: {}", unwrapped_my_value);

    // let unwrapped_my_no_value = my_no_value.unwrap(); // This will panic
    let unwrapped_or_my_value = my_value.unwrap_or(7);
    println!("Unwrapped MyOption value: {}", unwrapped_or_my_value);

    let unwrapped_or_my_value = my_no_value.unwrap_or(7);
    println!("Unwrapped MyOption value: {}", unwrapped_or_my_value);

    /*
        The Result Enum
        The Result enum is used for error handling in Rust. It is similar to the Option enum, but it is used to represent either a success or an error.

        The two variants are either Ok or Err. Ok indicates a success and stores an associated piece of data of generic type T while Err indicates an error and stores an associated piece of data of generic type E.

        Rust doesn't have exceptions like other programming languages, so the Result enum is used to handle errors in a more controlled way. Note that the Result enum is also part of the Rust prelude, so we can use it without the Result:: prefix.
    */

    let okay_result: Result<i32, &str> = Result::Ok(100); // This represents a successful result with an i32 value. Notice that we also had to define the data type of the error variant (&str in this case)
    let error_result: Result<i32, &str> = Result::Err("Something went wrong!"); // This represents a successful result with an i32 value. Notice that we also had to define the data type of the error variant (&str in this case)
    println!("Okay result: {:?}", okay_result);
    println!("Error result: {:?}", error_result);

    /*
        Parse Method on a String example of the Result Enum
    */
    let text = "50";
    let parsed_result = text.parse::<i32>(); // NOTE: This function attempts to convert the data type of the string into another data type
    println!("Parsed result: {:?}", parsed_result); // This will return Ok(50)
    
    let text = "Alabama";
    let parsed_result = text.parse::<i32>(); // NOTE: This function attempts to convert the data type of the string into another data type
    println!("Parsed result: {:?}", parsed_result); // This will return Err(ParseIntError { kind: InvalidDigit })

    /*
        Returning a Result Enum from a Function
    */
    let division_result = divide(10.0, 2.0);
    match &division_result {
        Result::Ok(value) => println!("Division result: {}", value),
        Result::Err(err) => println!("Error: {}", err),
    }

    /*
        Result Methods 
        The result enum also has methods like unwrap, expect, and unwrap_or similar to the option enum.
    */
    println!("Division result using unwrap: {}", &division_result.clone().unwrap());
    // println!("Division result using unwrap: {}", divide(30.0, 0).unwrap()); // This will panic
    // println!("Division result using unwrap_or: {}", divide(30.0, 0.0).expect("Invalid denominator ")); // This will give a custom panic message
    println!("Division result using unwrap_or: {}", &division_result.is_ok()); 
    println!("Division result using unwrap_or: {}", &division_result.is_err()); 

    /*
        Nuances of Unwrap Method on Result 
        The unwrap method on result may or may not implement the copy trait depending on the data types used in the Ok and Err variants. If both data types implement the copy trait, then the unwrap method will also implement the copy trait. However, if either data type does not implement the copy trait, then the unwrap method will not implement the copy trait. 
    */
    let my_result = operation(true);
    let content = match my_result { // Because these are heap allocated strings and do not implement the copy trait, ownership of either the Ok or Err variant is moved to the content variable. An easy fix is to borrow the my_result variable instead. We can also change the operation function to return an &str instead, as it implements the copy trait
        Ok(value) => value,
        Err(err) => err,
    }; 
    // println!("unwrap my_result: {}", my_result.unwrap()); // This won't work due to a partial transfer of ownership

    /*
        The while let construct 

        We previously learned about the "if let" construct, which is used to compare a hard-coded value to a dynamic enum variant and then declare a variable to hold the associated data if there is a match.
    */
    let mut sauces = vec!["Ketchup", "Mustard", "Mayonnaise"];

    if let Some(sauce) = sauces.pop() { // If this succeeds, then the "sauce" variable will take the value from the pop()
        println!("I have a sauce: {}", sauce);
    }
    
    if let Some(sauce) = sauces.pop() { // If this succeeds, then the "sauce" variable will take the value from the pop()
        println!("I have a sauce: {}", sauce);
    }

    if let Some(sauce) = sauces.pop() { // If this succeeds, then the "sauce" variable will take the value from the pop()
        println!("I have a sauce: {}", sauce);
    }

    // Instead, we can just use the while let construct to keep popping until there are no more items left
    let mut sauces2 = vec!["Ketchup", "Mustard", "Mayonnaise"];
    while let Some(sauce) = sauces2.pop() {
        println!("I have a sauce: {}", sauce);
    }

    /*Project */
    let infested = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", infested.chef_special());
    println!("{:?}", infested.deliver_burger("123 Elm St"));


    let clean = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    println!("{:?}", clean.chef_special());
    println!("{:?}", clean.deliver_burger("")); // No address
    println!("{:?}", clean.deliver_burger("123 Elm St")); // No address
}

# [derive(Debug)] 
struct Food {
    name: String,
}

# [derive(Debug)] 
struct Restaurant {
    reservations: u32, 
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        } else if self.reservations < 12 {
            return Some(Food{name: String::from("Uno Sashimi")});
        } else {
            return Some(Food{name: String::from("Strip Steak")});
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("Sorry, we have a mice problem"));
        } else if address.is_empty() {
            return Err(String::from("No Delivery Address Specified"));
        } else {
            return Ok(Food{name: String::from("Burger")});
        }
    }
}


fn operation(great_success: bool) -> Result<String, String> {
    if great_success {
        return Ok(String::from("Operation was a great success!"));
    } else {
        return Err("Operation failed.".to_string());
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Result::Err("Cannot divide by zero".to_string());
    }
    Result::Ok(numerator / denominator)
}

# [derive(Debug, Copy, Clone)] // The enum can implement the Debug, Copy, and Clone traits
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Called unwrap on a MyOption::None value"),
        }
    }

    fn unwrap_or(self, default: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => default,
        }
    }
}

fn is_item_in_stock(item_in_system: bool, item_in_stock: bool) -> Option<bool>{
    // Returns Some Boolean containing true if in stock, False if out of stock, or None if the item does not exist in catalog
    if item_in_system && item_in_stock {
        return Option::Some(true); 
    } else if item_in_system {
        return Option::Some(false);
    } else {
        return Option::None;
    }
}

fn safe_get_user_instrument(instruments: &[String], index: usize) {
    let retrieved_instrument = instruments.get(index);
    match retrieved_instrument {
        Option::Some(instrument) => println!("Found instrument: {} at index {}", instrument, index),
        Option::None => println!("Did not find an instrument at index {}", index),
    }
}
