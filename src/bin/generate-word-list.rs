use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), std::io::Error> {
    let frequency_list = read_frequency_list()?;

    let mut wordle_dictionary = read_wordle_dictionary()?;
    wordle_dictionary.sort_unstable_by_key(|w| frequency_list.get(w).unwrap_or(&0));
    wordle_dictionary.reverse();

    let mut word_list = File::create("src/word_list.txt")?;

    for word in wordle_dictionary {
        writeln!(word_list, "{}", word)?;
    }

    Ok(())
}

fn read_frequency_list() -> Result<HashMap<String, u64>, std::io::Error> {
    let file = File::open("frequency_list.txt")?;
    let reader = BufReader::new(file);

    let mut frequency_list = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut fields = line.split_whitespace();
        let frequency = fields.next().unwrap().parse::<u64>().unwrap();
        let word = fields.next().unwrap();

        if word.len() == 5 {
            *frequency_list.entry(word.to_owned()).or_insert(0) += frequency;
        }
    }

    Ok(frequency_list)
}

fn read_wordle_dictionary() -> Result<Vec<String>, std::io::Error> {
    let file = File::open("wordle_dictionary.txt")?;
    let reader = BufReader::new(file);

    let mut wordle_dictionary = Vec::new();

    for line in reader.lines() {
        wordle_dictionary.push(line?);
    }

    Ok(wordle_dictionary)
}
