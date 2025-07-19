use colored::Colorize;
use dirs;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const PROJ_NAME: &str = "rpj";

#[derive(clap::Args)]
pub struct InstallCommand {}

impl InstallCommand {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        // Build the project
        let build_status = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .status()
            .map_err(|_| "Failed to build the project")?;
        if !build_status.success() {
            return Err("Cargo build failed".into());
        }

        // Find exec name and target path
        let exe_name = if cfg!(windows) {
            format!("{PROJ_NAME}.exe")
        } else {
            PROJ_NAME.to_string()
        };
        let exe_path = PathBuf::from("target").join("release").join(&exe_name);

        // Get install directory using dirs
        let mut install_dir =
            dirs::data_local_dir().ok_or_else(|| -> Box<dyn std::error::Error> {
                "Could not find local data dir".into()
            })?;
        install_dir.push("rpj");
        install_dir.push("bin");

        // Create install dir if needed
        fs::create_dir_all(&install_dir).map_err(|_| "Failed to create install directory")?;

        // Copy binary
        let install_path = install_dir.join(exe_name);
        fs::copy(&exe_path, &install_path).map_err(|_| "Failed to copy binary")?;

        println!(
            "{} {}",
            "✔ Installed to".green(),
            install_path.to_string_lossy().dimmed()
        );

        // Add to PATH - only windows for now
        if cfg!(windows) {
            add_to_path_windows(&install_dir)?;
        } else {
            // TODO add support for other platforms
            todo!("Automatic PATH updating is not yet supported on this platform");
        }

        Ok(())
    }
}

fn add_to_path_windows(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let dir_str = dir.to_string_lossy();
    let output = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "[Environment]::SetEnvironmentVariable('Path', $env:Path + ';{dir_str}', 'User')"
            ),
        ])
        .output()
        .map_err(|_| "Failed to update PATH")?;

    if output.status.success() {
        println!(
            "{} {} {}",
            "✔ Added".green(),
            dir_str.dimmed(),
            "to user PATH".green()
        );
        Ok(())
    } else {
        Err(format!(
            "Failed to update PATH: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}
