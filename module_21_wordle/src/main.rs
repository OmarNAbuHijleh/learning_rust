/*
 * Wordle is a game where a person gets 6 chances to guess a 5 letter word. On each attempt, they are informed of which letters they got right at which positions
 * If a character is correct and in the right position - it's designated as such (green). If a character exists but is in the wrong position, it's indicated as such (yellow). If a character is not at all in the word, it's indicated as such (black)
*/

use colored::Colorize;
use std::io;

fn main() {
    let target_word = "trait";
    for attempt in 1..6{ // 6 attempts
        println!("Input a 5 letter word. Attempt number {attempt}");
        let input = io::stdin();

        let mut user_input = String::new();
        input
            .read_line(&mut user_input)
            .expect("Failed to provide input");
        let mut no_errors = true;
        for (target_character, user_character) in target_word.chars().zip(user_input.chars()) {
            if target_character == user_character {
                print!("{}|", format!(" {user_character} ").on_green());
            } else if target_word.contains(user_character) {
                no_errors = false;
                print!("{}|", format!(" {user_character} ").on_yellow());
            } else {
                no_errors = false;
                print!("{}|", format!(" {user_character} ").on_black());
            }
        }
        if no_errors {
            println!("\nYou win!");
            break;
        }
        println!();
    }
}
