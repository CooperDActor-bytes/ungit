use clap::{Arg, Command};
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let matches = Command::new("Git ungit")
        .version("1.0")
        .author("Cooper Dandilly <cooper@salty.cool>")
        .about("Deinitializes a Git repository")
        .arg(
            Arg::new("path")
                .help("Path to the repository to deinitialize")
                .required(true)
                .index(1),
        )
        .get_matches();

    let repo_path = matches.get_one::<String>("path").unwrap();

    // Construct the path to the .git directory
    let git_dir = Path::new(repo_path).join(".git");

    if git_dir.exists() && git_dir.is_dir() {
        match fs::remove_dir_all(&git_dir) {
            Ok(_) => {
                println!("Successfully deinitialized the repository at '{}'", repo_path);
            }
            Err(e) => {
                eprintln!("Failed to remove .git folder: {}", e);
                process::exit(1);
            }
        }
    } else {
        eprintln!(
            "The specified path '{}' does not contain a .git folder or is not a valid directory",
            repo_path
        );
        process::exit(1);
    }
}

