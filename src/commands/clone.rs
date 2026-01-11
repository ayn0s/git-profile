use crate::commands::r#use::use_profile;
use crate::config::{load_profiles, prompt_profile};
use std::collections::HashMap;
use std::process::Command;

#[cfg(windows)]
const GIT_EXECUTABLE: &str = "git.exe";
#[cfg(not(windows))]
const GIT_EXECUTABLE: &str = "git";

pub fn handle(url: String) {
    let profiles = load_profiles().unwrap_or_else(|err| {
        eprintln!("Profile file read error : {}", err);
        HashMap::new()
    });

    if profiles.is_empty() {
        println!(
            "[Error] No profiles found. Please create a profile first with 'gitp profile add'"
        );
        return;
    }

    let (profile, profile_key) = prompt_profile(&profiles);
    let repo_name = extract_repo_name(&url);
    println!(
        "Cloning {} in {} using profile '{}'",
        url, repo_name, profile.name
    );

    // Git clone with optional SSH key
    let status = if let Some(key) = &profile.ssh_key {
        let key_path = key.replace('\\', "/"); // Ensure forward slashes for SSH
        Command::new(GIT_EXECUTABLE)
            .env("GIT_SSH_COMMAND", format!("ssh -i {}", key_path))
            .args(["clone", &url])
            .status()
    } else {
        Command::new(GIT_EXECUTABLE).args(["clone", &url]).status()
    }
    .expect("[Err] Cloning error");

    if !status.success() {
        return;
    }

    use_profile(Some(profile_key));

    println!(
        "[OK] Repository cloned successfully with the profile '{}'",
        profile.name
    );
}

fn extract_repo_name(url: &str) -> String {
    let path = url.split('/').last().unwrap_or("repo");
    path.strip_suffix(".git").unwrap_or(path).to_string()
}
