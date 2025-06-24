use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub directory: String,
    pub run_cmd: Option<String>,
    pub gh_url: Option<String>,
    pub description: Option<String>,
}

impl Project {
    pub fn save(&self, path: &PathBuf) {
        let projects = load_projects(path);
        let mut projects: Vec<Project> = projects
            .into_iter()
            .filter(|p| p.name != self.name) // Remove existing project with the same name
            .collect();

        projects.push(self.clone());
        save_projects(path, &projects);
    }
}

/// Function to get the path to the RPJ store
pub fn get_store_path() -> PathBuf {
    // Get the dir for the user's local data
    let dir = dirs::data_local_dir().expect("Failed to get local data directory");
    // Make the path to the RPJ store
    let path = dir.join("rpj").join("projects.json");
    // Create the directory if it doesn't exist
    fs::create_dir_all(path.parent().unwrap())
        .expect("Failed to create directory for projects.json");

    // Make sure the file exists
    if !path.exists() {
        fs::write(&path, "[]").expect("Failed to create projects.json file");
    }

    path
}

/// Function to load projects from the RPJ store
pub fn load_projects(path: &PathBuf) -> Vec<Project> {
    // If the file exists, read it and load into a Vec<Project>
    // If it doesn't exist, return an empty Vec<Project>
    if path.exists() {
        let data = fs::read_to_string(path).expect("Failed to read projects file");
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

/// Function to save projects to the RPJ store
pub fn save_projects(path: &PathBuf, projects: &[Project]) {
    let data = serde_json::to_string_pretty(projects).expect("Failed to serialize projects");
    fs::write(path, data).expect("Failed to write projects file");
}

pub enum ProjectExistsResult {
    ExistsByName,
    ExistsByDirectory,
    DoesNotExist,
}

/// Check if a project already exists in the store
pub fn project_exists(
    path: &PathBuf,
    project_name: &str,
    project_path: Option<&PathBuf>,
) -> ProjectExistsResult {
    let projects = load_projects(path);
    if projects.iter().any(|p| p.name == project_name) {
        return ProjectExistsResult::ExistsByName;
    }

    if let Some(proj_path) = project_path {
        if projects
            .iter()
            .any(|p| fs::canonicalize(&p.directory).ok() == fs::canonicalize(proj_path).ok())
        {
            return ProjectExistsResult::ExistsByDirectory;
        };
    }

    ProjectExistsResult::DoesNotExist
}

/// Get project by name from the store
pub fn get_project_by_name(path: &PathBuf, project_name: &str) -> Result<Project, Box<dyn std::error::Error>> {
    let projects = load_projects(path);
    let res = projects.into_iter().find(|p| p.name == project_name);

    match res {
        Some(project) => Ok(project),
        None => Err(format!("Project '{}' not found", project_name).into()),
    }
}
