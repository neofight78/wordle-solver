use std::ops::{Index, IndexMut};

pub struct CharArray<T>([T; 26]);

impl<T> Index<char> for CharArray<T> {
    type Output = T;

    fn index(&self, index: char) -> &Self::Output {
        &self.0[index as usize - 'a' as usize]
    }
}

impl<T> IndexMut<char> for CharArray<T> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        &mut self.0[index as usize - 'a' as usize]
    }
}

impl<T: Copy> CharArray<T> {
    fn new(default: T) -> Self {
        CharArray([default; 26])
    }
}

impl CharArray<bool> {
    fn is_subset(&self, other: &Self) -> bool {
        for i in 'a'..='z' {
            if self[i] && !other[i] {
                return false;
            }
        }

        true
    }
}

pub struct Solver {
    words: Vec<&'static str>,
    possible: CharArray<[bool; 5]>,
    present: CharArray<bool>,
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
            possible: CharArray::new([true; 5]),
            present: CharArray::new(false),
            confirmed: [None; 5],
        }
    }

    pub fn reset(&mut self) {
        self.possible = CharArray::new([true; 5]);
        self.present = CharArray::new(false);
        self.confirmed = [None; 5]
    }

    pub fn feedback(&mut self, guess: &str, feedback: &str) {
        let guess = guess.chars().collect::<Vec<_>>();
        let feedback = feedback.chars().collect::<Vec<_>>();

        self.present = CharArray::new(false);

        for i in 0..5 {
            match feedback[i] {
                'a' => {
                    if (0..5).any(|j| guess[j] == guess[i] && feedback[j] == 'p') {
                        self.possible[guess[i]][i] = false;
                    } else {
                        self.possible[guess[i]] = [false; 5];
                    }
                }
                'p' => {
                    self.possible[guess[i]][i] = false;
                    self.present[guess[i]] = true;
                }
                'c' => {
                    self.confirmed[i] = Some(guess[i]);
                }
                _ => panic!("Invalid feedback has been incorrectly handled by the program!"),
            }
        }
    }

    pub fn guess(&self) -> Option<String> {
        'words: for &word in &self.words {
            let mut present = CharArray::new(false);

            'letters: for (i, c) in word.chars().enumerate() {
                if let Some(confirmed) = self.confirmed[i] {
                    if confirmed != c {
                        continue 'words;
                    }

                    continue 'letters;
                }

                if !self.possible[c][i] {
                    continue 'words;
                }

                present[c] = true;
            }

            if self.present.is_subset(&present) {
                return Some(word.to_owned());
            }
        }

        None
    }
}
