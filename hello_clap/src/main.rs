use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Cli {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // print value from inputs
    if let Some(arg_from_cli) = cli.name {
        println!("you're ... {arg_from_cli}");
    }

    // match has better error handling but i'm not interested in None Option
    // match cli.name {
    //     Some(name) => println!("you're ... {name}!!"),
    //     None => println!("no value was provided"),
    // }
}
