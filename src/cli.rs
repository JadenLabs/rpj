use crate::commands::*;
use anstyle::{AnsiColor, Color, Style};
use clap::{Parser, Subcommand, builder};
use colored::Colorize;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = format!(
        "{} - A project management CLI for quick project manipulation.\n  Run {} to get started.",
        "RPJ".bold().blue(),
        "rpj new <name> <directory>".bold().blue()
    ),
    styles = get_styles()
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        aliases = &["a", "import"],
        about = format!("Add an existing project using a .rpj file  {}", "(a, import)".dimmed())
    )]
    Add(AddCommand),

    #[command(
        aliases = &["c", "create", "init", "make"],
        about = format!("Create a new project  {}", "(c, create, init, make)".dimmed())
    )]
    New(NewCommand),

    #[command(
        aliases = &["e"],
        about = format!("Export a project to a file  {}", "(e)".dimmed())
    )]
    Export(ExportCommand),

    #[command(
        aliases = &["l", "ls", "all"],
        about = format!("List all projects  {}", "(l, ls, all)".dimmed())
    )]
    List(ListCommand),

    #[command(
        aliases = &["i"],
        about = format!("Install a project to the system bins  {}", "(i)".dimmed())
    )]
    Install(InstallCommand),

    #[command(
        aliases = &["g", "search", "find", "info"],
        about = format!("Get information about a project  {}", "(g, search, find, info)".dimmed())
    )]
    Get(GetCommand),

    #[command(
        aliases = &["vsc", "edit"],
        about = format!("Open project in VS Code  {}", "(vsc, edit)".dimmed())
    )]
    Code(CodeCommand),

    #[command(
        aliases = &["r", "execute", "exec", "start"],
        about = format!("Run a project  {}", "(r, execute, exec, start)".dimmed())
    )]
    Run(RunCommand),

    #[command(
        aliases = &["u"],
        about = format!("Update an existing project  {}", "(u)".dimmed())
    )]
    Update(UpdateCommand),

    #[command(
        aliases = &["d", "delete", "rm"],
        about = format!("Delete an existing project  {}", "(d, delete, rm)".dimmed())
    )]
    Remove(RemoveCommand),

    #[command(
        aliases = &["open-dir", "file", "explorer", "open"],
        about = format!("Open project in file explorer  {}", "(open-dir, file, explorer, open)".dimmed())
    )]
    Explore(ExploreCommand),

    #[command(about = "Debug internal state")]
    Debug(DebugCommand),

    #[command(about = "Get the path of a project")]
    Path(PathCommand),
    // Future:
    #[command(
        aliases = &["ter", "shell", "term", "sh", "cmd"],
        about = format!("Open a project in the terminal {}", "(ter, shell, term, sh, cmd)".dimmed()))
    ]
    Terminal(TerminalCommand),
    // #[command(aliases = &["structure", "tree"], about = "Show the project structure")]
    // Tree(TreeCommand),
    // #[command(aliases = &["label"], about = "Manage tags for a project")]
    // Tag(TagCommand),
    // #[command(aliases = &["n", "comment"], about = "Add a note to a project")]
    // Note(NoteCommand),
    // #[command(aliases = &["bk", "bkmk"], about = "Use bookmarks to quickly navigate directories")]
    // Bookmark(BookmarkCommand)
}

pub fn get_styles() -> builder::Styles {
    builder::Styles::styled()
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .header(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .placeholder(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Blue))),
        )
        .literal(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::BrightCyan))),
        )
}
