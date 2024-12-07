#![cfg(feature = "cli")]
use std::fs::File;
use std::io::Read;

use clap::Parser;
use marc8::Decoder;

#[derive(Parser)]
struct Config {
    #[arg(default_value = "-")]
    path: Option<String>,
}
// TODO: add ability to read from stdin
// TODO: test output
fn main() {
    let config = Config::parse();

    let mut reader = File::open(config.path.unwrap()).unwrap();
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    let mut decoder = Decoder::new(None, None, None);
    let result = decoder.decode(&buffer).unwrap();

    println!("{result}");
}
