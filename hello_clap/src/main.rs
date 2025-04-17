use anyhow::{Context, Result};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

// main program
fn main() -> Result<()> {
    // parsing arguments with clap
    let args = Cli::parse();
    // reading path argument and error handling with anyhow (with_context)
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // checks lines that contain the argument
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // returning success?
    Ok(())
}
