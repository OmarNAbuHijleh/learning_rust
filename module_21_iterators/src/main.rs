/*
 * Manual Iteration
 * Iteration: repeating the same operation on a sequence of items one item at a time
 */
/*
 * The Iterator and Intolterator Traits
 * All iterators rely on the "next" method behind the scenes, and it is how they navigate to the next sequential item regardless of the exact iterator type that we have
 * The "next" method returns an option enum because it's not always guaranteed that the we'll get something, especially if we go out of bounds
 * The benefit of the iterator trait is consistency
 *
 * A type can be converted into a collection by using the "IntoIterator" trait, and by doing so it'll become something that implements the Iterator trait
 */

/*
 * The IntoIterator Trait in Action
 * When a type implements the "IntoIter" trait, the type promises to be convertable to an iterator
 */

/*
 * Exhausting the Iterator
 * Iterators in rust are lazy. They don't start iterating automatically until explicitly told to do so. Iteration cannot occur again after the iterator is "exhausted"
 */

/*
 * The for Loop with Iterator
 * The "for" loop automatically calls the "into_iter" function and takes ownership of the iterator in the background
 */

/*
 * Why Iterator Can Be Immutable
 */

/*
 * The iter Method
 * The iter method will NOT take ownership away from the original data source. It does this by creating an iterator of immutable references to the values of the collection type
 */

/*
 * The iter_mut Method
 * This will create an iterator that yields a mutable reference to each element
 */

/*
 * Hashmap Iteration
 * Hashmap iteration won't inherently be ordered the same every time, because hashmaps don't have an order
 * You'll have 2 pieces of data in every value from the hashmap
 */

/*
 * String Iteration
 * A string character we see is not always equivalent to an individual byte in memory. An english alphabetic character occupies a byte but an emoji might occupy 2 or 4 bytes. Since for strings, we can either iterate over the characters or the bytes in the string.
 *
 * Due to the ambiguity, we can't use the "iter" method or the "iter_mut" method. Instead, we can use the "chars" or the "bytes" method
 */

/*
 * Solving a Problem with Iteration"
 */

/*
 * The for_each Method
 */

use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, u32> {
    // split the spaces
    let words = text.split_whitespace();
    let mut ret_hashmap = HashMap::new();
    for word in words {
        ret_hashmap.entry(word).and_modify(|input_word| {*input_word += 1}).or_insert(1_u32);
    }
    return ret_hashmap;
}

fn main() {
    println!("Manual Iteration");
    let numbers = vec![4, 8, 15, 16, 23, 42];
    // 1.) The loop keyword - continually executes a block until we force termination with the "break" keyword
    let mut current_idx = 0;
    let final_index = numbers.len() - 1;
    loop {
        if current_idx > final_index {
            break;
        }

        println!("{}", numbers[current_idx]);
        current_idx += 1;
    }

    // 2.) The while loop
    let mut current_idx = 0;
    let final_idx = numbers.len() - 1;
    while current_idx <= final_idx {
        println!("{}", numbers[current_idx]);
        current_idx += 1;
    }

    // 3.) The for loop
    for number in numbers {
        println!("{}", number);
    }

    println!("The Iterator and Intolterator Traits");

    println!("The IntoIterator Trait in Action");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter(); // This is the "IntoIter" struct - it is different from the "IntoIter" trait!. Note that the <i32> in the "IntoIter" is because our vector was using i32
    // println!("{my_vector:?}"); // This will not work because we have changed the ownership of the data
    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator = my_hashmap.into_iter();

    println!("Exhausting the Iterator");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter();
    println!("{:?}", my_iterator);
    // We can invoke the "next" method to get the next value as a Some or None depending on if a value exists
    println!("{:?}", my_iterator.next()); // NOTE: Here we see the first value in the iterator provided
    println!("{:?}", my_iterator); // NOTE: Here we see that the iterator is missing the first value

    println!("The for Loop with Iterator");
    // The for loop will automatically call the "next" method repeatedly and unwrapping the value of the "Some" variant. It terminates upon arriving at the "None" variant
    // NOTE: The for loop automatically calls the "into_iter" method behind the scenes
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter(); // This is redundant because of the previous note
    for number in my_iterator { // NOTE: Behind the scenes, ownership of the iterator moved to the for loop
        println!("{number}");
    }

    println!("Why Iterator Can Be Immutable");
    // NOTE: We don't have to make the iterator mutable. We can make the new owner variable mutable if desired. This happens implicitly behind the scenes in a for loop

    println!("The iter Method");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.iter(); // This is redundant because of the previous note
    for number in my_iterator { // NOTE: Behind the scenes, ownership of the iterator moved to the for loop
        println!("{number}");
    }
    println!("{my_vector:?}"); // Ownership has not transferred

    // NOTE: We can also achieve the same result by doing this:
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    for number in &my_vector { // NOTE: Behind the scenes, ownership of the iterator moved to the for loop
        println!("{number}");
    }

    let cities = vec![String::from("Chicago"), String::from("Los Angeles")];
    for city in cities { // Doing this will transfer the ownership of the values in the vector to the "city" variable
        println!("{city}");
    }
    // println!("{cities:?}"); // This won't work because ownership of the values in cities was transferred to "city"

    println!("The iter_mut Method");
    let mut flavors = [String::from("Chocolate"), String::from("Vanilla"), String::from("Strawberry")];
    let iterator = flavors.iter_mut();
    for flavor in iterator {
        flavor.push_str(" Ice Cream"); // Appending to each element of the string
    }
    println!("{flavors:?}");

    // We can also do this
    let mut flavors = [String::from("Chocolate"), String::from("Vanilla"), String::from("Strawberry")];
    // let iterator = flavors.iter_mut();
    for flavor in &mut flavors { // This achieves the same effect as the above code
        flavor.push_str(" Ice Cream"); // Appending to each element of the string
    }
    println!("{flavors:?}");
    // NOTE: With some data types, we still need to use the de-reference operator (the "*") to access the value at the address if we want to modify it, because not every method (like "push_str") will do that for us

    let mut school_grades = [85, 90, 72, 92];
    for grade in &mut school_grades {
        // grade -= 2; // This won't work and will try to perform the actionn on the reference. We have to de-reference it
        *grade -= 2;
    }
    println!("{school_grades:?}");

    println!("Hashmap Iteration");
    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (todo_key, todo_val) in todos { // moves ownership
        println!("Task: {todo_key}, Complete: {todo_val}");
    }
    // println!("{todos:?}"); // will not work because ownership moved

    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (todo_key, todo_val) in &todos { // does not move ownership
        println!("Task: {todo_key}, Complete: {todo_val}");
    }
    println!("{todos:?}"); // will work because ownership did not move

    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (_, todo_val) in &mut todos { // does not move ownership. Can use "_" to throw away a value
        *todo_val = true; // needed to change the value
    }
    println!("{todos:?}"); // will work because ownership did not move. Values successfully changed

    println!("String Iteration");
    let seafood = "Oyster🦪"; // This emoji occupies 4 bytes in memory
    for byte in seafood.bytes() {
        print!("{byte}/");
    }
    println!("{seafood}"); // This will not lose ownership

    for character in seafood.chars() {
        print!("{character}/");
    }
    println!("{seafood}"); // This will not lose ownership

    println!("{}", seafood.bytes().len()); // 10 bytes. These functions exhaust the iterator themselves
    println!("{}", seafood.chars().count()); // 7 chars. These functions exhaust the iterator themselves


    println!("Solving a Problem with Iteration");
    let some_text = "Some text to try testing the iteration test method we made where test shows up twice";
    println!("{:?}", count_words(some_text));

    println!("The for_each Method");

}
