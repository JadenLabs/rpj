use crate::utils::{
    get_project_by_name, get_store_path, run_terminal,
};
use colored::Colorize;

#[derive(clap::Args)]
pub struct RunCommand {
    pub name: String,
    #[clap(long, short = 't', help = "The terminal to run the command in")]
    pub terminal: Option<String>,
}

impl RunCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path()?;
        let project = get_project_by_name(&store_path, &self.name)?;

        let run_cmd = project.run_cmd.as_deref().ok_or_else(|| {
            format!(
                "Project {} does not have a run command defined. Use {} to update it.",
                self.name.red(),
                "rpj update <name> --run-cmd [command]".dimmed()
            )
        })?;

        println!(
            "{} {} {}{}{}\n",
            "ℹ Running".blue(),
            &self.name.blue().bold(),
            "using the '".blue(),
            &terminal.blue().bold(),
            "' terminal.".blue()
        );

        run_terminal(&project.directory, &self.terminal, &run_cmd)?;

        Ok(())
    }
}
