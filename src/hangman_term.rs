use std::io;
use std::io::Write;

use crate::Hangman;

pub const EXIT_KEY: &str = "q";

pub fn terminal(mut hangman: Hangman) {
    // Stores the user input
    let mut buffer = String::new();
    // Flag used to denote that the user has found the word
    let mut word_found = false;

    // Main game loop
    while !word_found && hangman.get_attempts() > 0 {
        // Clear the previous user input
        buffer.clear();
        // Display some information before getting user input
        println!(
            "Attempts: {}\nCurrent state: {}\nWrong letters: {}",
            hangman.get_attempts(),
            hangman.get_current_state(),
            hangman.get_wrong_letters()
        );
        // Input prompt
        print!("Input > ");
        // Get input
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("Try again! Couldn't understand you that time.");
            continue;
        }
        // Check if user wants to exit the game
        let trimmed_buf = buffer.trim();
        if trimmed_buf.eq(EXIT_KEY) {
            break;
        }
        // Process the user's guess
        if let Some(letter) = trimmed_buf.chars().next() {
            hangman.update(letter);
        } else {
            println!("Make sure to type a letter.");
            continue;
        }

        // Check if player found the word
        if hangman.is_word_found() {
            word_found = true;
        }
        println!();
    }
    // Display message depending on the user's result
    if word_found {
        println!("Well done! You've found the word!");
    } else {
        println!(
            "You didn't find the hidden word. :(\nThe hidden word was '{}'",
            hangman.get_hidden_word()
        );
    }
}
