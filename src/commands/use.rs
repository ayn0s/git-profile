use crate::config::load_profiles;
use crate::config::prompt_profile;

pub fn handle(name: Option<String>) {
    use_profile(name);
}

pub fn use_profile(name: Option<String>) {
    let profiles = load_profiles().unwrap_or_else(|_| {
        println!("[Err] No profile found");
        std::collections::HashMap::new()
    });

    if profiles.is_empty() {
        println!("[Err] No profile available");
        return;
    }

    let profile = if let Some(n) = name {
        if let Some(prof) = profiles.get(&n) {
            prof
        } else {
            prompt_profile(&profiles)
        }
    } else {
        prompt_profile(&profiles)
    };

    // Set Git config for current repository
    let output = std::process::Command::new("git")
        .args(["config", "user.name", &profile.name])
        .output();

    if let Err(e) = output {
        println!("[Error] Failed to set git user.name: {}", e);
        return;
    }

    let output = std::process::Command::new("git")
        .args(["config", "user.email", &profile.email])
        .output();

    if let Err(e) = output {
        println!("[Error] Failed to set git user.email: {}", e);
        return;
    }

    if let Some(ssh_key) = &profile.ssh_key {
        // Convert path separators to forward slashes for SSH
        let formatted_path = ssh_key.replace('\\', "/");
        let ssh_command = format!("ssh -i \"{}\"", formatted_path);

        let output = std::process::Command::new("git")
            .args(["config", "core.sshCommand", &ssh_command])
            .output();

        if let Err(e) = output {
            println!("[Error] Failed to set git SSH key: {}", e);
            return;
        }
    }

    println!("[OK] Successfully switched to profile '{}'", profile.name);
    println!("- Git username: {}", profile.name);
    println!("- Git email: {}", profile.email);
    if let Some(key) = &profile.ssh_key {
        println!("- SSH key: {}", key);
    }
}
