
#[derive(clap::Args)]
pub struct TerminalCommand {
    pub name: String,
}

impl TerminalCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
       

        Ok(())
    }
}
