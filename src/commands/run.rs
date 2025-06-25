use colored::Colorize;

use crate::utils::{get_project_by_name, get_store_path};
use std::process::Command;

#[derive(clap::Args)]
pub struct RunCommand {
    pub name: String,
    #[clap(long, short = 't', help = "The terminal to run the command in")]
    pub terminal: Option<String>
}

impl RunCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path();
        let project = get_project_by_name(&store_path, &self.name)?;

        // Check if there is a run command
        if let None = project.run_cmd {
            return Err(format!(
                "Project {} does not have a run command defined. Use {} to update it.",
                self.name.red(),
                "rpj update <name> --run-cmd [command]".dimmed()
            )
            .into());
        }

        // to be implemented

        Ok(())
    }
}

fn run_terminal_on_windows(directory: &str, terminal: &str, command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("cmd")
        .args(&["/C", "start", terminal, "/K", command])
        .current_dir(directory)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to run command in terminal: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
