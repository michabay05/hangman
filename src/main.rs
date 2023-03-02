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
    if let None = rand_word {
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
        if let Err(_) = io::stdin().read_line(&mut buffer) {
            println!("Try again! Couldn't understand you that time.");
            continue;
        }
        let trimmed_buf = buffer.trim();
        if trimmed_buf.eq("q") {
            break;
        }
        if let Some(letter) = trimmed_buf.chars().nth(0) {
            h.update(letter);
        } else {
            println!("Make sure to type a letter.");
            continue;
        }
        println!();
    }
    println!("The hidden word was {}", h.get_hidden_word());
}
