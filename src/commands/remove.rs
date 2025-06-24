use crate::utils::{
    ProjectExistsResult, get_store_path, load_projects, project_exists, save_projects,
};
use colored::Colorize;

#[derive(clap::Args)]
pub struct RemoveCommand {
    pub name: String,
}

impl RemoveCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the RPJ store path
        let store_path = get_store_path();
        println!("DEBUG: RPJ store path: {:?}", store_path);

        // Get the project
        match project_exists(&store_path, &self.name, None) {
            ProjectExistsResult::DoesNotExist => {
                return Err(
                    format!("Project '{}' does not exist in the RPJ store!", self.name).into(),
                );
            }
            _ => {}
        }

        // Load existing projects
        let mut projects = load_projects(&store_path);

        // Remove the project from the list
        projects.retain(|p| p.name != self.name);

        // Save the updated projects
        save_projects(&store_path, &projects);

        println!(
            "{} {} {}",
            "✔ Project".green(),
            self.name.blue(),
            "has been removed".green()
        );

        Ok(())
    }
}
