#![cfg(feature = "cli")]
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;
use marc8::Decoder;

#[derive(Parser)]
struct Config {
    /// Path to a file in MARC-8 encoding.
    #[arg(default_value = "-")]
    path: Option<String>,
    /// Hide UTF-8 warnings. Default: false.
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}
// TODO: test output
// TODO: improve error handling
fn main() {
    let config = Config::parse();
    let mut decoder = Decoder::new(None, None, Some(config.quiet));

    if let Some(ref path) = config.path {
        let reader: Box<dyn BufRead> = match path.as_str() {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(File::open(path).unwrap())),
        };
        for line in reader.lines() {
            let bytes = line.unwrap().into_bytes();
            let result = decoder.decode(&bytes).unwrap();
            println!("{}", result);
        }
    }
}
