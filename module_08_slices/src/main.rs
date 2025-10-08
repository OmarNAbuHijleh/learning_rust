// Module 8: Slices

fn main() {
    /*
        Intro to Slices
        Slices are a collection type - more specifically, they are a reference to a portion/sequence of a collection type

        Think of it like a slice of a pizza, or cake. A slice borrows some portion of the collection data type.
        That said, a slice can also refer to the entire collection as well
    */

    /*
        Create a String Slice from a String

    */

    let action_hero = String::from("Arnold Schwarzenneger");
    let string_reference = &action_hero; // A normal reference - this borrows the entire string 

    println!("{string_reference}");

    let first_name = &action_hero[0..6]; // This version is a slice reference. We're taking some
    println!("{first_name}"); // the number in the brackets represents how many bytes we're taking character in a string (this is not the case for emojis! The syntax used here is [first..last-1] - NOTE: A byte corresponds to a single byte

    let last_name: &str = &action_hero[7..21];
    println!("{last_name}");

    /*
        String Slices and String Literals
        Note: The data types of the example below and above are the same!
    */

    // Using the example above, but with a string literal:

    let action_hero = "Arnold Schwarzenneger";
    let string_reference = &action_hero;
    let first_name = &action_hero[0..6];
    println!("{string_reference} and {first_name}"); // Note that we're able to have two slice references here that refer to the same memory locations because they are slices of an immutable piece of data (Stack data!)
    let first_name = {
        let action_hero = "Arnold Schwarzenneger"; // This text is embedded into the final executable, so we won't have a dangling reference!
        &action_hero[0..6]
    };

    println!("{first_name}");

    /*
        String Slice Lengths

    What is the lenth of a string slice?
    The length of a string slice refers to the number of bytes! There's a method "len" that we can use for that
    */
    let food = "pizza";
    println!("{}", food.len());

    let pizza_slice = &food[0..3];
    println!("{}", pizza_slice.len());

    let smile = "🙂";
    println!("{}", smile.len()); // You'll notice that this is more than 1 byte. This is because emoji characters are not one byte each!. Note that we cannot take a slice of an emjoi unless it's the entire emoji!

    /*
        Syntatic Shortcuts
        These are shortcuts we can utilize in slicing situations
        With string slices, it doesn't matter what time of string we're using (binary embedded or heap allocated). This is because borrowing still only occurs at runtime
    */
    let name = "Omar Abu-Hijleh";
    let first_name = &name[..4]; // We don't need to go and define the starting point if we're taking from the beginning of the string
    println!("{first_name}");

    let last_name = &name[5..]; // We don't need to define the ending point if we're going all the way to the end of the string
    println!("{last_name}");

    let full_name_2 = &name[..]; // Gets the complete original string. It is still technically a slice
    let random_code = 88; // This is some random code I'm testing to see if the comment 

    /*
        String Slices as function parameters

        If we have a function take in &String as a parameter, it will only work with the heap allocated strings. However, if we have the function take in the &str, then it will work for both the &String and the &str!

        Rust has something called "deref coersion". This is because if we have a &String, then we know we can obtain a reference to a piece of it and so it'll perform the conversion to a &str. The other way around is not true however, and so the rust compiler does not make that assumption for the reverse scenario!
    */
    let name = String::from("Omar Abu-Hijleh");
    let first_name = &name[..4];

    do_hero_stuff(&name);

    /*
        Array Slices
        We can declare slices on a variety of composite types. One of them is the array!
    */
    let values = [4, 8, 16, 24, 32];
    let my_slice = &values[2..]; // Notice how the hint doesn't show the size of the slice! This establishes how dynamic the content of a slice can be!
    println!("{my_slice:?}");

    let my_slice = &values[..]; // Borrows all of the contents in the slice!

    /*
        Deref Coersion with Array Slices

        This is the same as with string references
    */
    let values = [4, 8, 16, 24, 32];
    let regular_reference = &values; // This is a regular reference. Notice that the reference is of type i32 and specifies the number of values in it for the type hint
    let slice_array_reference = &values[1..3]; // Notice that for this, we have the slice reference but the length is not specified in the type hint. This is because a reference to a slice is not the same as a normal reference.

    print_length_normal_reference(regular_reference);
    print_length_slice_reference(slice_array_reference);
    print_length_slice_reference(regular_reference);

    /*
        Mutable Array Slices

        Rust doesn't allow mutable slices of strings. It does permit mutable slices of arrays though!
    */

    let mut my_array = [3, 5, 8, 10, -1, 2, 7];

    // let my_slice = &my_array[2..4]; // This declaration is an immutable slice!
    let my_slice = &mut my_array[2..4]; // This declaration is an immutable slice!

    my_slice[1] = 0; // Notice how since the slice is mutable we're changing the value of one of element in the slice as well as the array

    println!("my slice is --> {:?}", my_slice);
    println!("my array is --> {:?}", my_array); // We can see that we've successfully changed the array through the slice reference

    /*
        Project
    */

    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two = &cereals[..2];
    println!("{:?}", first_two);

    let mid_three = &cereals[1..4];
    println!("{:?}", mid_three);

    let last_three = &mut cereals[2..];
    println!("{:?}", last_three);

    last_three[2] = String::from("Lucky Charms");
    println!("{:?}", cereals);

    let cookie_crisp = &cereals[0];

    let cookie = &cookie_crisp[..7];
    println!("{:?}", cookie);

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("{:?}", puffs);
}

fn print_length_slice_reference(input_slice: &[i32]) {
    // Notice how this one takes in a general slice (not specifying length?). This one works on
    // both the normal reference and the slice reference! That is because of dereferencing
    // coersion!
    println!("the input slice is of length: {}", input_slice.len());
}

fn print_length_normal_reference(input_slice: &[i32; 5]) {
    // Notice how this reference takes a very specific slice type and size?
    println!("the input slice is of length: {}", input_slice.len());
}

// // This one only works with &String
// fn do_hero_stuff(input_hero_name: &String) {
//     println!("{input_hero_name} does the right thing");
// }

// This one will work with both &str and &String
fn do_hero_stuff(input_hero_name: &str) {
    println!("{input_hero_name} does the right thing");
}
