use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "CLI for interacting with a PiMesh System"
)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    User,
}

fn main() {
    let cli = Cli::parse();

    match cli.debug {
        1 => println!("Debug Low"),
        2 => println!("Debug High"),
        _ => {},
    }

    match &cli.command {
        Some(Commands::User) => {
            println!("User Stuff")
        },
        None => {},
    }

}
