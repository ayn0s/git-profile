use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;
use dialoguer::{Select, Input, Confirm};

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub ssh: Option<String>,
}

pub fn config_path() -> PathBuf {
    config_dir().unwrap().join("gitp/profiles.json")
}

pub fn prompt_profile(profiles: &HashMap<String, Profile>) -> &Profile {
    println!("Available profiles :");
    for (key, prof) in profiles.iter() {
        println!("  - {} ({})", key, prof.email);
    }

    let selected_name = {
            let options: Vec<String> = profiles
                .iter()
                .map(|(_, p)| p.name.clone())
                .collect();

            let index = Select::new()
                .with_prompt("Profile to use ?")
                .items(&options)
                .default(0)
                .interact()
                .unwrap();
            options[index].clone()
     
    };

    println!("Profile to use? ");

    profiles.get(&selected_name).expect("Unable to find the profile")
}

pub fn load_profiles() -> Result<HashMap<String, Profile>, std::io::Error> {
    let path = config_path();
    
    // S'il n'existe pas, on le crée vide
    if !path.exists() {
        println!("[Info] No profile file found, creating one at : '{}'.", path.display());
        let empty: HashMap<String, Profile> = HashMap::new();
        let content = serde_json::to_string_pretty(&empty).unwrap();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, content).unwrap();
    return Ok(empty);
    }
    


    // Sinon, on le lit normalement
    let content = fs::read_to_string(path).expect("Unable to read profiles.json");
    Ok(serde_json::from_str(&content).expect("Invalid JSON file"))
}

pub fn save_profiles(profiles: HashMap<String, Profile>) -> std::io::Result<()> {
    use std::fs;
    let data = serde_json::to_string_pretty(&profiles)?;
    let path = config_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, data)?;
    Ok(())
}
