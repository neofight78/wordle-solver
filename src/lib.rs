mod puzzler;
mod solver;

pub use puzzler::Puzzler;
pub use solver::Solver;

pub fn bench() -> Vec<usize> {
    let mut guess_counts = Vec::new();
    let mut solver = Solver::new();

    let words = include_str!("word_list.txt").lines();

    'word: for word in words {
        let puzzler = Puzzler::new(word);
        solver.reset();

        for i in 1..=26 {
            if let Some(guess) = solver.guess() {
                if guess == word {
                    guess_counts.push(i);
                    continue 'word;
                }
                solver.feedback(&guess, &puzzler.feedback(&guess));
            } else {
                panic!("Failed to solve for {}", word);
            }
        }

        panic!("Failed to solve for {}", word);
    }

    guess_counts
}
