use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Get the file path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-file>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Open the file for reading
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Create a HashMap to store word counts
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    // Read lines from the file and count words
    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let word = word.to_lowercase(); // Convert word to lowercase for case-insensitive counting
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }

    // Sort the words by frequency in descending order
    let mut sorted_counts: Vec<(&String, &usize)> = word_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

    // Print out the sorted word frequencies
    for (word, count) in sorted_counts {
        println!("{}: {}", word, count);
    }

    Ok(())
}
