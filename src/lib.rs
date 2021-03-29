use std::fs;
use std::process;

pub mod input {

    fn read_file(filename: &str) -> Result<String, &'static str> {
        match std::fs::read_to_string(filename) {
            Ok(s) => Ok(s),
            Err(e) => return Err("Shit!"),
        }
    }

    pub fn find_occurrences(word: &str, filename: &str) {
        let contents = read_file(filename)
            .expect("Something went wrong with reading the file");
        
        if contents.trim().is_empty() {
            println!("The file was empty!");
        }

        let mut count: i32 = 0;
        let mut modifier = String::new();

        for line in contents.lines() {
            if line.contains(&word) {
                count += 1;
            }
        }

        if count > 1 {
            modifier = String::from("s");
        }

        println!("The world {} occurred {} time{} in the text file.",
            word, count, modifier);
    }
}

pub mod service {

    pub fn print_usage() {
        println!("USAGE: occo WORD FILENAME")
    }
}