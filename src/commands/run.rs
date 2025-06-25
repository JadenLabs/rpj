use crate::utils::{get_project_by_name, get_store_path};
use colored::Colorize;
use std::process::Command;

#[derive(clap::Args)]
pub struct RunCommand {
    pub name: String,
    #[clap(long, short = 't', help = "The terminal to run the command in")]
    pub terminal: Option<String>,
}

impl RunCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let store_path = get_store_path()?;
        let project = get_project_by_name(&store_path, &self.name)?;

        let run_cmd = project.run_cmd.as_deref().ok_or_else(|| {
            format!(
                "Project {} does not have a run command defined. Use {} to update it.",
                self.name.red(),
                "rpj update <name> --run-cmd [command]".dimmed()
            )
        })?;

        let terminal = self.terminal.unwrap_or_else(|| {
            if cfg!(target_os = "windows") {
                "powershell".to_string()
            } else {
                "bash".to_string()
            }
        });

        println!(
            "{} {} {}{}{}\n",
            "ℹ Running".blue(),
            &self.name.blue().bold(),
            "using the '".blue(),
            &terminal.blue().bold(),
            "' terminal.".blue()
        );

        if cfg!(target_os = "windows") {
            run_terminal_on_windows(&project.directory, &terminal, run_cmd)?;
        } else {
            run_terminal_on_unix(&project.directory, &terminal, run_cmd)?;
        }

        Ok(())
    }
}

fn run_terminal_on_windows(
    directory: &str,
    terminal: &str,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let lower = terminal.to_lowercase();
    let mut cmd;

    match lower.as_str() {
        "cmd" => {
            cmd = Command::new("cmd");
            cmd.args(["/C", &format!("cd /d {} && {}", directory, command)]);
        }
        "powershell" | "ps" => {
            cmd = Command::new("powershell");
            cmd.args(["-Command", &format!("cd '{}'; {}", directory, command)]);
        }
        "pwsh" => {
            cmd = Command::new("pwsh");
            cmd.args(["-Command", &format!("cd '{}'; {}", directory, command)]);
        }
        other => {
            return Err(format!("Unsupported terminal on Windows: {}", other).into());
        }
    }

    cmd.spawn()?.wait()?;
    Ok(())
}

fn run_terminal_on_unix(
    directory: &str,
    terminal: &str,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let shell_cmd = format!("cd '{}' && {}", directory, command);
    let mut cmd = Command::new(terminal);

    match terminal {
        "bash" | "sh" | "zsh" => {
            cmd.args(["-c", &shell_cmd]);
        }
        "gnome-terminal" => {
            cmd.args(["--", "bash", "-c", &shell_cmd]);
        }
        other => {
            return Err(format!("Unsupported terminal on Unix: {}", other).into());
        }
    }

    cmd.spawn()?.wait()?;
    Ok(())
}
