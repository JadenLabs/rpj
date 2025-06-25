use crate::utils::get_store_path;
use colored::Colorize;

#[derive(clap::Args)]
pub struct DebugCommand {
    pub args: Option<Vec<String>>,
}

impl DebugCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the RPJ store path
        let store_path = get_store_path()?;
        println!(
            "{} RPJ Store Path: {}",
            "ℹ".blue(),
            store_path
                .canonicalize()
                .map_err(|_| "Failed to canonicalize path")?
                .to_string_lossy()
                .dimmed()
        );

        Ok(())
    }
}
