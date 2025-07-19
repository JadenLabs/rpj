use std::process::Command;

pub fn get_default_terminal() -> String {
    if cfg!(target_os = "windows") {
        String::from("powershell")
    } else {
        String::from("bash")
    }
}

pub fn run_terminal(
    directory: &String,
    terminal: Option<&String>,
    command: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let default_terminal = get_default_terminal();
    let terminal: &String = terminal.unwrap_or(&default_terminal);

    if cfg!(target_os = "windows") {
        run_terminal_on_windows(directory, terminal, command)?;
    } else {
        run_terminal_on_unix(directory, terminal, command)?;
    };

    Ok(())
}

pub fn run_terminal_on_windows(
    directory: &String,
    terminal: &str,
    command: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let terminal = terminal.to_lowercase();
    let mut cmd;

    match terminal.as_str() {
        "cmd" => {
            cmd = Command::new("cmd");
            if let Some(cmd_str) = command {
                cmd.args(["/C", &format!("cd /d {directory} && {cmd_str}")]);
            } else {
                cmd.args(["/K", &format!("cd /d {directory}")]);
            }
        }
        "powershell" | "ps" => {
            cmd = Command::new("powershell");
            if let Some(cmd_str) = command {
                cmd.args(["-Command", &format!("cd '{directory}'; {cmd_str}")]);
            } else {
                cmd.args(["-NoExit", "-Command", &format!("cd '{directory}'")]);
            }
        }
        "pwsh" => {
            cmd = Command::new("pwsh");
            if let Some(cmd_str) = command {
                cmd.args(["-Command", &format!("cd '{directory}'; {cmd_str}")]);
            } else {
                cmd.args(["-NoExit", "-Command", &format!("cd '{directory}'")]);
            }
        }
        _ => {
            return Err(format!("Unsupported terminal on Windows: {terminal}").into());
        }
    }

    cmd.spawn()?.wait()?;
    Ok(())
}

pub fn run_terminal_on_unix(
    directory: &String,
    terminal: &String,
    command: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let shell_command = match command {
        Some(cmd_str) => format!("cd '{}' && {}", directory, cmd_str),
        None => format!("cd '{directory}'"),
    };
    let mut cmd = Command::new(terminal);

    match terminal.as_str() {
        "bash" | "sh" | "zsh" => {
            cmd.args(["-c", &shell_command]);
        }
        "gnome-terminal" => {
            cmd.args(["--", "bash", "-c", &shell_command]);
        }
        _ => {
            return Err(format!("Unsupported terminal on Unix: {terminal}").into());
        }
    }

    cmd.spawn()?.wait()?;
    Ok(())
}
