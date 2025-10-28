use crate::config::{load_profiles, prompt_profile};
use std::process::Command;
use std::path::Path;
use std::collections::HashMap;

#[cfg(windows)]
const GIT_EXECUTABLE: &str = "git.exe";
#[cfg(not(windows))]
const GIT_EXECUTABLE: &str = "git";

pub fn handle(url: String)  {
    let profiles = load_profiles().unwrap_or_else(|err| {
        eprintln!("Profile file read error : {}", err);
        HashMap::new()
    });

    if profiles.is_empty() {
        println!("[Error] No profiles found. Please create a profile first with 'gitp profile add'");
        return;
    }

    let profile = prompt_profile(&profiles);
    let repo_name = extract_repo_name(&url);
    println!("Cloning {} in {} using profile '{}'", url, repo_name, profile.name);

    // Git clone with optional SSH key
    let status = if let Some(key) = &profile.ssh_key {
        Command::new(GIT_EXECUTABLE)
            .env("GIT_SSH_COMMAND", format!("ssh -i {}", key))
            .args(["clone", &url])
            .status()
    } else {
        Command::new(GIT_EXECUTABLE)
            .args(["clone", &url])
            .status()
    }.expect("[Err] Cloning error");

    if !status.success() {
        return;
    }

    let repo_path = Path::new(&repo_name);

    let _ = Command::new(GIT_EXECUTABLE)
        .args(["config", "user.name", &profile.name])
        .current_dir(repo_path)
        .status();

    let _ = Command::new(GIT_EXECUTABLE)
        .args(["config", "user.email", &profile.email])
        .current_dir(repo_path)
        .status();

    println!("[OK] Repository cloned successfully with the profile '{}'", profile.name);
}

fn extract_repo_name(url: &str) -> String {
    let path = url.split('/').last().unwrap_or("repo");
    path.strip_suffix(".git").unwrap_or(path).to_string()
}

