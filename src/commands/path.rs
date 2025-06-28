use crate::utils::{get_store_path, get_project_by_name};


#[derive(clap::Args)]
pub struct PathCommand {
    pub name: String,
}

impl PathCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path()?;
        let project = get_project_by_name(&store_path, &self.name)?;

        println!("{}", project.directory);

        Ok(())
    }
}
