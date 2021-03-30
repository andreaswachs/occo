extern crate spinners;

use spinners::{Spinner, Spinners};
use std;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use terminal_size::{terminal_size, Height, Width};

mod args_helper {
    pub(super) fn print_usage() {
        println!("USAGE: occo WORD FILENAME")
    }
}

pub struct Config {
    pub word: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
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
    println!(
        "\n{}",
        (format!(
            "The word {} occurred {} time{} in the text file.",
            config.word, count, modifier
        ))
    );
}
