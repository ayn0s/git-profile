use crate::config::{load_profiles, prompt_profile};
use std::process::Command;
use std::collections::HashMap;
use std::fs;
use dialoguer::{Select, Input, Confirm};

pub fn handle(name: Option<String>)  {
    let profiles = load_profiles().unwrap_or_else(|_| {
        println!("[Err] No profile found");
        HashMap::new()
    });
    let profile = prompt_profile(&profiles);
    
    let path = if let Some(name) = name {
        fs::create_dir_all(&name).unwrap();
        name
    } else {
        ".".to_string()
    };


    Command::new("git").args(["init"]).current_dir(&path).status().unwrap();
    Command::new("git").args(["config", "user.name", &profile.name]).current_dir(&path).status().unwrap();
    Command::new("git").args(["config", "user.email", &profile.email]).current_dir(&path).status().unwrap();

    println!("[OK] Repository initialized with the profile '{}'", profile.name);
}

