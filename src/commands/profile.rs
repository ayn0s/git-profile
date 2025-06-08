use crate::config::{config_path, Profile, save_profiles, load_profiles};
use std::collections::HashMap;
use dialoguer::{Select, Input, Confirm};
use std::io::{self, Write};

pub fn list(_verbose: bool) {
    let profiles = super::super::config::load_profiles().unwrap_or_else(|err| {
        eprintln!("Read error : {}", err);
        HashMap::new()
    });
    println!("Available profiles :");

    for (key, prof) in profiles.iter() {
        println!("- {} → {} <{}>", key, prof.name, prof.email);
    }

}


pub fn add(name: Option<String>, email: Option<String>, ssh: Option<String>) {
    let name = name.unwrap_or_else(|| {
        Input::new()
            .with_prompt("Profile name")
            .interact_text()
            .unwrap()
    });

    let email = email.unwrap_or_else(|| {
        Input::new()
            .with_prompt("Email")
            .interact_text()
            .unwrap()
    });

    let ssh = ssh.or_else(|| {
        let use_ssh = Confirm::new()
            .with_prompt("Add SSH key? ")
            .default(false)
            .interact()
            .unwrap();

        if use_ssh {
            Some(
                Input::new()
                    .with_prompt("SSH hostname")
                    .interact_text()
                    .unwrap(),
            )
        } else {
            None
        }
    });

    println!("[OK] Profile added :\n- Name: {name}\n- Email: {email}\n- SSH: {:?}", ssh);
    let mut profiles = load_profiles().unwrap_or_else(|_| {
        return HashMap::new()
    });
    let key = name.clone();
    let new_profile = Profile { name:name, email:email, ssh:ssh };
    profiles.insert(key, new_profile);
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
            let options: Vec<String> = profiles
                .iter()
                .map(|(_, p)| p.name.clone())
                .collect();

            let index = Select::new()
                .with_prompt("Which profile do you want to delete?")
                .items(&options)
                .default(0)
                .interact()
                .unwrap();
            options[index].clone()
        }
    };

    profiles.retain(|_, p| p.name != selected_name);

    save_profiles(profiles).unwrap();

    println!("[OK] Profile '{}' deleted successfully.", selected_name);
    
}


