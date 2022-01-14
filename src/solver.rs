use std::collections::{HashMap, HashSet};

pub struct Solver {
    words: Vec<&'static str>,
    possible: HashMap<char, [bool; 5]>,
    present: HashSet<char>,
    confirmed: [Option<char>; 5],
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Self {
        let word_list = include_str!("word_list.txt");

        Self {
            words: word_list.lines().collect(),
            possible: HashMap::new(),
            present: HashSet::new(),
            confirmed: [None; 5],
        }
    }

    pub fn reset(&mut self) {
        self.possible.clear();
        self.present.clear();
        self.confirmed = [None; 5]
    }

    pub fn feedback(&mut self, guess: &str, feedback: &str) {
        let guess = guess.chars().collect::<Vec<_>>();
        let feedback = feedback.chars().collect::<Vec<_>>();

        self.present.clear();

        for i in 0..5 {
            match feedback[i] {
                'a' => {
                    if (0..5).any(|j| guess[j] == guess[i] && feedback[j] == 'p') {
                        self.possible.entry(guess[i]).or_insert([true; 5])[i] = false;
                    } else {
                        *self.possible.entry(guess[i]).or_default() = [false; 5];
                    }
                }
                'p' => {
                    self.possible.entry(guess[i]).or_insert([true; 5])[i] = false;
                    self.present.insert(guess[i]);
                }
                'c' => {
                    self.confirmed[i] = Some(guess[i]);
                }
                _ => panic!("Invalid feedback has been incorrectly handled by the program!"),
            }
        }
    }

    pub fn guess(&self) -> Option<String> {
        let mut present = HashSet::new();

        'words: for &word in &self.words {
            present.clear();

            'letters: for (i, c) in word.chars().enumerate() {
                if let Some(confirmed) = self.confirmed[i] {
                    if confirmed != c {
                        continue 'words;
                    }

                    continue 'letters;
                }

                if let Some(possible) = self.possible.get(&c) {
                    if !possible[i] {
                        continue 'words;
                    }
                }

                present.insert(c);
            }

            if self.present.is_subset(&present) {
                return Some(word.to_owned());
            }
        }

        None
    }
}
