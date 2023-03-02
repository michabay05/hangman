mod hangman;

use crate::hangman::Hangman;
use rand::Rng;
use std::fs;

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
    let rand_word = gen_random_word(WORD_FILE);
    let h = Hangman::new(rand_word);
}
