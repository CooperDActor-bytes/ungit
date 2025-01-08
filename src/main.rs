use clap::{Arg, Command};
use std::fs;
use std::path::Path;
use std::process::{Command as ProcessCommand, exit};
use std::env;

fn main() {
    let matches = Command::new("ungit")
        .version("1.2.0")
        .author("Cooper Dandilly <cooper@salty.cool>")
        .about("A tool to deinitialize Git repositories")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("deinit")
                .about("Deinitializes a Git repository")
                .arg(
                    Arg::new("path")
                        .help("Path to the repository to deinitialize")
                        .required(true),
                ),
        )
        .subcommand(Command::new("update").about("Updates the ungit tool to the latest version"))
        .subcommand(Command::new("auto-update").about("Checks for and applies updates automatically"))
        .get_matches();

    match matches.subcommand() {
        Some(("deinit", sub_matches)) => {
            let repo_path = sub_matches.get_one::<String>("path").unwrap();
            let git_dir = Path::new(repo_path).join(".git");

            if git_dir.exists() && git_dir.is_dir() {
                match fs::remove_dir_all(&git_dir) {
                    Ok(_) => {
                        println!("Successfully deinitialized the repository at '{}'", repo_path);
                    }
                    Err(e) => {
                        eprintln!("Failed to remove .git folder: {}", e);
                        exit(1);
                    }
                }
            } else {
                eprintln!(
                    "The specified path '{}' does not contain a .git folder or is not a valid directory",
                    repo_path
                );
                exit(1);
            }
        }
        Some(("update", _)) => {
            println!("Updating ungit to the latest version...");
            if let Err(e) = update_tool() {
                eprintln!("Failed to update ungit: {}", e);
                exit(1);
            }
        }
        Some(("auto-update", _)) => {
            println!("Checking for updates...");
            if let Err(e) = auto_update() {
                eprintln!("Failed to auto-update: {}", e);
                exit(1);
            }
        }
        _ => unreachable!(), // This should never happen due to arg_required_else_help
    }
}

/// Updates the ungit tool to the latest version by pulling from GitHub and rebuilding.
fn update_tool() -> Result<(), String> {
    let repo_url = "https://github.com/CooperDActor-bytes/ungit.git";
    let temp_dir = "/tmp/ungit_update";

    // Clone the repository into a temporary directory
    println!("Cloning repository...");
    let status = ProcessCommand::new("git")
        .args(["clone", "--depth", "1", repo_url, temp_dir])
        .status()
        .map_err(|e| format!("Failed to run git command: {}", e))?;

    if !status.success() {
        return Err("Failed to clone the repository".into());
    }

    // Build the project
    println!("Building the new version...");
    let status = ProcessCommand::new("cargo")
        .args(["build", "--release"])
        .current_dir(temp_dir)
        .status()
        .map_err(|e| format!("Failed to run cargo build: {}", e))?;

    if !status.success() {
        return Err("Failed to build the new version".into());
    }

    // Move the binary to /usr/local/bin
    println!("Installing the new version...");
    let binary_path = Path::new(temp_dir).join("target/release/ungit");
    let status = ProcessCommand::new("sudo")
        .args(["mv", binary_path.to_str().unwrap(), "/usr/local/bin/ungit"])
        .status()
        .map_err(|e| format!("Failed to move binary: {}", e))?;

    if !status.success() {
        return Err("Failed to install the new version".into());
    }

    // Clean up the temporary directory
    println!("Cleaning up...");
    let _ = fs::remove_dir_all(temp_dir);

    println!("ungit has been updated successfully!");
    Ok(())
}

/// Checks if updates are available and applies them if necessary.
fn auto_update() -> Result<(), String> {
    // Fetch the latest version from GitHub
    let repo_url = "https://github.com/CooperDActor-bytes/ungit.git";
    let temp_dir = "/tmp/ungit_auto_update";

    println!("Fetching the latest version...");
    let status = ProcessCommand::new("git")
        .args(["ls-remote", repo_url, "HEAD"])
        .output()
        .map_err(|e| format!("Failed to fetch remote version: {}", e))?;

    let remote_commit = String::from_utf8(status.stdout)
        .map_err(|e| format!("Failed to parse remote version: {}", e))?;

    // Compare the current version with the remote commit (you can add versioning logic here)
    if remote_commit.contains(env!("CARGO_PKG_VERSION")) {
        println!("You are already using the latest version.");
        return Ok(());
    }

    println!("New version available. Updating...");
    update_tool()
}

