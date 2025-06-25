use crate::utils::{get_store_path,};
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
            "{} {}",
            "ℹ RPJ Store Path:".blue(),
            store_path.to_string_lossy().dimmed()
        );

        Ok(())
    }
}
