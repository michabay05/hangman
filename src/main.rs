mod hangman;

use crate::hangman::Hangman;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const WORD_FILE: &str = "words.txt";

fn gen_random_word(path: &str) -> String {
    let mut rng = rand::thread_rng();
    let content = fs::read_to_string(path);
    if let Err(err) = content {
        panic!("Failed to find specified file.\nERROR: {}", err);
    }
    let content = content.unwrap();
    let mut split_words = content.split('\n');
    let rand_ind = rng.gen_range(0..split_words.clone().count());
    let rand_word = split_words.nth(rand_ind);
    if rand_word.is_none() {
        return gen_random_word(path);
    }
    rand_word.unwrap().to_string()
}

fn main() {
    let mut h = Hangman::new(gen_random_word(WORD_FILE));

    let mut buffer = String::new();
    while h.get_attempts() > 0 {
        buffer.clear();
        println!(
            "Attempts: {}\nCurrent state: {}\nWrong letters: {}",
            h.get_attempts(),
            h.get_current_state(),
            h.get_wrong_letters()
        );
        print!("Input > ");
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("Try again! Couldn't understand you that time.");
            continue;
        }
        let trimmed_buf = buffer.trim();
        if trimmed_buf.eq("q") {
            break;
        }
        if let Some(letter) = trimmed_buf.chars().next() {
            h.update(letter);
        } else {
            println!("Make sure to type a letter.");
            continue;
        }
        println!();
    }
    println!("The hidden word was {}", h.get_hidden_word());
}
