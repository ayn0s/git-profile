use crate::config::{load_profiles, prompt_profile};
use std::process::Command;
use std::path::Path;
use std::collections::HashMap;

pub fn handle(url: String)  {
    let profiles = load_profiles().unwrap_or_else(|err| {
        eprintln!("Profile file read error : {}", err);
        HashMap::new()
    });;
    let profile = prompt_profile(&profiles);

    let repo_name = extract_repo_name(&url);
    println!("Cloning {} in {}", url, repo_name);

    // Git clone
    let clone_status = Command::new("git")
        .args(["clone", &url])
        .status()
        .expect("[Err] Cloning error");

    if !clone_status.success() {
        return;
    }

    let repo_path = Path::new(&repo_name);

    let _ = Command::new("git")
        .args(["config", "user.name", &profile.name])
        .current_dir(repo_path)
        .status();

    let _ = Command::new("git")
        .args(["config", "user.email", &profile.email])
        .current_dir(repo_path)
        .status();

    println!("[OK] Repository cloned successfully with the profile '{}'", profile.name);
}

fn extract_repo_name(url: &str) -> String {
    let path = url.split('/').last().unwrap_or("repo");
    path.strip_suffix(".git").unwrap_or(path).to_string()
}

