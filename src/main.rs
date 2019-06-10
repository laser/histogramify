use std::collections::HashMap;
use std::io::{self, Read};
use std::sync::{Mutex, Arc};
use std::thread;

fn main() -> Result<(), io::Error> {
    let input = strip_punctuation(read_from_stdin()?);
    let words: Vec<&str> = input.split_whitespace().collect();
    for (word, frequency) in sort(word_frequencies(words)) {
        println!("{}: {}", word, frequency)
    }
    Ok(())
}

const THREADS: u8 = 4;

fn word_frequencies(words: Vec<&str>) -> HashMap<&str, u32> {
    let mut word_frequencies: Arc<Mutex<HashMap<&str, u32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    let words_for_threads = words.chunks(words.len() / THREADS as usize);
    for words in words_for_threads {
        let word_frequencies = Arc::clone(&word_frequencies);
        let handle = thread::spawn(move || {
            for word in words {
                word_frequencies
                    .lock()
                    .unwrap()
                    .entry(word)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *word_frequencies.lock().unwrap()
}

fn sort(word_frequencies: HashMap<&str, u32>) -> Vec<(&str, u32)> {
    let mut sorted_word_frequencies: Vec<(&str, u32)> = vec![];
    for (word, frequency) in word_frequencies {
        sorted_word_frequencies.push((word, frequency));
    }
    sorted_word_frequencies.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_word_frequencies
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
