use crate::utils::{get_project_by_name, get_store_path};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct ExportCommand {
    pub name: String,
    #[arg(
        long,
        help = "Path to export the project file - defaults to current directory"
    )]
    pub export_path: Option<String>,
}

impl ExportCommand {
    pub fn handle(self) {
        // Get the RPJ store path
        let store_path = get_store_path();

        // Get the project
        let project_res = get_project_by_name(&store_path, &self.name);
        if project_res.is_none() {
            eprintln!("Project '{}' does not exist in the RPJ store!", self.name);
            return;
        }
        let mut project = project_res.unwrap();
        project.directory = "".to_string();

        // Export the project to a file
        let parent_dir = match self.export_path {
            Some(path) => PathBuf::from(path),
            None => env::current_dir().expect("Failed to get current directory"),
        };
        println!(
            "Exporting project '{}' to directory '{}'",
            &project.name,
            parent_dir.to_string_lossy()
        );
        let export_path = parent_dir.join(format!("{}.rpj", &project.name));
        let serialized_project =
            serde_json::to_string_pretty(&project).expect("Failed to serialize project to JSON");

        // Write to file
        fs::write(&export_path, serialized_project).expect(
            format!(
                "Failed to write project '{}' to file '{}'",
                &project.name,
                export_path.to_string_lossy()
            )
            .as_str(),
        );
    }
}
