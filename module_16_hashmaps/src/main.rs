/*
    Create a  Hashmap with a new Function
    A hashmap is a collection type consisting of key-value pairs. A key must be unique while a value can be a duplicate. Each entry in the hashmap consists of a key and value.
    There is no concept of order - for that we use an array or vector. For associations - a hashmap is a good data structure.

    Hashmaps are very fast for getting a corresponding value. These are stored on a heap! The hashmap type is in the standard library

    There is a "from" variant of the hashmap constructor that can be used to create a hashmap from a tuple
*/

/*
    The Remove Method
    The remove method will remove a key value pair by using a key.
    This returns an Option enum (None or Some), in case key doesn't exist
*/

/*
    HashMaps and Ownership
    The hashmap lives on the heap. It's also the owner of the data stored in the keys and values and will take ownership of anything that doesn't have the copy trait
*/

/*
    Access a Value by Key
    We can access a value by using the square bracket notation and passing in the key (similar to python). The problem with this is that if the key doesn't exist, we'll have
    a run time panic. To avoid this, we can use the get method, which returns an Option enum (None or Some)
 */

/*
 *  Overwriting a Value with an Existing Key
    Using the insert method, we can replace the prior key
 */

/*
    The entry method

*/

use std::collections::HashMap;

fn main() {
    println!("Adding to Hashmap");

    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 24.99);
    menu.insert(String::from("Tuna"), 30.99);
    menu.insert(String::from("Burger"), 15.49);

    println!("{menu:?}");

    let mut country_capital: HashMap<String, String> = HashMap::new();
    // let mut country_capital = HashMap::<String, String>::new(); // NOTE: Can also do it like this!
    country_capital.insert(String::from("France"), String::from("Paris"));
    country_capital.insert(
        String::from("United States"),
        String::from("Washington D.C."),
    );
    println!("{country_capital:?}");

    println!("Removing from hashmap");
    let data = [("jimmy", 1), ("omar", 10), ("ali", 5)]; // this is an array of tuples that contain the key-value pairs respectively
    let mut names_hash = HashMap::from(data);
    println!("{:?}", names_hash);

    let jimmy = names_hash.remove("jimmy");
    println!("{:?}", jimmy.unwrap()); // jimmy variable has the value in the hashmap
    let jimmy2 = names_hash.remove("jimmy");
    print!("None results - ");
    println!("{:?}", jimmy2); // This should be a None based on it not being in the hashmap anymore!
    println!("{:?}", names_hash);
    println!("{:?}", names_hash["omar"]); // NOTE: this is how we access a value of a key!

    println!("Hashmaps and Ownership");
    let mut coffee_pairings = HashMap::new();
    let drink = String::from("Latte");
    let drink2 = String::from("Latte2");
    coffee_pairings.insert(&drink, drink2);
    println!("{:?}", drink); // No problems
    // println!("{:?}", drink2); // Will throw an error

    println!("Accessing a Value by it's key");
    let value = &coffee_pairings[&drink];
    let value2 = &coffee_pairings[&String::from("Latte")];
    println!("{:?}, {:?}", value, value2);

    let value3 = coffee_pairings.get(&String::from("Omar String")); // This will give a None
    println!("{:?}", value3);

    //Overwriting an existing key's value
    println!("Overwriting an existing key's value");
    println!("Before: {:?}", coffee_pairings);
    coffee_pairings.insert(&drink, String::from("Hot Chocolate"));
    println!("After: {:?}", coffee_pairings);
}
