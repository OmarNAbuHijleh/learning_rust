// Data Types Module
fn main() {
    /*
        Different data types:
            scalar type: holds a single value. 4 of these in Rust - integers, floating-point, booleans, and characters
                integer subcategory - signed and unsigned
                The bit representation for both signed an unsigned ranges from 8 bits to 128 bits
            
    */

    // Integers
    let _eight_bit: i8 = -112; // i8 -- only 127 is the maximum value we can represent, -128 is the minimum

    // We can use the _ symbol as a way of visually making a numerical value easier to read. It functions as a separator
    let _sixteen_bit_signed: i16 = 32500;
    let _sixteen_bit_signed_2: i16 = 32_500; //This is the same value as above! we're using the _ symbol to make it easier to look at the number


    /* 
    The usize and the isize types 
    usize and isize are aliases for an existing integer type. This will vary based on the architecture of the OS the code is being run on.

    usize will be 32 bits on a 32 bit system and 64 bits on a 64 bit system!

    the advantage of this is that we can write one bit of code that is then reusable on multiple systems
    */
    let _days: usize = 500; 


    /*
        Strings and raw strings
        Rust has different types of strings.        
     */ 
    println!("This is a string literal. Our code knows the string and it's contents at compile time");
    println!("This is adding in a newline character\n\nnewline result");
    println!("\tThis is a tab character");
    println!("Juliet said \"Hello, Romeo!\""); // We use the \for quotations to help our code differentiate between the terminator of string and the character in the string
    
    let filepath = "C:\\My Documents\\new\\videos"; // double back slashes in the string to make a single "\" character appear
    println!("{filepath}");
    let new_filepath = r"C:\My Documents\new\videos"; // The "r" character in the front tells the compiler to ignore the special characters in the string
    println!("{new_filepath}");

    /*
        Intro to Methods
        A method is a function that lives on a value. It's an action we can ask the value to execute.
        The method operates on a value
    */
    let value: i32 = -15;
    println!("{}", value.abs()); //This will give us 15
    let empty_space = "          string_contents           ";
    println!("{}", empty_space.trim());
    println!("{}", value.pow(2));
    

    /*
        Floating point types:
        Rust has 2 of them - 32 bit and 64 bit. There is no unsigned float type in Rust!
    */
    let pi: f64 = 3.14159; // Note that Rust will round a floating point value to meet the bit length requirements

    println!("{}", pi.floor() );
    println!("{}", pi.ceil() );
    println!("{}", pi.round() );

    /*
        Formatting floats with the format specifier
    
        A format specifier is a syntax that allows us to customize the printed representation of the underlying value. It doesn't change the value itself, 
        just what it looks like when we perform, for instance, a print
    */
    let pi = 3.14159;
    println!("The value of pi to the first two decimal points is {pi:.2}");
    println!("The value of pi to the first two decimal points is {:.2}", pi);

    /*
        Casting types with the "as" keyword

    */
    let miles_away: i32 = 50;
    let _miles_away_i8: i8 = miles_away as i8;

    let miles_away_float: f64 = 100.329032;
    let _miles_away_f32: f32 = miles_away as f32;
    let miles_away_int = miles_away_float as i32;
    println!("{miles_away_int}");

    /*
        Math Operations
        5 + 4 ---> Here, "+" is the operator and "5" and "4" are the operands

        In the case of an integer divided by an integer, Rust performs floor division

        We can perform the following:
        Addition (+)
        Subtraction (-)
        Multiplication (*)
        Division (/)
        Modulus (%)

    */
    let floor_division = 5/3;
    let decimal_division = 5.0 / 3.0;
    println!("floor division: {floor_division}");
    println!("decimal division: {decimal_division}");

    /*
        Augmented Assignment Operator
        It's common that we perform an operation on a specific variable and then reassign to the variable

    */
    let mut year = 2025;
    year = year + 1;
    println!("{year}");
    year += 1;
    println!("{year}");

    /*
        Intro to Booleans
        A type whose values can only be true or false
        Rust uses a full byte for a boolean for performance reasons. Individual bits are too slow to access
        A boolean can be inverted with the ! operator

        Equality and Inequality Operators
        ==
        !=
        >=
        >
        <=
        <
        &&
        ||  
    */
    let is_handsome = true;
    let is_silly = false;
    let age: i32 = 32;

    let is_young = age < 35;
    println!("{is_young}");
    println!("{} {}", age.is_positive(), age.is_negative());

    println!("{}", !age.is_positive()); // inverting the boolean

    /*
        The character type
        A character datatype represents a single unicode character. It doesn't necessarily have to be alphabetic. It could be an emoji, for example. UTF-8 is another example.
        Unicode characters in Rust are 32 bits so that we can store any possible unicode character
    */
    let first_character = 'c';
    let emoji = '🙂'; // This is another character
    println!("{} is alphabetic? {}", first_character, first_character.is_alphabetic());
    println!("{} is alphabetic? {}", first_character, first_character.is_uppercase());
    println!("{} is alphabetic? {}", first_character, first_character.is_lowercase());
    println!("{} is alphabetic? {}", emoji, emoji.is_alphabetic());


    /*
        The Array Type
        This is a compound type. It contains more than one value. 
        It is a fixed size collection of homogoneous data (i.e. 1 data type the entire time)
        It's also an ordered sequence of elements.
    */
    let numbers = [4, 8, 15, 16, 23, 42];
    let mut apples = ["Hershey", "Recess Puffs", "Mars"];
    println!("Length: {}", apples.len());

    let candy_bar = apples[0];
    println!("The candy bar is {}", candy_bar);

    apples[0] = "Crunch"; // Changing array value at specified index
    let candy_bar = apples[0]; 
    println!("The candy bar is {}", candy_bar);


    /*
        The display trait
        In the real world, a contract is a document people sign that state their obligations. It's a similar idea to traits in Rust.

        In Rust, a trait is like a contract that requires a type support one or more methods. 
        They establish consistency between different types. When a type honors those requirements, it "implements" the trait. The types themselves can still vary in implemenatation logic

        A type doesn't have to implement a trait. It is up to the developer what traits are implemented for what types. This is basically interfaces in other programming languages

        NOTE: The benefit if a trait is that they are handled during compile time instead of run time, reducing the number of dynamic lookups that take place!

        ABOUT THE DISPLAY TRAIT: The display trait requires that a type can be represented as a user-friendly, readable string. 
        When we use the {} interpolation syntax, Rust calls on the format method for that type.

        Integers, floats, and booleans all implement the display trait so that we're able to interpolate them with curly braces.

        It's not always clear how a complex type should be represented as a piece of text. An example would be an array type. Rust didn't implement the display trait for the array
    */
    // println!("{}", apples); // This will give an error
    println!("{}", 1);
    println!("{}", 1.5);
    println!("{}", true);

    /*
        The debug trait
        The goal of the debug trait is to format a given type to a string for debugging purposes. It's similar to the display trait. 
        The debug trait is made for an array, whereas the display trait is not!
    */
    println!("{:?}", apples); // This is how we call the debug trait
    println!("{apples:?}"); // This is how we call the debug trait
    println!("{apples:#?}"); // This is how we call the debug trait - this one is going to prettify the output

    /*
        The dbg! macro
        A macro is not exactly a function but is instead replaced with valid function calls.
        The debug macro exists to print and return the value of a given expression for quick and dirty debugging.

        A trait is an interface basically. a macro is the actual function call that takes place for that interface!
        The argument we pass into the debug macro MUST implement the debug trait!
    */
    dbg!(2+2); 
    dbg!(apples);


    /*
        The Tuple type
    */

}
