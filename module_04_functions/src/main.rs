// Module 4 -- Function

/*
    Functions in Rust
        These are declared using the "fn" keyword followed by a snakecase syntax usually
        Functions in Rust can be declared before our after the main function, it doesn't matter. 
        The function just needs to be defined in the same scope as the main function
*/

fn main() {
    println!("Hello, world!");
    // Examples of function calls
    bake_pizza();
    swim_in_profit();


    /*
        Parameters and Arguments
        Parameters - the name of an expected input
        Argument - The actual value

        Format
        fn func(var_name: dtype) {
        }
    */
    open_store("Chicago");

    /*
        Explicit Return Values
        These are the output of a function. Every Rust function must have a return value
    */
    let new_val: i32 = square_value(3);
    println!("{new_val}");

    /*
        Implicit Return Values
        A function will automatically implicitly return the return value of the last line of a function. 
        To do this, you need to remove the "return" keyword, as well as the ";" symbol!
    */

    /*
        The Unit as a Return Type
        A unit is an empty tuple - a tuple without values. This is the default return value of a function where the return type is not specified
    */


    /*
        Blocks in Functions
        We can declare a block within a function body. It's not a function, but it is isolated in scope! It's a boundary for the code. It has access to outside variables, but
        the outside variables do NOT have access to the variables inside the scope! We can create "anonymous" functions in Rust (simlar to lambdas in python) by taking a block and it's output
        as a variable assignment!
    */

    let multiplier: i32 = 3;
    let calculation_result = {
        let value = 5+4;
        value*multiplier // not adding a semicolon here means that our return type is the type of the last line. In this case - we have an i32!
    };

    println!("{calculation_result}");

    /*
        Project
    */
    apply_to_jobs(10, "Data Scientist");
    let val_9 = is_even(9);
    let val_8 = is_even(8);
    println!("9 is even: {val_9}\n\n8 is even: {val_8}");

    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));

}

fn alphabets(input_string: &str) -> (bool, bool){
    return (input_string.contains('a'), input_string.contains('z'));
}

fn is_even(input_value: i32) -> bool {
    return 0 == {
        input_value % 2
    }
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I am applying to {number} {title} jobs.");
}

fn square_value(input_value: i32) -> i32 { return input_value.pow(2);
}

fn open_store(neighborhood: &str){
    println!("Opens my pizza store in {neighborhood}");
}

fn bake_pizza(){
    println!("Baking a pizza");
}

fn swim_in_profit(){
    println!("So much $ so little time");
}
