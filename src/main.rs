extern crate occo;

use std::env;

fn main() {
    // TODO: Implement clap arg parsing: https://github.com/clap-rs/clap


    let args: Vec<String> = env::args().collect();

    let config = occo::Config::new(&args);

    occo::find_occurrences(&config);
}
