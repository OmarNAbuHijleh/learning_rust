// Module 6 - Ownership!

fn main() {
   /*
        Ownership
        Ownership is a compiler level feature that exists to protect us, the coder

        It's a set of compiler rules that the program will check to ensure that the program will be free of memory errors. Ownership is the memory management model that Rust follows
        In C, the developer has the onus of managing the memory. This leads to two bugs - forgetting to free memory and attempting to free already free memory

        Java, Python, Ruby, and Go have Garbage Collection - they look for data no longer in use and clean it automatically. 
        The garbage collector itself takes up memory and can be used at inopportune moments. 

        The rust compiler will not compile a program if an ownership rule is violated.

        How it works
        Every piece of data in the program has an owner. The owner is responsible for cleaning up the data that is no longer in use
        Every value in a Rust program has ONE owner
        The owner can change as a program runs, but there's only one owner for a piece of data at a given time
        The owner is usually a name. A variable or parameter is an example of an owner
        Ownership can extend to composite types such as arrays or tuples. An array owns its own elements

    */

    /*
        The Stack and Heap

        The stack and the heap are two different regions of the computer's memory. They read and write data in different ways
        Rust will write to either the stack or heap memory.

        The stack is faster but only works on fixed size data
        The heap is slower but supports dynamically sized data


        Stack: Last in first out. It stores values in order and removes starting from the most recent addition
            Adding to a stack: Push
            Removing from a stack: Pop

            - all stack data must have a consistent size at compile time
        
        Heap: A large area of storage space. a program requests space from the heap at run time. Things like user input or file content - where we don't know the sizing
            1.) Memory allocator finds a big enough area of memory and returns a memory address
            2.) The memory address is then stored on the stack, as it has a fixed size!

            - Allocating on the heap is slower, and so is finding data

        Ownership is a compiler feature that primarily exists to reduce heap data

     */ 


    /*
        Scope and Ownership 

        When we assign a value to a variable, the variable becomes the owner of the value. When the variable goes out of scope, the owner knows to deallocate the data from memory
 
     */

    let age = 32;
    {
        let is_handsome = true; // This memory gets released once this block is passed
    }

    /*
        The Copy Trait 
        A trait is an interface. 

        The copy trait mandates that a type can be copied or duplicated. Everything we've learned so far implements the copy trait    
    */
    let time = 2025;
    let year = time; // Rust will make a full copy of this on the stack because rust implements the copy trait for integers

    //The original time variable remains valid after being copied.
    println!("The time is {time} and the year is {year}");

    /*
        The String Type
        Rust has 2 string types. The "str" strings. This type is not stored on the stack or the heap, rather the binary executable.     
    */
    let food = "pasta"; // At compile time rust knows to include this text into the binary.
    let text = String::new(); // This is a dynamic string type that might vary over time. This can undergo mutation operations. It's stored on the Heap!
    let text2 = String::from("Kitkat"); // We can directly pass in the raw string that will live on the heap and is still able to undergo changes
    println!("Food is static and it's value is {food}. Text is dynamic and it's value is |{text}|. Text2 is dynamic and it's value is {text2}");

    /*
        The push_str() method on a string type
        This adds content to the end of a heap-allocated string. In other words, concatenation!         
    */
    let mut name: String = String::from("Omar"); //NOTE: When we do this, we're also creating a stack entry because this is a static string!
    // NOTE: The stack entry will hold 3 pieces of data - the length of the string, the capacity of the string (bytes of available space in the heap location), and the reference to the string's heap location. The heap will have the string itself
    name.push_str(" Abu-Hijleh");
    println!("{name}");
    // If the newly added characters exceed the capacity available in bytes, the text will be moved to a new heap location and the old memory will be deallocated

    /*
        Moves and Ownership
        Moves are a key feature of ownership and refers to the transfer of ownership from one owner to another. 
        There can only be one owner at a time but who the owner is can change!
    */
    let person = String::from("Omar");
    println!("{person}");
    let genius = person; // This code will operate differently from the integer example. THE COPY TRAIT DOES NOT OCCUR HERE. This moves ownership to genius - ie. the reference changes!
    // Now the person variable doesn't have ownership of the contents!

    //println!("{person}"); // This fails


    /*
        The Drop Function
        The drop function deallocates memory on a heap. It's used behind the scenes at the end of a block. We can also use it explicitly
    */
    drop(genius); // This deallocates the memory location
    

    /*
        The clone method

        When we want to duplicate heap data, we need to tell Rust explicitly using the "clone" method. It comes from the Clone trait!
    */

    let person = String::from("Omar");
    let genius = person.clone(); // Now if we drop genius, it won't matter
    drop(person);
    println!("{genius} is the genius!");

    /*
        References and Borrowing

        Every rust value has one owner at a time. cloning and ownership are fine, but cloning in particular can be taxing.

        Borrowing allows a variable to be used without taking ownership of it! A reference is an address in memory. Borrowing is when that reference is being assigned to a value, and is not memory intensive
        The "&" is how we do borrowing

        The reference to a value must never outlive the referer
    */

    let my_stack_value = 2;
    let borrowing_var = &my_stack_value;


    let my_heap_value = String::from("Volkswagon"); // This is on the Heap
    let my_heap_reference = &my_heap_value; // This is on the stack, and takes us to the heap value! Both this and the last value are deallocated from memory at the end

    //drop(my_heap_value); // This causes an error because we have to drop the borrower first!
    println!("{my_heap_reference}");


    /*
        The Dereference Operator
        This is represented by the "*". To dereference means to access the data that the memory address points to
        
    */
    println!("{}", *my_heap_reference);

    /*
        String vs &String vs str vs &str
        String - A dynamic piece of data stored on the heap at run time
        &String - The memory address of the dynamic piece of data on the heap (this is stored directly on the stack though)
        str - a hardcoded, read-only piece of text that we encode directly on the binary file
        &str - the reference to the text in memory that is already encoded on the binary file
    */
    let ice_cream = "Cookies and Cream"; // This is not stored on the stack or the heap, this is hardcoded text on the binary that gets loaded into a memory location. 
    // The data type of the above variable is a reference to the text in the binary. It's not on the stack nor the heap!
    println!("{}", ice_cream); // if we were to look at the binary of the executable, there's a spot where "Cookies and Cream" is embedded directly into the binary executable!

    /*
        The Copy Trait with References

    */
}
