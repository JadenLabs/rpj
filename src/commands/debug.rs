use crate::utils::get_store_path;

#[derive(clap::Args)]
pub struct DebugCommand {
    pub args: Option<Vec<String>>,
}

impl DebugCommand {
    pub fn handle(self) {
        // Get the RPJ store path
        let store_path = get_store_path();
        println!(
            "RPJ Store Path: {}",
            store_path
                .canonicalize()
                .expect("Failed to format store path string")
                .to_string_lossy()
        );
    }
}
