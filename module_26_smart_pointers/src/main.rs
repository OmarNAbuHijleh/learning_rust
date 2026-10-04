/*
 * Raw Pointers
 * Rust has variables - a name or piece of data - that are assigned a rust type
 * A pointer is a variable that contains an address in memory. By storing an address, a pointer variable
 * indirectly points to a piece of data. A pointer prevents duplication of the piece of data and instead just points to the address
 *
 * Example: Writing down a home address on a piece of paper. The paper is the pointer and the address is the data it points to.
 *
 * Throughout the course, we've worked with a lot of references.
 * A reference is a type of pointer that points to valid data. We create references with the borrow operator "&"
 * Rust's borrow checker guarantees that references point to valid/allocated data. Pointers in other language do not make the same promise.
 *
 *
 * A raw pointer is a variable that stores a memory address without any safety checks. It may point to valid data but it may also point to deallocated memory.
 * Raw pointers are common in languages like C and C++ that do not have a garbage collector. Developers still call them references even though they lack the guarantees
 * of Rust's references.
 *
 * Rust supports raw pointers but references are safer and recommended.
 *
 * To use rust's raw pointers, you have to manually opt out of rust's safety guarantees!
 *
 * In C/C++, a pointer is a variable that stores a memory address of a piece of data while a reference is an alias (another name) for an existing variable.
 *
 * Smart pointers abstract away the complexity of raw pointers and provide a safer alternative.
 */

/*
 * Raw Pointers and Unsafe Code
 */

/*
 * Smart Pointers
 *
 */

fn main() {
    println!("Raw Pointers and Unsafe Code");

    let sushi = String::from("Yellowtail");
    let sushi_reference = &sushi; // This is a reference
    let sushi_reference_2 = &sushi; // We can make as many references as we want if it's to an immutable variable. This will break if we make the variable mutable

    let sushi_raw_pointer_1 = &raw const sushi; //A raw pointer that points to the address of the sushi variable
    let sushi_raw_pointer_2: *const String = &sushi; // A regular reference being forced into a raw pointer

    // Unsafe example of code! We can make as many raw mutable pointers as we want, but they all point to the same address. The compiler will only show a violation when we try to dereference the pointer, hence
    // why we would need an "unsafe" block to dereference it without the compiler complaining
    let mut sushi = String::from("Yellowtail");
    let sushi_raw_mutable_pointer_1 = &raw mut sushi; // A raw mutable pointer that points to the address of the sushi variable
    let sushi_raw_mutable_pointer_2 = &raw mut sushi; // Another raw mutable pointer!

    unsafe {
        println!("{}, {}", *sushi_raw_pointer_1, *sushi_raw_pointer_2);
    }
    println!("Now we're dropping the variable while keeping the raw pointers dangling");
    drop(sushi); // We drop the variable, leaving the raw pointers dangling. This means our pointer is now pointing to invalid memory! It may contain something totally different from before now!
    // This keyword acknowledges the possibility of undefined behavior so the compiler allows us to do whatever we want.
    unsafe {
        println!("{}, {}", *sushi_raw_pointer_1, *sushi_raw_pointer_2);
    }

    println!("Smart Pointers");
}
