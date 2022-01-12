use wordle_solver::bench;

fn main() {
    let results = bench();

    let total = results.len();
    let average = results.iter().sum::<usize>() / results.len();
    let most = results.iter().max().unwrap();
    let solved = results.iter().filter(|&&r| r <= 6).count();

    println!();
    println!("Benchmark for Wordle Solver {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!(
        "Solved {} out of {} ({}%) in 6 guesses or less.",
        solved,
        total,
        solved * 100 / total
    );
    println!("Failed to solve {} in 6 guesses or less.", total - solved);
    println!("Average number of guesses needed: {}.", average);
    println!("Most number of guesses needed: {}.", most);
    println!();
}
