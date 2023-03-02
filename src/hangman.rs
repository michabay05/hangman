pub struct Hangman {
    attempts: u8,
    hidden_word: String,
    current_guess: String,
}

impl Hangman {
    pub fn new(hidden_word: String) -> Self {
        let mut current_guess = String::new();
        for _ in 0..hidden_word.len() {
            current_guess.push('-');
        }
        Self {
            attempts: 6,
            hidden_word,
            current_guess,
        }
    }

    pub fn update(&mut self, ltr: char) {
        let mut correct_ltr = false;
        for i in 0..self.hidden_word.len() {
            if self.hidden_word.chars().nth(i).unwrap() == ltr {
                correct_ltr = true;
                self.current_guess.remove(i);
                self.current_guess.insert(i, ltr);
            }
        }
        if !correct_ltr {
            self.attempts -= 1;
        }
    }
}
