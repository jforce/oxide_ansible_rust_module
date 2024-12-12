use std::env;
use std::fs;
use std::process;

use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct ModuleArgs {
    oxide_host: String,
    oxide_token: String,
    name: String,
    public_key: Option<String>,
    description: Option<String>,
    state: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    msg: String,
    changed: bool,
    failed: bool,
    ssh_key: Option<Value>,
}

fn fail_json(msg: &str) -> ! {
    let response = Response {
        msg: msg.to_string(),
        changed: false,
        failed: true,
        ssh_key: None,
    };
    println!("{}", serde_json::to_string(&response).unwrap());
    process::exit(1);
}

fn main() {
    // Get what's passed by Ansible
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        fail_json("Expected json.");
    }

    let input_file = &args[1];
    let input = fs::read_to_string(input_file)
        .unwrap_or_else(|err| fail_json(&format!("Failed to read input file: {}", err)));

    let module_args: ModuleArgs = serde_json::from_str(&input)
        .unwrap_or_else(|err| fail_json(&format!("Failed to parse input JSON: {}", err)));

    let response = match module_args.state.as_str() {
        "present" => create_ssh_key(&module_args),
        "absent" => delete_ssh_key(&module_args),
        _ => Err("Invalid state. Use 'present' or 'absent'.".into()),
    };

    match response {
        Ok(res) => println!("{}", serde_json::to_string(&res).unwrap()),
        Err(err) => fail_json(&format!("{}", err)),
    }
}

fn create_ssh_key(args: &ModuleArgs) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();

    // Check if the key already exists
    let url = format!("{}/v1/me/ssh-keys/{}", args.oxide_host, args.name);
    let get_response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", args.oxide_token))
        .send()?;

    if get_response.status().is_success() {
        let existing_key: Value = get_response.json()?; // Parse existing key data

        // Extract and compare the public key
        let existing_public_key = existing_key
            .get("public_key")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();

        let new_public_key = args
            .public_key
            .as_deref()
            .unwrap_or("")
            .trim();

        if existing_public_key == new_public_key {
            return Ok(Response {
                msg: format!("SSH key '{}' already exists", args.name),
                changed: false,
                failed: false,
                ssh_key: Some(existing_key),
            });
        } else {
            return Err(format!("SSH key name '{}' already exists",args.name).into());
        }
    } else if get_response.status() == 404 {
        // Key does not exist, create a new one
        let post_url = format!("{}/v1/me/ssh-keys", args.oxide_host);
        let payload = serde_json::json!({
            "name": args.name,
            "public_key": args.public_key.as_ref().ok_or("Missing public_key")?,
            "description": args.description.as_ref().unwrap_or(&"Managed by Ansible".to_string())
        });

        let post_response = client
            .post(&post_url)
            .header("Authorization", format!("Bearer {}", args.oxide_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()?;

        if post_response.status().is_success() {
            let created_key: Value = post_response.json()?;
            return Ok(Response {
                msg: format!("SSH key '{}' created successfully", args.name),
                changed: true,
                failed: false,
                ssh_key: Some(created_key),
            });
        } else {
            return Err(format!(
                "Failed to create SSH key '{}': {}",
                args.name,
                post_response.text()?
            )
            .into());
        }
    }

    Err(format!("Failed to retrieve SSH key '{}'",args.name).into())
}

fn delete_ssh_key(args: &ModuleArgs) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("{}/v1/me/ssh-keys/{}", args.oxide_host, args.name);

    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", args.oxide_token))
        .send()?;

    if response.status().is_success() {
        Ok(Response {
            msg: "SSH key deleted successfully".to_string(),
            changed: true,
            failed: false,
            ssh_key: None,
        })
    } else {
        let error_body = response.text()?;
        let parsed_error: Value = serde_json::from_str(&error_body).unwrap_or_default();

        if let Some(error_code) = parsed_error.get("error_code") {
            if error_code == "ObjectNotFound" {
                return Ok(Response {
                    msg: format!("SSH key '{}' not found (already absent)", args.name),
                    changed: false,
                    failed: false,
                    ssh_key: None,
                });
            }
        }

        Err(format!("Failed to delete SSH key: {}",error_body).into())
    }
}
