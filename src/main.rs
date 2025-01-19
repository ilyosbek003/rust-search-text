use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

/// A simple command-line tool for searching patterns in files
#[derive(Parser)]
#[command(name = "rust-v1")]
#[command(about = "A simple grep-like tool built in Rust", long_about = None)]
struct Cli {
    /// The pattern to search for
    pattern: String,

    /// The path to the file or directory to search in
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let pattern = Regex::new(&args.pattern).expect("Invalid regex pattern");
    let path = Path::new(&args.path);

    if path.is_file() {
        search_in_file(path, &pattern)?;
    } else if path.is_dir() {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.path().is_file() {
                search_in_file(entry.path(), &pattern)?;
            }
        }
    } else {
        eprintln!("Error: Path '{}' is neither a file nor a directory.", args.path);
    }

    Ok(())
}

/// Search for the pattern in a single file
fn search_in_file(path: &Path, pattern: &Regex) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if pattern.is_match(&line) {
            println!(
                "{}:{}: {}",
                path.display(),
                line_number + 1,
                line
            );
        }
    }

    Ok(())
}
