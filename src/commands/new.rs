use crate::utils::{
    get_store_path, load_projects, project_exists, save_projects, Project, ProjectExistsResult,
};
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
    pub fn handle(self) {
        // Get the RPJ store path
        let store_path = get_store_path();
        println!("DEBUG: RPJ store path: {:?}", store_path);

        // Check if the project already exists in the RPJ store
        let project_path = PathBuf::from(&self.directory);
        let project_canon_path = project_path
                .canonicalize()
                .expect("Failed to canonicalize project path")
                .to_string_lossy()
                .to_string();
        match project_exists(&store_path, &self.name.as_str(), Some(&project_path)) {
            ProjectExistsResult::ExistsByName => {
                eprintln!("Project '{}' already exists in the RPJ store!", self.name);
                return;
            }
            ProjectExistsResult::ExistsByDirectory => {
                eprintln!(
                    "Project path '{}' already exists in the RPJ store!",
                    project_canon_path
                );
                return;
            }
            ProjectExistsResult::DoesNotExist => {
                println!(
                    "Creating new project '{}' in directory '{}'",
                    self.name, self.directory
                );
            }
        }

        // Create a new project instance
        let project = Project {
            name: self.name.clone(),
            directory: project_canon_path,
            run_cmd: self.run_cmd,
            gh_url: self.gh_url,
            description: self.description,
        };

        // Load existing projects from the RPJ store
        let mut projects = load_projects(&store_path);
        // println!("DEBUG: Loaded projects: {:?}", projects);

        projects.push(project);
        save_projects(&store_path, &projects);

        println!("Project '{}' has been created.", self.name);
    }
}
