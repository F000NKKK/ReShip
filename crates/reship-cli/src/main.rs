#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "reship", version, about = "ReShip release distribution CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Reserved for the first publisher implementation.
    Publish,
    /// Reserved for channel promotion without rebuilding a release.
    Promote,
    /// Inspect server/channel state.
    Inspect,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Publish => println!("publish is defined by the v0.1 architecture and implemented in a follow-up RS task"),
        Command::Promote => println!("promote is defined by the v0.1 architecture and implemented in a follow-up RS task"),
        Command::Inspect => println!("inspect is defined by the v0.1 architecture and implemented in a follow-up RS task"),
    }
}
