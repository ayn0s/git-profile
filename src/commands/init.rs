use crate::config::{load_profiles, prompt_profile};
use std::process::Command;
use std::collections::HashMap;
use std::fs;

#[cfg(windows)]
const GIT_EXECUTABLE: &str = "git.exe";
#[cfg(not(windows))]
const GIT_EXECUTABLE: &str = "git";

pub fn handle(name: Option<String>)  {
    let profiles = load_profiles().unwrap_or_else(|_| {
        println!("[Error] No profiles found. Please create a profile first with 'gitp profile add'");
        HashMap::new()
    });

    if profiles.is_empty() {
        return;
    }

    let profile = prompt_profile(&profiles);
    
    let path = if let Some(name) = name {
        fs::create_dir_all(&name).unwrap();
        name
    } else {
        ".".to_string()
    };

    println!("Initializing Git repository using profile '{}'", profile.name);

    Command::new(GIT_EXECUTABLE).args(["init"]).current_dir(&path).status().unwrap();
    Command::new(GIT_EXECUTABLE).args(["config", "user.name", &profile.name]).current_dir(&path).status().unwrap();
    Command::new(GIT_EXECUTABLE).args(["config", "user.email", &profile.email]).current_dir(&path).status().unwrap();

    println!("[OK] Repository initialized with the profile '{}'", profile.name);
}

