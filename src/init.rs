use directories::BaseDirs;
use inquire::Text;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use toml;

#[derive(Serialize)]
struct OrgSet {
    org: Option<String>,
    set_always: bool,
}

#[derive(Serialize)]
struct UserInfo {
    username: String,
    token: String,
    org_set: Option<OrgSet>,
}

fn save_to_xdg_config(content: &str) -> std::io::Result<PathBuf> {
    let spinner_chars = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
    let spinner_interval = Duration::from_millis(50);

    // Function to display spinner
    fn show_spinner(spinner_chars: &[&str], interval: Duration) {
        for _ in 0..3 {
            // Adjust the range for the spinner duration
            for &char in spinner_chars {
                print!("\r{}", char);
                std::io::stdout().flush().expect("Failed to flush stdout");
                sleep(interval);
            }
        }
    }

    // Show the spinner in a separate thread
    let spinner_handle = std::thread::spawn(move || {
        show_spinner(&spinner_chars, spinner_interval);
    });

    if let Some(base_dirs) = BaseDirs::new() {
        let config_dir = base_dirs.config_dir().join("revq");

        fs::create_dir_all(&config_dir)?;
        let config_file = config_dir.join("config.toml");
        fs::write(&config_file, content)?;
        spinner_handle.join().expect("Spinner thread panicked");
        return Ok(config_file);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Base directory not found",
    ))
}

pub fn init() {
    let username = Text::new("Please enter your GitHub username;")
        .prompt()
        .expect("Failed to get the username");

    let token = Text::new("Please enter your Github token;")
        .prompt()
        .expect("Failed to get the token");

    let org_input = Text::new("Enter your Github organization name (optional).")
        .prompt_skippable()
        .expect("Failed to get the organization");

    // Convert empty string to None for organization
    let org = if let Some(org_value) = org_input {
        if org_value.is_empty() {
            None
        } else {
            Some(org_value)
        }
    } else {
        None
    };

    let mut org_set = None;

    // If an organization is provided, prompt for whether to include a set_always
    let set_always = if org.is_some() {
        let set_always_input = Text::new("Include set_always information? (yes/no)")
            .prompt()
            .expect("Failed to get the set_always information");
        set_always_input.to_lowercase() == "yes" // Convert to boolean
    } else {
        false // Default to false if no organization is provided
    };

    // Only create org_set if either org or set_always is provided
    if org.is_some() || set_always {
        org_set = Some(OrgSet { org, set_always });
    }

    // Create an instance of UserInfo
    let user_info = UserInfo {
        username,
        token,
        org_set,
    };

    let toml_output = toml::to_string(&user_info).expect("Failed to serialize to TOML");
    match save_to_xdg_config(&toml_output) {
        Ok(path) => println!("\rConfiguration saved to: {}", path.display()),
        Err(e) => eprintln!("Failed to save configuration: {}", e),
    }
}
