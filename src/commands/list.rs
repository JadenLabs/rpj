use crate::utils::{get_store_path, load_projects};
use colored::Colorize;

#[derive(clap::Args)]
pub struct ListCommand {}

impl ListCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path()?;
        let projects = load_projects(&store_path);

        if projects.is_empty() {
            return Err("No projects found.".into());
        } else {
            println!("{}", "Projects:".bold().green());
            for project in projects {
                println!(
                    "→ {} {}",
                    project.name.bright_cyan().bold(),
                    format!("({})", project.directory).dimmed()
                );
            }
        }

        Ok(())
    }
}
