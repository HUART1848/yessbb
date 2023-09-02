use rand::seq::{IteratorRandom, SliceRandom};

use crate::utils::{self, flushed_print};

static RAW_LIST: &str = include_str!("../words.txt");

const DEFAULT_MAX_WORD_SIZE: usize = 5;
const DEFAULT_DURATION:u64 = 5;

pub struct WordPuzzle {
    max_word_size: usize,
    duration: std::time::Duration,
}

impl WordPuzzle {
    pub fn new(max_word_size: usize, seconds: u64) -> Self {
        return Self {
            max_word_size,
            duration: std::time::Duration::from_secs(seconds),
        };
    }

    fn default_list() -> &'static str {
        return RAW_LIST;
    }

    fn default_filter(&self, word: &str) -> bool {
        if word.len() <= self.max_word_size && word.chars().all(|c| c.is_ascii_alphabetic()) {
            return true;
        }

        return false;
    }

    fn shuffle_word(word: &str) -> String {
        let mut slice: Vec<char> = word.chars().collect();
        slice.shuffle(&mut rand::thread_rng());

        return slice.iter().collect();
    }

    pub fn get_word(&self) -> String {
        let lines = Self::default_list()
            .lines()
            .filter(|w| self.default_filter(w));
        let word = lines
            .choose(&mut rand::thread_rng())
            .expect("could not obtain a word");

        return word.to_owned();
    }

    pub fn puzzle(&self) -> bool {
        let target = self.get_word();
        let scrambled = Self::shuffle_word(target.as_ref());

        println!("What is this word ? You have {} seconds:", self.duration.as_secs());
        flushed_print(scrambled.as_str());
        std::thread::sleep(self.duration);
        
        utils::flushed_print("\rYou answer: ");
        let mut attempt = String::new();
        std::io::stdin().read_line(&mut attempt).expect("could not read the answer");

        if target.eq(&attempt) {
            println!("Well done!");
            return true;
        }

        println!("Wrong! The word was {}", target);
        return false;
    }
}

impl Default for WordPuzzle {
    fn default() -> Self {
        return Self::new(DEFAULT_MAX_WORD_SIZE, DEFAULT_DURATION);
    }
}
git 