use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;
use marc8::Marc8;

// TODO: change path type to PathBuf?
// TODO: add arg for g0
// TODO: add arg for g1
#[derive(Parser)]
struct Cli {
    /// Path to a file in MARC-8 encoding.
    #[arg(default_value = "-")]
    path: Option<String>,
    /// Hide UTF-8 warnings. Default: false.
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}
// TODO: test reading from stdin
// TODO: test reading from file
// TODO: improve error handling
fn main() {
    let cli = Cli::parse();
    let mut converter = Marc8::new(None, None, Some(cli.quiet));

    if let Some(ref path) = cli.path {
        let reader: Box<dyn BufRead> = match path.as_str() {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(File::open(path).unwrap())),
        };
        for line in reader.lines() {
            let bytes = line.unwrap().into_bytes();
            let result = converter.convert(&bytes).unwrap();
            println!("{}", result);
        }
    }
}
