use crate::utils::{get_project_by_name, get_store_path};
use colored::Colorize;
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
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the RPJ store path
        let store_path = get_store_path()?;

        // Get the project
        let mut project = get_project_by_name(&store_path, &self.name)?;
        project.directory = "".to_string();

        // Export the project to a file
        let parent_dir = match self.export_path {
            Some(path) => PathBuf::from(path),
            None => env::current_dir().map_err(|_| "Failed to get current directory")?,
        };
        println!(
            "{} Exporting {} to directory {}",
            "ℹ".blue(),
            &project.name.to_string().blue(),
            parent_dir.to_string_lossy().dimmed()
        );

        let export_path = parent_dir.join(format!("{}.rpj", &project.name));
        let serialized_project = serde_json::to_string_pretty(&project)
            .map_err(|_| "Failed to serialize project to JSON")?;

        // Write to file
        fs::write(&export_path, serialized_project).map_err(|_| {
            format!(
                "Failed to write project '{}' to file '{}'",
                &project.name,
                export_path.to_string_lossy()
            )
        })?;

        println!(
            "{} {} {} {}",
            "✔ Project".green(),
            &project.name.to_string().blue(),
            "exported to".green(),
            export_path.to_string_lossy().dimmed(),
        );

        Ok(())
    }
}
