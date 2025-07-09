use std::process::Command;


pub fn run_terminal(directory: &String, terminal: Option<&String>, command: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    let terminal = terminal.unwrap_or_else(|| {
        if cfg!(target_os = "windows") {
                "powershell"
        } else {
                "bash"
        }
    });
    
    if cfg!(target_os = "windows") {
        run_terminal_on_windows(directory, &terminal, run_cmd)?;
    } else {
        run_terminal_on_unix(directory, &terminal, run_cmd)?;
    };
    

    Ok(())
}


pub fn run_terminal_on_windows(
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

pub fn run_terminal_on_unix(
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
