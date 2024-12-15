use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;
use marc8::{Marc8, Marc8Error};

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
fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let mut converter = Marc8::new(None, None, Some(cli.quiet));

    if let Some(ref path) = cli.path {
        let reader: Box<dyn BufRead> = match path.as_str() {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(File::open(path)?)),
        };
        for line in reader.lines() {
            let bytes = line?.into_bytes();
            let result = converter.convert(&bytes)?;
            println!("{result}");
        }
    }

    Ok(())
}

#[derive(Debug)]
enum Error {
    Marc8Error(Marc8Error),
    IoError(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Marc8Error(e) => e.fmt(f),
            Self::IoError(e) => e.fmt(f),
        }
    }
}

impl From<Marc8Error> for Error {
    fn from(value: Marc8Error) -> Self {
        Self::Marc8Error(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
