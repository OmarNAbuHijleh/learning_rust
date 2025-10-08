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
        Stack types implement the copy trait with full copies of the values when needed. This is because stack data is fixed in size. The same rules apply to references
        References are a type and therefore implement the copy trait as well. 
    */
    drop(ice_cream);
    let ice_cream = "Cookies and Cream";
    let dessert = ice_cream; // This references the same "Cookies and Cream" text from the binary. The reference is copied in this case, and we have 2 entries in the stack. There is no movement of ownership!
    println!("{}", ice_cream);
    println!("{}", dessert); // These are both references to the same memory of the binary, however they are 2 distinct variables and are both on the stack!

    /*
        Ownership and Function Parameters
        We've now explored the concepts of copying and moving in ownership. We're going to explore this topic further in regards to function parameters.

        Again, this depends on the type of data being used. Stack data (integers, floats, booleans) - then Rust creates a copy of the data and there is no transfer of ownership.
        When dealing with heap data, ownership IS transferred.
    */
    let input_value = 32;
    print_my_value(input_value); // Here, we have an actual copy being made since it's a stack entry
    println!("{input_value}"); // We can see the input_value is still valid here. 

    // Now we'll do the same thing using a string instead
    let oranges = String::from("Oranges");
    print_my_string(oranges);
    // println!("{oranges} is still valid?"); // A string does not implement the copy trait, so it's actually cleared from the heap following the above function call
    // NOTE: We can fix the problem above by passing in a cloned version of the value!

    /*
        Mutable Parameters

        Just like variables, function parameters are immutable by default. We can declare them mutable if we like though!
    */
    let mut burger = String::from("Burger");
    // let meal = burger;
    // meal.push_str(" and Milkshake"); // This line will not work because even though the burger variable is mutable, the meal variable is not!
    add_fries_2(burger); // Keep in mind that burger is deallocated from memory following this!


    /* 
        Return Values

    */
    let cake = bake_cake(); // A return value allows the deallocation that would have taken place on the heap for the bake_cake() function to a value in the scope that called the function. This also applies to implicit returns
    println!("I now have a {cake} cake");

    // This can be challenging because now we always need a return statement
    let current_meal = String::new();
    let current_meal = add_flour(current_meal); // This will deallocate the text on the heap, so we need to have the return value. This is very inelegant, and this problem will be solved in the next module on References and Borrowing
    println!("{current_meal}");
    // add_sugar();
    // add_salt();


    /*
        Project for this module
    */

    // Question: Does Rust move ownership for this? No - This is because this is a data type on the stack and therefore a copy will be made instead
    let is_concert: bool = false;
    let is_event = is_concert;
    println!("Concert? {is_concert}\nEvent? {is_event}");

    // Question: Does Rust move ownership for this? No - This is a string literal and is in the binary data in memory (i.e. static memory)
    let sushi_2 = "Salmon";
    let dinner_2 = sushi_2;
    println!("{sushi_2} and {dinner_2}");
    
    // Question: Does Rust move ownership for this? Yes - This is because this is a data type on the heap and therefore the sushi ownership will be deallocated
    let sushi = String::from("Salmon");
    let dinner = sushi; 
    // println!("Sushi? {sushi}\nDinner? {dinner}"); 

    // Declaring the eat_meal function that accepts a "meal" parameter of type string
    let dinner = eat_meal(dinner); // dinner gets deallocated due to the fact it is a heap string and it's ownership is passed into the function. We clear it manually in the function but it would be deallocated regardless
    println!("Dinner is: {dinner} today right?");

}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    return meal;
}

fn bake_cake() -> String {
    let cake = String::from("Chocolate Mousse");
    return cake;
}

fn add_flour(mut meal: String) -> String{
    meal.push_str("Add Flour\n");
    return meal
}

fn add_fries(input_string: String){
    // Add to the input_string that we received
    // Note that this code does not work. This is because the input is immutable currently. BUT we still have the problem if we make the original variable mutable prior to the function call
    // The reason it doesn't matter if the variable we pass in is mutable outside the function call is that the variable outside the function call is no longer the owner at that point!
    // Since we're dealing with a heap allocation, we're dealing with the original data
    // input_string.push_str(" and Fries"); // This one does not work because "input_string" is not mutable
}

fn add_fries_2(mut input_string: String){
    // This is the version with a mutable variable. Here, we can mutate the data in memory!
    input_string.push_str(" and Fries"); 
    println!("{input_string}"); // This will work now
}

fn print_my_string(input_string: String){
    // Because we receive a Heap-Stored value (in this case - a string) we find that Rust actually drops it from memory following the execution of this function call!
    println!("You passed in {input_string}");
}


fn print_my_value(value: i32){
    println!("{value} is my value");
}
