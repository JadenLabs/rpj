mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New(cmd) => cmd.handle(),
        Commands::Remove(cmd) => cmd.handle(),
        Commands::Update(cmd) => cmd.handle(),
    }
}
