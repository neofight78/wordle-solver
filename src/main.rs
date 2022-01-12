use std::io::{stdin, stdout, Error, Write};

use wordle_solver::Solver;

fn main() -> Result<(), Error> {
    println!();
    println!("Wordle Solver version {}", env!("CARGO_PKG_VERSION"));
    println!("Enter exit to finish, or restart to start again.");

    let mut solver = Solver::new();

    'main: loop {
        println!();
        let guess = solver.guess();

        if guess.is_none() {
            println!("Failed to find a solution (was the feedback correct?). Trying again...");
            solver.reset();
            continue;
        }

        let guess = guess.unwrap();

        println!("My guess: {}.", guess);
        print!("Please enter feedback as five characters (a - absent, p - present, c - correct): ");
        stdout().flush()?;

        let mut input = String::new();

        loop {
            input.clear();
            stdin().read_line(&mut input)?;

            if input.is_empty() {
                break 'main;
            }

            let input = input.trim();

            match input {
                "ccccc" => break 'main,
                "exit" | "q" | "quit" => break 'main,
                "restart" => {
                    solver.reset();
                    continue 'main;
                }
                _ if input.len() == 5 && input.chars().all(|c| ['a', 'p', 'c'].contains(&c)) => {
                    break
                }
                _ => (),
            }

            print!("Invalid input, please try again: ");
            stdout().flush()?;
        }

        solver.feedback(&guess, &input);
    }

    println!();
    Ok(())
}
