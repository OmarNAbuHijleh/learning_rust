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
    The entry method accepts a hashmap key and returns an enum called "Entry" representing the possibility that the enum exists or does not exist

    The "occupied" and the "vacant" entries are the enum value results
    "occupied" will store the key and value. Vacant will just store the key
    The "or_insert" method can be used to store the key and a value if it is vacant!

    NOTE; The results of the "or_insert" will be the value (default or existing)
*/

/*
    The Hash Set
    This is often times just a 'set' in other languages. It is in the collections module
*/

/*
 * PROJECT

 */

use std::collections::HashMap;
use std::collections::HashSet;

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

    // The entry method
    println!("The Entry Method");
    let vacant_result = String::from("Vacant Result");
    let expected_occupied_type = coffee_pairings
        .entry(&drink)
        .or_insert(String::from("Almond Milk"));
    println!("{:?}", &expected_occupied_type);
    let expected_vacant_type = coffee_pairings
        .entry(&vacant_result)
        .or_insert(String::from("Almond Milk"));
    println!("{:?}", &expected_vacant_type);
    println!("{:?}", coffee_pairings);

    // The hash set
    println!("The Hash Set");
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:?}", concert_queue);
    concert_queue.insert("Omar");
    println!("{:?}", concert_queue);
    println!("{:?}", concert_queue.len());
    concert_queue.insert("Omar"); // This won't do anything because the entry is already present
    concert_queue.remove("Omar"); // Will return a boolean "True" if it exists and is removed and "False" otherwise

    concert_queue.insert("Omar"); // This won't do anything because the entry is already present
    concert_queue.contains("Omar"); // This will return "true" or "false"

    println!("{:?}", concert_queue.get("Omar")); // This will give the "Some" variant if it exists and "None" otherwise


    // The hash set operations
    println!("Hash Set Operations");
    // These let us compare sets together
    let mut movie_queue: HashSet<&str> = HashSet::new();
    let mut concert_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");


    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    // Note that this is UNORDERED
    println!("{:?}", movie_queue.union(&concert_queue)); // A union. Note that the union is a new set, it doesn't affect the original sets. Note that this is a special struct called "Union".
    println!("{:?}", movie_queue);

    println!("{:?}", movie_queue.difference(&concert_queue)); // A difference

    println!("{:?}", movie_queue.symmetric_difference(&concert_queue)); // A symmetric difference - gets the values that are in one set or the other but not both.

    println!("{:?}", movie_queue.is_disjoint(&concert_queue)); // gives boolean "true" if there is absolutely no overlap between the two sets, "false" otherwise.

    println!("{:?}", movie_queue.is_subset(&concert_queue)); // gives boolean "true" if concert_queue contains all the values in movie_queue, "false" otherwise.

    println!("{:?}", movie_queue.is_superset(&concert_queue)); // gives boolean "true" if movie_queue contains all the values in concert_queue, "false" otherwise.



    println!("PROJECT");
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from(
        [
            ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
            ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
            ("Mustard", vec!["Hot dog", "Burgers", "Pretzels"])
        ]
    );

    println!("{:?}", sauces_to_meals);


    let result = sauces_to_meals.remove("Mayonnaise");
    println!("{:?}", result);

    let result = sauces_to_meals.get("Mustard");
    println!("{:?}", result);

    sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);


    println!("{:?}", sauces_to_meals);

}
