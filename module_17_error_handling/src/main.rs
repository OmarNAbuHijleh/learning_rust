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
 *
 */
use std::process::exit;
use std::fs::File;
use std::io::stdin;
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
    let file = match File::open(input.trim()) { // The "input.trim()" removes the newline character from the input
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to Open File --> Error: {err}");
            exit(1)
        }
    };
    println!("{:#?}", file);

    println!("Reading the File's Contents");
}
