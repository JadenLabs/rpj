use crate::utils::{get_project_by_name, get_store_path, run_terminal};
use colored::Colorize;

#[derive(clap::Args)]
pub struct TerminalCommand {
    pub name: String,
    #[clap(long, short = 't', help = "The terminal to run the command in")]
    pub terminal: Option<String>,
}

impl TerminalCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path()?;
        let project = get_project_by_name(&store_path, &self.name)?;

        let ter_str = &self.terminal.clone().unwrap_or("default".into());
        println!(
            "{} {} {}{}{}\n",
            "ℹ Running".blue(),
            &self.name.blue().bold(),
            "using the ".blue(),
            ter_str.blue().bold(),
            " terminal.".blue()
        );

        run_terminal(&project.directory, self.terminal.as_ref(), None)?;

        Ok(())
    }
}
