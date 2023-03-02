pub struct Hangman {
    attempts: u8,
    hidden_word: String,
    current_guess: String,
    wrong_letters: Vec<char>,
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
            wrong_letters: vec![],
        }
    }

    pub fn get_attempts(&self) -> u8 {
        self.attempts
    }

    pub fn get_hidden_word(&self) -> &String {
        &self.hidden_word
    }

    pub fn get_current_state(&self) -> &String {
        &self.current_guess
    }

    pub fn get_wrong_letters(&self) -> String {
        let mut wrong_letters = String::new();
        wrong_letters.push_str("[ ");
        for (i, el) in self.wrong_letters.iter().enumerate() {
            wrong_letters.push(*el);
            if i + 1 != self.wrong_letters.len() {
                wrong_letters.push_str(", ");
            }
        }
        wrong_letters.push_str(" ]");
        wrong_letters
    }

    pub fn update(&mut self, ltr: char) {
        if !ltr.is_ascii_alphabetic() {
            return;
        }
        let mut correct_ltr = false;
        for i in 0..self.hidden_word.len() {
            if self.hidden_word.chars().nth(i).unwrap() == ltr {
                correct_ltr = true;
                self.current_guess.remove(i);
                self.current_guess.insert(i, ltr);
            }
        }
        if !correct_ltr {
            if self.attempts != 0 {
                self.attempts -= 1;
            }
            self.wrong_letters.push(ltr);
        }
    }
}
