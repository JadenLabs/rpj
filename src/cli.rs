use crate::commands::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(aliases = &["c", "create", "init", "make"], about = "Create a new project")]
    New(NewCommand),

    #[command(aliases = &["d", "delete", "rm"], about = "Delete an existing project")]
    Remove(RemoveCommand),

    #[command(aliases = &["u"], about = "Update an existing project")]
    Update(UpdateCommand),

    // #[command(aliases = &["a", "import"], about = "Add an existing project)]
    // Add(AddCommand),

    // #[command(aliases = &["l", "ls", "all"], about = "List all projects")]
    // List(ListCommand),

    // #[command(aliases = &["g", "search", "find", "info"], about = "Get information about a project")]
    // Get(GetCommand),

    // #[command(aliases = &["vsc", "edit"], about = "Open project in VS Code")]
    // Code(CodeCommand),

    // #[command(aliases = &["r", "execute", "exec", "start"], about = "Run a project")]
    // Run(RunCommand),

    // #[command(aliases = &["open-dir", "file", "explore", "open"], about = "Open project in file explorer")]
    // Explorer(ExplorerCommand),

    // Debug(DebugCommand),

    // #[command(about = "Get the path of a project")]
    // Path(PathCommand),

    // #[command(aliases = &["ter", "shell", "term", "sh", "cmd"], about = "Open a project in the terminal")]
    // Terminal(TerminalCommand),

    // #[command(aliases = &["tree", "structure"], about = "Show the project structure")]
    // Tree(TreeCommand),

    // #[command(aliases = &["label"], about = "Manage tags for a project")]
    // Tag(TagCommand),

    // #[command(aliases = &["n", "comment"], about = "Add a note to a project")]
    // Note(NoteCommand),
}
