mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

fn main() {
    let cli = Cli::parse();

    let res: Result<(), Box<dyn std::error::Error>> = match cli.command {
        Commands::New(cmd) => cmd.handle(),
        Commands::Remove(cmd) => cmd.handle(),
        Commands::Update(cmd) => cmd.handle(),
        Commands::Debug(cmd) => cmd.handle(),
        Commands::Export(cmd) => cmd.handle(),
        Commands::Add(cmd) => cmd.handle(),
        Commands::Install(cmd) => cmd.handle(),
        Commands::List(cmd) => cmd.handle(),
        Commands::Get(cmd) => cmd.handle(),
        Commands::Code(cmd) => cmd.handle(),
        Commands::Run(cmd) => cmd.handle(),
        Commands::Explore(cmd) => cmd.handle(),
        Commands::Path(cmd) => cmd.handle(),
        Commands::Terminal(cmd) => cmd.handle(),
    };

    if let Err(e) = res {
        eprintln!("{} {}", "✖".red(), e);
        std::process::exit(1);
    }
}
