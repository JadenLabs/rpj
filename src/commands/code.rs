use colored::Colorize;

use crate::utils::{get_project_by_name, get_store_path};
use std::process::Command;

#[derive(clap::Args)]
pub struct CodeCommand {
    pub name: String,
}

impl CodeCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path()?;
        let project = get_project_by_name(&store_path, &self.name)?;

        println!(
            "{} {} {} ({})",
            "ℹ Opening project".blue(),
            project.name.bold().blue(),
            "in VS Code...".blue(),
            &project.directory.dimmed(),
        );

        launch_vscode(&project.directory)?;

        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn launch_vscode(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cmd")
        .args(["/C", "code", dir])
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Failed to open VS Code: {e}").into())
}

#[cfg(not(target_os = "windows"))]
fn launch_vscode(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("sh")
        .args(["-c", &format!("code {dir}")])
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Failed to open VS Code: {e}").into())
}
