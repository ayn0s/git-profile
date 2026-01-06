use crate::config::{Profile, create_ssh_key, list_ssh_keys, load_profiles, save_profiles};
use dialoguer::{Confirm, Input, Select};
use std::collections::HashMap;

pub fn list(_verbose: bool) {
    let profiles = super::super::config::load_profiles().unwrap_or_else(|err| {
        eprintln!("Read error : {}", err);
        HashMap::new()
    });
    println!("Available profiles:");

    for (name, prof) in profiles.iter() {
        println!(
            "- {} → {} <{}> (Git name: {})",
            name,
            prof.email,
            prof.ssh_key.as_deref().unwrap_or("no SSH key"),
            prof.name
        );
    }
}

fn is_valid_email(email: &str) -> bool {
    if !email.contains('@') {
        return false;
    }
    if let Some(domain_part) = email.split('@').nth(1) {
        return domain_part.contains('.');
    }
    false
}

pub fn add(name: Option<String>, email: Option<String>, ssh: Option<String>) {
    let profile_id: String = Input::new()
        .with_prompt("Profile name (used to select the profile)")
        .interact_text()
        .unwrap();

    let git_name = name.unwrap_or_else(|| {
        Input::new()
            .with_prompt("Git username (will be used as user.name for commits)")
            .interact_text()
            .unwrap()
    });

    let email = email.unwrap_or_else(|| {
        loop {
            let input: String = Input::new()
                .with_prompt("Git email (will be used as user.email for commits)")
                .interact_text()
                .unwrap();

            if is_valid_email(&input) {
                break input;
            }
            println!("[Error] Invalid email format. Please enter a valid email (example: user@domain.com)");
        }
    });

    let ssh_key = if let Some(key) = ssh {
        Some(key)
    } else {
        let use_ssh = Confirm::new()
            .with_prompt("Add SSH key?")
            .default(false)
            .interact()
            .unwrap();

        if use_ssh {
            let options = vec![
                "Use existing key",
                "Specify path manually",
                "Create new key",
            ];
            let selection = Select::new()
                .with_prompt("How would you like to add the SSH key?")
                .items(&options)
                .default(0)
                .interact()
                .unwrap();

            match selection {
                0 => {
                    let existing_keys = list_ssh_keys();
                    if existing_keys.is_empty() {
                        println!("No SSH keys found in ~/.ssh/");
                        None
                    } else {
                        let key_index = Select::new()
                            .with_prompt("Choose an existing SSH key")
                            .items(&existing_keys)
                            .default(0)
                            .interact()
                            .unwrap();
                        Some(existing_keys[key_index].clone())
                    }
                }
                1 => Some(
                    Input::new()
                        .with_prompt("Enter SSH key path")
                        .interact_text()
                        .unwrap(),
                ),
                2 => {
                    let use_email = Confirm::new()
                        .with_prompt("Add your email as a comment in the SSH key?")
                        .default(false)
                        .interact()
                        .unwrap();

                    match create_ssh_key(&profile_id, if use_email { Some(&email) } else { None }) {
                        Ok(key_path) => {
                            println!("[OK] Created new SSH key: {}", key_path);
                            println!(
                                "Don't forget to add the public key ({}.pub) to your Git provider",
                                key_path
                            );
                            Some(key_path)
                        }
                        Err(e) => {
                            eprintln!("[Error] Failed to create SSH key: {}", e);
                            None
                        }
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    };

    println!("\n[OK] Git profile added:");
    println!("- Profile name: {}", profile_id);
    println!("- Git username: {}", git_name);
    println!("- Git email: {}", email);
    if let Some(key) = &ssh_key {
        println!("- SSH key: {}", key);
    }

    let mut profiles = load_profiles().unwrap_or_else(|_| HashMap::new());

    let new_profile = Profile {
        name: git_name,
        email,
        ssh_key,
    };

    profiles.insert(profile_id, new_profile);
    let _ = save_profiles(profiles);
}

pub fn remove(name: Option<String>) {
    let mut profiles = load_profiles().unwrap_or_else(|_| {
        println!("[Err] No profile found");
        HashMap::new()
    });

    if profiles.is_empty() {
        println!("[Err] No profile to delete");
        return;
    }

    let selected_name = match name {
        Some(n) => n,
        None => {
            let options: Vec<String> = profiles.keys().cloned().collect();

            let index = Select::new()
                .with_prompt("Which profile do you want to delete?")
                .items(&options)
                .default(0)
                .interact()
                .unwrap();
            options[index].clone()
        }
    };

    if let Some(removed) = profiles.remove(&selected_name) {
        println!(
            "[OK] Profile '{}' (Git name: {}) deleted successfully.",
            selected_name, removed.name
        );
        save_profiles(profiles).unwrap();
    } else {
        println!("[Error] Profile '{}' not found.", selected_name);
    }
}
