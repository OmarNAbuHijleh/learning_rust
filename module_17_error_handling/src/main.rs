/*
 * The panic! macro
 There are two types of errors that can occur at run-time:
 - Recoverable: Errors that we can define the code to handle
 - Unrecoverable: Errors that we cannot define the code to handle

 When the program runs into an error at run-time, we call that a panic
 When a panic occurs, the program will stop executing and print a backtrace, which tells us the line number and file where the panic occurred, as well as other files that were running at the time.

 We can force a panic manually with a macro called panic!

 "Unwinding": This is the default behavior of a panic, where the variables are cleaned up before the panic occurs

 */

/*
* The Process Module and the Exit Function

Sometimes we want to exit the program gracefully without showing a bunch of messages to the user.

We can do this using the exit function from the process module

The convention is that if there is an exit code of 0, the program exited gracefully. If not, then the program exited with an error.
*/

/*
 * Standard Error (eprintln! macro)
 The "e" in eprintln stands for error. Standard output vs error output is that standard output is for users and error output is for the program itself.

 The eprintln! macro lets us print to a different channel. For example, if in the terminal we decide to save the output using "cargo run > output.txt", we'll see the error printed out on the terminal but not the normal text!
 */

 /*
  * Opening a File
  */

/*
 * Asking the User for Input
 */

/*
 * Reading the File's Contents
 */

 /*
 * Propogating Errors
 * To propogate an error is to send it up so that it is handled by a higher-level piece of code. For example, if a function calls another function that returns an error, the error can be delivered by the calling function even if it occurs in the called function.
 */

 /*
  * Understanding Error Type Redeclaration
    Rust's standard library includes many different error types that are used to represent different kinds of errors that can occur during program execution.
    The examples we have in this one are io errors, but you can have others like the fmt errors.
  */

 /*
 * The ? Operator (The Try Operator)
 * Rust offers a special operator to propogate errors in a concise way. This is the '?' operator.
 */
/*
 * The read_to_string Associated Function
   There's a simpler way to do everything we've done already.
 */

/*
 * Using ? with Option
 * We can use the ? operator with the Option types to propogate errors in a concise way.
 */
use std::process::exit;
use std::fs::{self, File}; // This is the file system module
use std::io::{stdin, Read};

fn main() {
    println!("The panic macro");
    // None.unwrap(); // This will cause a panic, as it only works on the "Some" variant and not the "None" variant
    // panic!("Something went wrong"); // This is an example of a panic

    println!("The process module and the exit function");
    // exit(0); // This will exit the program gracefully
    // println!("This will not be printed"); // This will throw a warning from the compiler, as it is unreachable

    println!("Standard Error (eprintln! macro)");
    eprintln!("This is an error message!");

    println!("Opening a File");
    let file = File::open("story.txt"); // This is a result enum. Either Ok or Err depending on how it did in opening the file
    let failed_open = File::open("nonsense.txt"); // This will give an Err.
    println!("{file:#?}"); // Reads the file contents

    println!("Asking the User for Input");
    println!("Please enter the name of the file you would like to read.");
    let mut input = String::new();
    let requested_file_status = stdin().read_line(&mut input);
    let mut file = match File::open(input.trim()) { // The "input.trim()" removes the newline character from the input
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to Open File --> Error: {err}");
            exit(1)
        }
    };
    println!("{:#?}", file);

    println!("Reading the File's Contents");
    let mut file_contents = String::new();
    let read_operation = &file.read_to_string(&mut file_contents);
    if let Err(err) = read_operation {
        eprintln!("Failed to read the file --> Error: {err}");
        exit(1)
    }
    println!("{:#?}", file_contents);

    println!("Propogating Errors");
    let result_propogated = read_file();
    match result_propogated {
        Ok(contents) => println!("{:?}", contents),
        Err(contents) => println!("Error --> {contents}")
    };

    println!("The ? Operator (The Try Operator)"); // Look for the operator in the "read_file" function

    println!("The read_to_string Associated Function");
    let res = read_file2();
    if let Err(err) = res {
        eprintln!("Error for readfile2 --> {err}");
        exit(1);
    }
    println!("{:?}", res.unwrap());

    println!("Using ? with Option");
    let mut animals = vec!["Giraffe", "Monkey", "Elephant"];
    let length_value = length_of_last_element(&mut animals);
    println!("{:?}", length_value );

    println!("Project Results");
    let result = write_to_file();
    match result {
        Ok(file_name) => println!("Successfully wrote to the file {file_name}"),
        Err(err) => {
            eprintln!("Error writing to file: {err}");
            exit(1);
        }
    }

    println!("If you're seeing this file, everything worked!");
}

fn write_to_file() -> Result<String, std::io::Error> {
    println!("What would you like the file name to be?");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name)?;
    println!("What would you like to write to the file {}?", file_name.trim());
    let mut file_contents_to_write = String::new();
    stdin().read_line(&mut file_contents_to_write)?;

    fs::write(&file_name, &file_contents_to_write)?;
    return Ok(file_name);
}

fn read_file() -> Result<String, std::io::Error> {
    println!("Running the \"read_file\" function");

    println!("Enter the name of the file you would like to read.");
    let mut input = String::new();
    let requested_file = stdin().read_line(&mut input)?; //NOTE: This is an example of the ? operator. When we use this, if we get an error the function automatically returns it, otherwise it just assigns the Ok value to the variable!
    // if let Err(err) = requested_file {
    //     return Err(err);
    //     // eprintln!("Failed to read input --> Error: {err}");
    //     // exit(1);
    // }

    println!("Opening the File");
    let file = File::open(input.trim());
    if let Err(error) = file {
        return Err(error);
        // eprintln!("Failed to open file --> Error: {error}");
        // exit(1);
    }

    println!("Reading the File's Contents");
    let mut file_contents = String::new();
    let read_file = file.unwrap().read_to_string(&mut file_contents);
    if let Err(err) = read_file {
        return Err(err);
        // eprintln!("Failed to read the file contents --> Error: {err}");
        // exit(1);
    }

    // We can also do this instead of the stuff above:
    /*
    let mut file_contents = String::new();
    File::open(input.trim())?.read_to_string(&mut file_contents)?; // NOTE: We have the ? operator here twice!
     */

    println!("file_contents {file_contents}");
    return Ok(file_contents);
}

fn read_file2() -> Result<String, std::io::Error> {
    println!("The second read_file function!");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    return fs::read_to_string(input.trim()); // Reads the entirety of a file's contents into a string
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?; // This returns an Option (some or none)
    return Option::Some(last_element.len());
}
