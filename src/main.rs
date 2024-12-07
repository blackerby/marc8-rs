#![cfg(feature = "cli")]
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;
use marc8::Decoder;

#[derive(Parser)]
struct Config {
    #[arg(default_value = "-")]
    path: Option<String>,
}
// TODO: add ability to read from stdin
// TODO: test output
// TODO: add flag for quiet option
fn main() {
    let config = Config::parse();

    let file = File::open(config.path.unwrap()).unwrap();
    let reader = BufReader::new(file);
    let mut decoder = Decoder::new(None, None, None);

    for line in reader.lines() {
        let bytes = line.unwrap().into_bytes();
        let result = decoder.decode(&bytes).unwrap();
        println!("{}", result);
    }
}
