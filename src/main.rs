use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;

#[derive(Parser, Debug)]
struct Head {
    /// File to read
    pub file: String,
}

fn main() {
    let args = Head::parse();

    let file = File::open(args.file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
