use crate::utils::{get_project_by_name, get_store_path};
use colored::Colorize;

#[derive(clap::Args)]
pub struct UpdateCommand {
    pub name: String,
    #[arg(long, help = "Directory of the project")]
    pub directory: Option<String>,
    #[arg(long, help = "Command to run the project")]
    pub run_cmd: Option<String>,
    #[arg(long, help = "GitHub URL for the project")]
    pub gh_url: Option<String>,
    #[arg(long, help = "Description of the project")]
    pub description: Option<String>,
}

impl UpdateCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        if self.directory.is_none()
            && self.run_cmd.is_none()
            && self.gh_url.is_none()
            && self.description.is_none()
        {
            return Err("No fields provided to update.".into());
        }

        // Get the RPJ store path and the project
        let store_path = get_store_path()?;
        let mut project = get_project_by_name(&store_path, &self.name)?;

        // Update the project fields
        if let Some(directory) = self.directory {
            project.directory = directory.clone();
            println!(
                "{} Updated directory to: {}",
                "→".blue(),
                directory.dimmed()
            );
        }
        if let Some(run_cmd) = self.run_cmd {
            project.run_cmd = Some(run_cmd.clone());
            println!(
                "{} Updated run command to: {}",
                "→".blue(),
                run_cmd.dimmed()
            );
        }
        if let Some(gh_url) = self.gh_url {
            project.gh_url = Some(gh_url.clone());
            println!("{} Updated GitHub URL to: {}", "→".blue(), gh_url.dimmed());
        }
        if let Some(description) = self.description {
            project.description = Some(description.clone());
            println!(
                "{} Updated description to: {}",
                "→".blue(),
                description.dimmed()
            );
        }

        // Save the updated project back to the RPJ store
        project.save(&store_path);

        println!(
            "{} {} {}",
            "✔ Project".green(),
            self.name.blue(),
            "has been updated".green()
        );

        Ok(())
    }
}
