use crate::utils::{
    Project, ProjectExistsResult, get_store_path, load_projects, normalize_path, project_exists,
    save_projects,
};
use colored::Colorize;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct NewCommand {
    pub name: String,
    pub directory: String,
    #[arg(long, help = "Command to run the project")]
    pub run_cmd: Option<String>,
    #[arg(long, help = "GitHub URL for the project")]
    pub gh_url: Option<String>,
    #[arg(long, help = "Description of the project")]
    pub description: Option<String>,
}

impl NewCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the RPJ store path
        let store_path = get_store_path()?;

        // Check if the project already exists in the RPJ store
        let project_path = normalize_path(&PathBuf::from(&self.directory))?;
        match project_exists(&store_path, self.name.as_str(), Some(&project_path)) {
            ProjectExistsResult::ExistsByName => {
                return Err(
                    format!("Project '{}' already exists in the RPJ store!", self.name).into(),
                );
            }
            ProjectExistsResult::ExistsByDirectory => {
                return Err(format!(
                    "Project path '{}' already exists in the RPJ store!",
                    self.directory
                )
                .into());
            }
            _ => {}
        }

        // Create a new project instance
        let project = Project {
            name: self.name.clone(),
            directory: project_path.to_string_lossy().to_string(),
            run_cmd: self.run_cmd,
            gh_url: self.gh_url,
            description: self.description,
        };

        // Load existing projects from the RPJ store
        let mut projects = load_projects(&store_path);

        projects.push(project);
        save_projects(&store_path, &projects);

        println!(
            "{} {} {}",
            "✔ Project".green(),
            self.name.blue(),
            "has been created".green()
        );

        Ok(())
    }
}
