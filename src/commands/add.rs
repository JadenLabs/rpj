use crate::utils::{
    Project, ProjectExistsResult, get_store_path, load_projects, project_exists, save_projects,
};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct AddCommand {
    pub path: String,
    #[arg(
        long,
        help = "Force add the project, even if it already exists - will delete the existing project first"
    )]
    pub force: bool,

    #[arg(long, help = "Delete the .rpj file after adding the project")]
    pub delete_after: bool,
}

impl AddCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        // Parse the path to the .rpj file
        let mut path = PathBuf::from(&self.path).canonicalize().map_err(|e| {
            format!(
                "Failed to parse path {}: {}",
                self.path.to_string().dimmed(),
                e.to_string().red()
            )
        })?;

        if !path.exists() {
            return Err(format!(
                "The path {} does not exist!",
                self.path.to_string().dimmed()
            )
            .into());
        }

        // If directory, look for .rpj file
        if path.is_dir() {
            let entries = fs::read_dir(&path).map_err(|e| {
                format!(
                    "Failed to read the directory {}: {}",
                    path.to_string_lossy().dimmed(),
                    e.to_string().red()
                )
            })?;

            let mut found_path: Option<PathBuf> = None;

            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().map_or(false, |ext| ext == "rpj") {
                    println!(
                        "{} Found .rpj file: {}",
                        "ℹ".blue(),
                        file_path.display().to_string().dimmed(),
                    );
                    found_path = Some(file_path);
                    break;
                }
            }

            path = found_path.ok_or_else(|| {
                format!("No .rpj file found in directory {}", self.path.to_string())
            })?;
        }

        // Read the .rpj file
        let data = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read the .rpj file: {}", e.to_string().dimmed()))?;
        let mut project: Project = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse the .rpj file: {}", e.to_string().dimmed()))?;

        project.directory = path
            .parent()
            .ok_or("Failed to get the parent directory of the .rpj file")?
            .to_string_lossy()
            .to_string();

        // Get store path and check if project exists
        let store_path = get_store_path();
        match project_exists(&store_path, &project.name, None) {
            ProjectExistsResult::ExistsByName => {
                if self.force {
                    let mut projects = load_projects(&store_path);
                    projects.retain(|p| p.name != project.name);
                    save_projects(&store_path, &projects);
                } else {
                    return Err(format!(
                        "Project {} already exists! Use {} to overwrite it.",
                        project.name.blue(),
                        "--force".green(),
                    )
                    .into());
                }
            }
            _ => {}
        }

        // Load projects
        let mut projects = load_projects(&store_path);
        projects.push(project.clone());
        save_projects(&store_path, &projects);

        println!("{} {}", "✔ Successfully added".green(), project.name.blue());

        // Optionally delete the .rpj file
        if self.delete_after {
            match fs::remove_file(&path) {
                Ok(_) => println!("Deleted the .rpj file at '{}'.", path.to_string_lossy()),
                Err(_) => eprintln!(
                    "Failed to delete the .rpj file at '{}'. Please delete it manually.",
                    path.to_string_lossy()
                ),
            }
        }

        Ok(())
    }
}
