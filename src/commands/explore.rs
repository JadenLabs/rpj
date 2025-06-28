use crate::utils::{get_store_path, get_project_by_name};
use std::{process::Command};


#[derive(clap::Args)]
pub struct ExploreCommand {
    pub name: String,
}

impl ExploreCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path()?;
        let project = get_project_by_name(&store_path, &self.name)?;

        if cfg!(target_os = "windows") {
            run_explorer_on_windows(&project.directory)?;
        } else {
            run_explorer_on_unix(&project.directory)?;
        }

        Ok(())
    }
}

fn run_explorer_on_windows(directory: &String) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("explorer").arg(directory).spawn()?.wait()?;
    Ok(())
}

fn run_explorer_on_unix(directory: &String) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("xdg-open").arg(directory).spawn()?.wait()?;
    Ok(())
}
