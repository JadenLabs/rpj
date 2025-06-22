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
    pub fn handle(self) {
        // Parse the path to the .rpj file
        let mut path = PathBuf::from(&self.path)
            .canonicalize()
            .expect(format!("{} Failed to parse path", "✖".red()).as_str());
        if !path.exists() {
            eprintln!(
                "{} The specified path '{}' does not exist!",
                "✖".red(),
                self.path
            );
            return;
        }

        match fs::read_dir(&path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    if file_path.extension().map_or(false, |ext| ext == "rpj") {
                        println!(
                            "Found .rpj file: {}",
                            file_path.display().to_string().green(),
                        );
                        path = file_path;
                        break;
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to read the directory '{}': {}", path.display(), err);
                return;
            }
        }

        // Read the .rpj file
        let data = fs::read_to_string(&path).expect("Failed to read the .rpj file");
        let mut project: Project =
            serde_json::from_str(&data).expect("Failed to parse the .rpj file");
        project.directory = path
            .parent()
            .expect("Failed to get the parent directory of the .rpj file")
            .to_string_lossy()
            .to_string();

        // Get store path and check if project exists
        let store_path = get_store_path();
        match project_exists(&store_path, &project.name, None) {
            ProjectExistsResult::ExistsByName => {
                if self.force {
                    println!(
                        "Project '{}' already exists in the RPJ store! Removing it first.",
                        project.name
                    );
                    let mut projects = load_projects(&store_path);
                    projects.retain(|p| p.name != project.name);
                    save_projects(&store_path, &projects);
                } else {
                    eprintln!(
                        "Project '{}' already exists in the RPJ store! Use --force to overwrite it.",
                        project.name
                    );
                    return;
                }
            }
            _ => {}
        }

        // Load projects
        let mut projects = load_projects(&store_path);
        projects.push(project.clone());
        save_projects(&store_path, &projects);

        println!(
            "Project '{}' has been successfully added to the RPJ store!",
            project.name
        );

        // Optionally delete the .rpj file
        if self.delete_after {
            if fs::remove_file(&path).is_err() {
                eprintln!(
                    "Failed to delete the .rpj file at '{}'. Please delete it manually.",
                    path.to_string_lossy()
                );
            } else {
                println!("Deleted the .rpj file at '{}'.", path.to_string_lossy());
            }
        }
    }
}
