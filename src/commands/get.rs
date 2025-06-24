use crate::utils::{get_project_by_name, get_store_path};
use colored::Colorize;

#[derive(clap::Args)]
pub struct GetCommand {
    pub name: String,
}

impl GetCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path();
        let project = get_project_by_name(&store_path, &self.name)?;

        println!(
            "{} {}\n",
            "Project:".bold(),
            project.name.blue().bold().underline()
        );

        println!("{} {}", "Directory:".bold(), project.directory.dimmed());

        if let Some(description) = project.description {
            println!("{} {}", "Description:".bold(), description.dimmed());
        }

        if let Some(run_cmd) = project.run_cmd {
            println!("{} {}", "Run command:".bold(), run_cmd.dimmed());
        }

        if let Some(gh_url) = project.gh_url {
            println!("{} {}", "GitHub URL:".bold(), gh_url.dimmed());
        }

        Ok(())
    }
}
