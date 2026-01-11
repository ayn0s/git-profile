use dialoguer::Select;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub ssh_key: Option<String>,
}

fn get_ssh_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".ssh"))
}

fn is_ssh_private_key(path: &Path) -> bool {
    if let Ok(mut file) = fs::File::open(path) {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            return content.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----")
                || content.starts_with("-----BEGIN RSA PRIVATE KEY-----")
                || content.starts_with("-----BEGIN DSA PRIVATE KEY-----")
                || content.starts_with("-----BEGIN EC PRIVATE KEY-----")
                || content.starts_with("-----BEGIN PRIVATE KEY-----");
        }
    }
    false
}

pub fn list_ssh_keys() -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(ssh_dir) = get_ssh_dir() {
        if let Ok(entries) = fs::read_dir(ssh_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    // Skip known non-key files and .pub files
                    if [
                        "known_hosts",
                        "known_hosts.old",
                        "config",
                        "authorized_keys",
                    ]
                    .iter()
                    .any(|x| file_name == *x)
                        || file_name.ends_with(".pub")
                    {
                        continue;
                    }

                    if is_ssh_private_key(&path) {
                        if let Some(path_str) = path.to_str() {
                            // Convert Windows path separators to forward slashes
                            keys.push(path_str.replace('\\', "/"));
                        }
                    }
                }
            }
        }
    }

    if keys.is_empty() {
        println!("[Warning] No SSH keys found in ~/.ssh/");
    } else {
        println!("[Info] Found {} SSH key(s)", keys.len());
    }

    keys
}

pub fn create_ssh_key(name: &str, comment: Option<&str>) -> std::io::Result<String> {
    let ssh_dir = get_ssh_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        )
    })?;
    fs::create_dir_all(&ssh_dir)?;

    let key_path = ssh_dir.join(format!("id_ed25519_{}", name));
    // Convert path to string with forward slashes
    let key_path_str = key_path
        .to_str()
        .map(|s| s.replace('\\', "/"))
        .unwrap_or_default();

    let mut command = std::process::Command::new("ssh-keygen");
    command.args([
        "-t",
        "ed25519",
        "-f",
        &key_path_str,
        "-N",
        "", // Empty passphrase
        "-C",
        comment.unwrap_or(""), // Empty comment by default
    ]);

    command.status()?;
    Ok(key_path_str)
}

pub fn config_path() -> PathBuf {
    config_dir()
        .expect("Could not find config directory")
        .join("gitp")
        .join("profiles.json")
}

pub fn prompt_profile(profiles: &HashMap<String, Profile>) -> (&Profile, String) {
    println!("Available profiles:");
    for (name, prof) in profiles.iter() {
        println!(
            "  - {} → {} {}",
            name,
            prof.email,
            prof.ssh_key
                .as_ref()
                .map_or("(no SSH key)".to_string(), |k| format!("(SSH: {})", k))
        );
    }

    let selected_name = {
        let options: Vec<String> = profiles.keys().cloned().collect();

        let index = Select::new()
            .with_prompt("Select profile to use")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();
        options[index].clone()
    };

    (
        profiles
            .get(&selected_name)
            .expect("Unable to find the profile"),
        selected_name,
    )
}

pub fn load_profiles() -> Result<HashMap<String, Profile>, std::io::Error> {
    let path = config_path();

    if !path.exists() {
        println!(
            "[Info] No profile file found, creating one at: '{}'",
            path.display()
        );
        let empty: HashMap<String, Profile> = HashMap::new();
        let content = serde_json::to_string_pretty(&empty)?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(&path, content)?;
        return Ok(empty);
    }

    let content = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?)
}

pub fn save_profiles(profiles: HashMap<String, Profile>) -> std::io::Result<()> {
    let path = config_path();
    fs::create_dir_all(path.parent().unwrap())?;
    let content = serde_json::to_string_pretty(&profiles)?;
    fs::write(path, content)
}
