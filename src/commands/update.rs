use crate::utils::{get_project_by_name, get_store_path};

#[derive(clap::Args)]
pub struct UpdateCommand {
    pub name: String,
    #[arg(long, help = "Directory of the project")]
    pub directory: Option<String>,
    #[arg(long, help = "Command to run the project")]
    pub run_cmd: Option<String>,
    #[arg(long, help = "GitHub URL for the project")]
    pub gh_url: Option<String>,
    #[arg(long, help = "Description of the project")]
    pub description: Option<String>,
}

impl UpdateCommand {
    pub fn handle(self) {
        if self.directory.is_none()
            && self.run_cmd.is_none()
            && self.gh_url.is_none()
            && self.description.is_none()
        {
            eprintln!("No fields provided to update.");
            return;
        }
        println!("Updating project: {}", self.name);

        // Get the RPJ store path
        let store_path = get_store_path();

        // Get the project
        let project_res = get_project_by_name(&store_path, &self.name);
        if project_res.is_none() {
            eprintln!("Project '{}' does not exist in the RPJ store!", self.name);
            return;
        }

        let mut project = project_res.unwrap();
        // Update the project fields
        if let Some(directory) = self.directory {
            println!("Updating directory to: {}", &directory);
            project.directory = directory;
        }
        if let Some(run_cmd) = self.run_cmd {
            println!("Updating run command to: {}", &run_cmd.as_str());
            project.run_cmd = Some(run_cmd);
        }
        if let Some(gh_url) = self.gh_url {
            println!("Updating GitHub URL to: {}", &gh_url);
            project.gh_url = Some(gh_url);
        }
        if let Some(description) = self.description {
            println!("Updating description to: {}", &description);
            project.description = Some(description);
        }

        // Save the updated project back to the RPJ store
        project.save(&store_path)
    }
}
