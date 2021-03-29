extern crate spinners;

pub mod input {
    use std;
    use std::io::prelude::*;
    use std::fs::File;
    use spinners::{Spinner, Spinners};

    pub fn find_occurrences(word: &str, filename: &str) {

        let mut count: i32 = 0;
        let mut modifier = String::new();

        let file = File::open(filename)
        .expect("err");
        let reader = std::io::BufReader::new(file);

        let sp = Spinner::new(Spinners::SimpleDots, "Looking for occurrences...".into());
        for line in reader.lines() {
            match line {
                Ok(text) => {
                    if text.contains(&word) {
                        count += 1;
                        sp.message(format!("Found {} occurrences so far", count).into());
                    }
                    
                },
                Err(_) => { /* We are just ignoring errors at the moment.. */ }
            }
        }
        
        if count > 1 {
            modifier = String::from("s");
        }
        sp.stop();
        println!("\n{}", (format!("The word {} occurred {} time{} in the text file.",
            word, count, modifier)));
    }
}

pub mod service {

    pub fn print_usage() {
        println!("USAGE: occo WORD FILENAME")
    }
}