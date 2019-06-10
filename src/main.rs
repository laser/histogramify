use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let input = strip_punctuation(read_from_stdin()?);
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut word_frequencies: HashMap<&str, u32> = HashMap::new();
    for word in words {
        word_frequencies
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut sorted_word_frequencies: Vec<(&str, u32)> = Vec::new();
    for (word, frequency) in word_frequencies {
        sorted_word_frequencies.push((word, frequency));
    }
    sorted_word_frequencies.sort_by(|a, b| { b.1.cmp(&a.1) });

    for (word, frequency) in sorted_word_frequencies {
        println!("{}: {}", word, frequency)
    }
    Ok(())
}

fn strip_punctuation(input: String) -> String {
    input
        .chars()
        .filter(|&c| c.is_alphabetic() || c.is_whitespace())
        .collect()
}

fn read_from_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
