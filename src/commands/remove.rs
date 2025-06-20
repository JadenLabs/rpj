use crate::utils::{
    get_store_path, load_projects, project_exists, save_projects, ProjectExistsResult,
};

#[derive(clap::Args)]
pub struct RemoveCommand {
    pub name: String,
}

impl RemoveCommand {
    pub fn handle(self) {
        // Get the RPJ store path
        let store_path = get_store_path();
        println!("DEBUG: RPJ store path: {:?}", store_path);

        // Get the project
        match project_exists(&store_path, &self.name, None) {
            ProjectExistsResult::DoesNotExist => {
                eprintln!("Project '{}' does not exist in the RPJ store!", self.name);
                return;
            }
            _ => {
                println!("Deleting project '{}'", self.name);
            }
        }

        // Load existing projects
        let mut projects = load_projects(&store_path);

        // Remove the project from the list
        projects.retain(|p| p.name != self.name);

        // Save the updated projects
        save_projects(&store_path, &projects);

        println!("Project '{}' has been deleted.", self.name);
    }
}
