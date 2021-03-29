pub mod input {

    pub fn read_file(filename: &str) {
        println!("{}", format!("Im reading: {}", filename));
        

    }
}

pub mod service {

    pub fn print_usage() {
        println!("USAGE: occo WORD FILENAME")
    }
}