use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]

struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // TODO: use if let instead
    // print value from inputs
    match cli.name {
        Some(name) => println!("you're ... {name}!!"),
        None => println!("no value was provided"),
    }
}
