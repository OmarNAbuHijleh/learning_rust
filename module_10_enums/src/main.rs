use std::slice::Windows;

// Module 10: Enums
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades, 
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit 
}

#[derive(Debug)]
struct Credentials{
    username: String, 
    password: String,
}

#[derive(Debug)]
enum PaymentMethodType{
    // CreditCard(String, i32, bool), // The enum variant can store associated pieces of data
    CreditCard(String),
    DebitCard(String),
    Paypal(String, String),
    CashApp(Credentials), // Notice that we're using the credentials struct for the CashApp variant of the enum. This is so that we know that one string will be for the username and another will be for the password
    CashApp2 {username: String, password: String }, // Notice that we're using the credentials struct for the CashApp variant of the enum. This is so that we know that one string will be for the username and another will be for the password
}

#[derive(Debug)]
enum Bean{
    Pinto,
    Black, 
    Kidney,
}
#[derive(Debug)]
enum Meat{
    Chicken,
    Steak,
}
#[derive(Debug)]
enum RestarauntItem {
    Burrito{meat: Meat, bean: Bean},
    Bowl(Meat, Bean),
    VeganPlate(Bean),
}
#[derive(Debug)]
enum OperatingSystems {
    MacOS (String), // This is going to say the macOS version
    Linux {distribution: String},
    Windows,
}
fn main() {
    /*
        Intro to Enums

        An Enum is a type that represents a set of possible values. Each possible value is called a variant.

        Example: Days of the week, seasons of the year, countries of the world

        The idea is that there's a limited set of possibilties. It can use the Debug trait and is declared similiarly to the manner in which a class is declared
     */
    let first_card = CardSuit::Hearts; // Hearts is a variant in the namespace of CardSuit, so we need to declare it in this manner
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs; // It's mutable, so it can take another Variant 


    let card_suits = [CardSuit::Hearts, CardSuit::Spades, CardSuit::Diamonds, CardSuit::Clubs]; // We can put enums in lists and Tuples as well
    let card_tuple = (CardSuit::Hearts, CardSuit::Spades, CardSuit::Diamonds, CardSuit::Clubs);

    /*
        Enum with associated values

        We can add associated data to an enum. Each variant doesn't have to have the same number of associated data
     */

    let visa = (PaymentMethodType::CreditCard, String::from("4100-5003-2008-3301")); // It's often the case that people combine an enum variant with associated data. For this reason, Rust supports enum variants storing more data
    let visa = PaymentMethodType::CreditCard(String::from("4100-5003-2008-3301"));

    let mastercard = PaymentMethodType::DebitCard(String::from("4400-0318-4328-9238"));
    println!("{:#?}, {:#?}", visa, mastercard);


    let paypal_user = PaymentMethodType::Paypal(String::from("omar@gmail.com"), String::from("fakepassword"));
    println!("{:#?}", paypal_user);

    /*
        Enum Memory

        How does rust deal with inconsistencies in required memory for different data types?

        Rust will select the Variant with the highest amount of memory usage based on the data types. For example, a pointer to a string takes 24 bytes of data on the stack. In our example above, the Paypal variant uses 2 string pointers, so that results in 48 bytes of data.
        Therefore, each variant of the PaymentMethodType will be 48 bytes of data in size!

        The total size of the enum memory isn't just related to the variant. It'll allocated at least the amount of the largest variant type
     */

    /*
        Struct Variants
        With a struct variant, instead of assigning the associated data inline based on its position, we can assign it to a name. For example, if "Paypal" enum type takes in two strings, but one is the username and the other is the password, we want to add that information for clarity

        We can define a struct and set that as the datatype of the enum variant
        We can also just name a struct variant without using a struct datatype as an enum variant. Each appraoch comes with its own advantages or disadvantages
    */

    // Using a struct as the data type of the variant - this decouples the struct from the variant
    let cashapp_credentials = Credentials{username: String::from("Homer Simpson"), password: String::from("Woohoo")};
    let cashapp_user = PaymentMethodType::CashApp(cashapp_credentials);
    println!("{:?}", cashapp_user);

    // Using the struct variant format - this gives us named fields for the variant that we can assign directly
    let cashapp_user2 = PaymentMethodType::CashApp2{username: String::from("Homer Simpson 2"), password: String::from("Woohoo2")};
    println!("{:?}", cashapp_user2);

    /*
        Nesting Enums in Enums 
        Since an enum is a valid data type, we can store an enum within another enum! 
     */
    let lunch = RestarauntItem::Bowl(Meat::Chicken, Bean::Black);
    let dinner = RestarauntItem::Burrito{meat: Meat::Steak, bean: Bean::Kidney};
    let brunch = RestarauntItem::VeganPlate(Bean::Pinto);

    println!("{:?}", lunch);
    println!("{:?}", dinner);
    println!("{:?}", brunch);


    /*
        The match keyword
        The match keyword compares a given value against a collection of patterns and executes a block of code based on the first pattern (also known as an "arm") that matches

        Match is good for covering all possible cases of enums! 
     */
    let input_integer = 50;
    match input_integer{
        5 => println!("The number is 5"),
        8 => println!("The number is 8"),
        _ => println!("The number is not 5 or 8"), // The catch all pattern that will always match
    }

    // If we have an enum variant that stores some associated data, we can access that data in the match arms. There will be a different syntax for a tuple variant vs a struct variant in those cases though!
    println!("My computer is {:?}", years_since_OS_release(OperatingSystems::Windows));
    println!("My computer is {:?}", years_since_OS_release(OperatingSystems::MacOS(String::from("Sequoia"))));
    println!("My computer is {:?}", years_since_OS_release(OperatingSystems::Linux{distribution: String::from("Fedora")}));
    
    /*
        Defining Methods on Enums

     */ 

}

fn years_since_OS_release(os: OperatingSystems) -> u32 {
    match os { // NOTE: notice how the rust analyzer gives an error because it expects a match for all of the possible enums, if any are missing!
        OperatingSystems::Windows => { // We can do this since we don't have any sort of associated data
            println!("This is an old operating systems");
            39 // NOTE: notice here we're using an implicit return such that the block returns the same datatype as the other following cases in our match statement!
        },
        OperatingSystems::Linux {distribution} => { // We have to use the name of the associated data
            // the "distribution" keyword is accessible from here!
            println!("The linux distribution being used is {:?}", distribution);
            34           
        }  
        OperatingSystems::MacOS(mac_os_version_string)=> { // We decide what name we want to assign the associated data
            //Here, name_used is of type String and is something we can use in this code block. We assigned "mac_os_version_string" for the first associated data that is part of the MacOS variant
            println!("We're using MacOS version {:?}", mac_os_version_string);
            23
        } 
    }
}
