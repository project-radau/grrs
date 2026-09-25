// https://rust-cli.github.io/book/tutorial/impl-draft.html

use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use anyhow::{Context};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))
        .unwrap();

    let reader = BufReader::new(file);

    // Gehe jede Zeile der Datei einzeln durch
    for line in reader.lines() {
        let line = line?; // Liest die Zeile ein und fängt eventuelle I/O-Fehler ab
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}


