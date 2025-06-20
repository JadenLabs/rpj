#[derive(clap::Args)]
pub struct UpdateCommand {
    pub name: String,
}

impl UpdateCommand {
    pub fn handle(self) {
        // ...
    }
}
