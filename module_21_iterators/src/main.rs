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
 */

use std::collections::HashMap;

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
 }
