fn main() {
    /*
        The if statement


    */

    let var: i32 = 3;
    if var==3{
        println!("Variable is {var}=3");
    }


    let season_var: &str = "random_text"; // Change this as desired
    fun_func(season_var);

    /*
        Assigning the result of an "if" statement to a variable
     */

    let is_summer: bool = if season_var=="summer" {true} else {false};
    println!("{is_summer}"); 


    /*
        The match statement
        The match statement can react to all possible varaints of a value. It's basically repeated "if" statements for the input. Note we have to write an evaluation for all possible values!
    
        Each "arm" of a match statement can also return a value!

        NOTE: 
        - A match statement is only going to look for the first match, perform it's requirements, then break
        - A match statment needs to be exhaustive. The "_" character is a good catch-all. This is usually going to be last because otherwise the following statements would never break
        - We can perform multiple evaluations and conditionals within a match statement. This would involve a variable name
        - We can use the "unreachable!()" macro for isntances that are not actually possible. It's more efficient for the Rust compiler.
    */
    let evaluation: bool = true;

    match evaluation {
        true => {
            println!("The value is True");
        }
        false => {
            println!("The evaluation is False");
        }
    }

    // Mapping the evaluation to a return value
        let value = match evaluation {
        true => 20,
        false => {
            println!("The evaluation is False");
            30 // This is going to get returned from the match statement!
        }
    };
    println!("Value is {value}");


    // using an _ in a match statement arm
    match season_var {
        "summer" => println!("School's out"),
        "winter" => println!("Brr"),
        _ => println!("Either spring or fall") //This is the catch all pattern. We'll often need this in match statements so that the statements are exhaustive
        
    }

    // This is the match statement with multiple values and conditionals. Note that we can also use the "unreachable" macro for situations that are, in fact, unreachable
    let var_value = 1;
    match var_value {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 !=0 => println!("{value} is an odd number"),
        _ => unreachable!()
    }

    /*
        The Loop and Break Keywords
        We use the "loop" keyword to continuously do a task. We can use an if statement with the "break" keyword to break from the loop
    */
    let mut counter = 10;
    loop {
        if counter == 0{
            println!("Blastoff!!!!");
            break;
        }
        println!("{counter}");
        counter-=1;
    }

    /*
        The Continue keyword
        The "continue" keyword allows us to skip the execution of the rest of the code block and run straight to the next iteration of the loop!    
    */

    counter = 21;
    loop {
        if counter == 0 {
            println!("Blastoff");
            break;
        }
        // We will only print if the counter is modulo 3
        counter -= 1;
        if counter % 3 != 0 {
            continue
        }
        println!("{counter} seconds to blastoff . . .");
    }

    /*
        The while loop

        We don't need the "break" keyword because the while loop has a mechanism to terminate automatically
    */

    counter = 21;
    while counter > 0 {
        counter -= 1;
        if counter==0 {
            println!("Blastoff!!!");
            continue
        } else if counter % 3 != 0 {
            continue
        }

        println!("{counter} seconds to blastoff . . . ");
    }


    /*
        Recursion
        This is when a function calls itself. To avoid calling itself infinitely, we use the following:
        - A base case. This allows us to set up the condition to return the final result
    */

    countdown(5);

    /*
        Debugging in Rust
        Note that the debugger in rust let's us view a variable value by typing it "? <variable name>"
        We can use the "watch" window in vscode to type in an expression related to a variable that will automatically recompute at each iteration when we arrive at a break point.
        You can also just type in a code expression into the actual code, but that requires recompilation and you have to remember to delete it after the fact
    */


}

fn countdown(seconds: i32) {
    // This function takes in a number of seconds and then recursively performs a coundown sequence!
    if seconds==0 {
        println!("Blastoff!!!");
        return ()
    }
    println!("{seconds} to blastoff . . .");

    countdown(seconds-1);
}


fn fun_func(input_var: &str) {
    if input_var == "summer" {
        println!("It is summer. Beach Fun");
    } else if input_var == "winter" {
        println!("Brrr");
    } else if input_var == "spring" {
        println!("Green Grass!");
    } else {
        println!("Hope you had a great fall!");
    }
}
