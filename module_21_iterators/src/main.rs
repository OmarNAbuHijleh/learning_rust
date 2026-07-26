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
 *
 */
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
}
