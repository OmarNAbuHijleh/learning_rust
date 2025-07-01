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
    println!("{new_val}")

    /*
        Implicit Return Values
        A function will automatically implicitly return the return value of the last line of a function. 
        To do this, you need to remove the "return" keyword, as well as the ";" symbol!
    */

    /*
        The Unit as a Return Type
        NOTE: Omar Left off here
    */
}

fn square_value(input_value: i32) -> i32 {
    return input_value.pow(2);
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
