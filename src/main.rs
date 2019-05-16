extern crate clap;

use clap::{App, Arg};
use std::fs::File;

fn main() {
    let matches = App::new("bc")
        .version("1.0")
        .about("Counts bytes in files")
        .author("Ben Boyter")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input files to byte count")
            .required(true)
            .multiple(true)
            .index(1))
        .get_matches();


    let dir_names: Vec<&str> = matches.values_of("INPUT").unwrap().collect();

    for (_size, &str) in dir_names.iter().enumerate() {
        println!("{} {}", str, count_file(str));
    }
}

fn count_file(file: &str) -> u64 {
    match File::open(file) {
        Ok(x) => {
            match x.metadata() {
                Ok(y) => y.len(),
                _ => 0,
            }
        }
        _ => 0
    }
}