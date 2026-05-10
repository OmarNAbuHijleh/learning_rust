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
 */
use std::process::exit;
fn main() {
    println!("The panic macro");
    // None.unwrap(); // This will cause a panic, as it only works on the "Some" variant and not the "None" variant
    // panic!("Something went wrong"); // This is an example of a panic

    println!("The process module and the exit function");
    exit(0); // This will exit the program gracefully
    println!("This will not be printed"); // This will throw a warning from the compiler, as it is unreachable

    println!("Standard Error (eprintln! macro)");
}
