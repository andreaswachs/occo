extern crate spinners;

use spinners::{Spinner, Spinners};
use std;
use std::fs::File;
use std::io::prelude::*;
use std::process;

mod args_helper {
    pub(super) fn print_usage() {
        println!("USAGE: occo WORD FILENAME")
    }
}

mod helper_functions {
    use terminal_size::{terminal_size, Height, Width};

    
    pub(super) fn clear_terminal_line() {
        let size = terminal_size();
        if let Some((Width(w), Height(_))) = size {
            // return the carriage to the beginning of the line
            print!("\r");

            // Print spaces to clear the line
            for _ in 0..w - 1 {
                print!(" ");
            }

            print!("\r");
        }
    }
}

pub struct Config {
    pub word: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Self {
        if args.len() != 3 {
            println!("Insufficient number of arguments passed to occo.");
            args_helper::print_usage();
            process::exit(-1);
        }

        let word = args[1].clone();
        let filename = args[2].clone();

        if word.trim().is_empty() || filename.trim().is_empty() {
            println!("Either the search word or the filename was not given");
            args_helper::print_usage();
            process::exit(-2);
        }

        Self { word, filename }
    }
}
pub fn find_occurrences(config: &Config) {
    let mut count: i32 = 0;
    let mut modifier = String::new();

    let file = File::open(&config.filename).expect("err");
    let reader = std::io::BufReader::new(file);

    let sp = Spinner::new(Spinners::SimpleDots, "Looking for occurrences...".into());
    for line in reader.lines() {
        match line {
            Ok(text) => {
                if text.contains(&config.word) {
                    count += 1;
                    sp.message(format!("Found {} occurrences so far", count).into());
                }
            }
            Err(_) => { /* We are just ignoring errors at the moment.. */ }
        }
    }

    if count > 1 {
        modifier = String::from("s");
    }
    sp.stop();

    helper_functions::clear_terminal_line();
    println!(
        "{}",
        (format!(
            "The word {} occurred {} time{} in the text file.",
            config.word, count, modifier
        ))
    );
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn create_new_config_config_should_be_valid() {
        // Arrange
        let word:String = String::from("bob");
        let filename:String = String::from("manuscript.txt");
        let args = vec!["occo".to_string(), word.clone(), filename.clone()];

        // Act
        let config = Config::new(&args);

        // Assert
        assert_eq!(filename, config.filename);
        assert_eq!(word, config.word);

    }
}