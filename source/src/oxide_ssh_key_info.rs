use std::env;
use std::fs;
use std::process;
use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::Client;

#[derive(Deserialize)]
struct ModuleArgs {
    oxide_host: String,
    oxide_token: String,
    name: Option<String>,
}

fn fail_json(msg: &str) -> ! {
    let response = serde_json::json!({
        "msg": msg,
        "changed": false,
        "failed": true
    });
    println!("{}", serde_json::to_string(&response).unwrap());
    process::exit(1);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read what's passed by Ansible
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        fail_json("Expected JSON input file path as an argument.");
    }

    let input_file = &args[1];
    let input = fs::read_to_string(input_file)
        .unwrap_or_else(|err| fail_json(&format!("Failed to read input file: {}", err)));

    let module_args: ModuleArgs = serde_json::from_str(&input)
        .unwrap_or_else(|err| fail_json(&format!("Failed to parse input JSON: {}", err)));

    let client = Client::new();

    if let Some(key_name) = &module_args.name {
        // Fetch a specific SSH key
        let url = format!("{}/v1/me/ssh-keys/{}", module_args.oxide_host, key_name);
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", module_args.oxide_token))
            .send()?;

        if response.status().is_success() {
            let ssh_key: Value = response.json()?;
            let output = serde_json::json!({
                "msg": format!("SSH key '{}' fetched successfully", key_name),
                "changed": false,
                "ssh_key": ssh_key
            });
            println!("{}", serde_json::to_string(&output).unwrap());
        } else {
            fail_json(&format!(
                "Failed to fetch SSH key '{}': {}",
                key_name,
                response.text()?
            ));
        }
    } else {
        // Fetch all SSH keys
        let url = format!("{}/v1/me/ssh-keys", module_args.oxide_host);
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", module_args.oxide_token))
            .send()?;

        if response.status().is_success() {
            let ssh_keys: Value = response.json()?;
            let output = serde_json::json!({
                "msg": "SSH keys fetched successfully",
                "changed": false,
                "ssh_keys": ssh_keys
            });
            println!("{}", serde_json::to_string(&output).unwrap());
        } else {
            fail_json(&format!("Failed to list SSH keys: {}", response.text()?));
        }
    }

    Ok(())
}
