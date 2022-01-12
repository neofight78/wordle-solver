pub struct Puzzler(Vec<char>);

impl Puzzler {
    pub fn new(word: &str) -> Self {
        Self(word.chars().collect())
    }

    pub fn feedback(&self, guess: &str) -> String {
        let guess = guess.chars().collect::<Vec<_>>();

        let mut feedback = String::new();

        for (i, &c) in guess.iter().enumerate() {
            if self.0[i] == c {
                feedback.push('c');
                continue;
            }

            if !self.0.contains(&c) {
                feedback.push('a');
                continue;
            }

            let total = self.0.iter().filter(|&&a| a == c).count();
            let correct = guess
                .iter()
                .enumerate()
                .filter(|&(i, &g)| g == c && g == self.0[i])
                .count();
            let present_so_far = guess[0..i]
                .iter()
                .enumerate()
                .filter(|&(i, &g)| g == c && g != self.0[i])
                .count();

            feedback.push(if correct + present_so_far < total {
                'p'
            } else {
                'a'
            });
        }

        feedback
    }
}
