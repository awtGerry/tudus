use std::path::PathBuf;

use dirs::config_dir;
use toml;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub lang: String,
    pub theme: Theme
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub variant: Option<String>, // For now the only available options are light/dark, later custom theme will be available.
}

/* Create default configurations */
// Example:
// language = "en";
// [theme]
// variant = "dark";
pub fn create_default_configs() {
    default_path().unwrap();
    if !std::path::Path::new(&config_dir().unwrap().join("tudus").join("config.toml")).exists() {
        let default_config: Config = Config {
            lang: "en".to_string(),
            theme: Theme {
                variant: Some("dark".to_string()),
            },
        };

        let toml = toml::to_string(&default_config).unwrap();
        // Write to file
        std::fs::write(config_dir().unwrap().join("tudus").join("config.toml"), toml).unwrap();
    }
}

#[allow(unused)]
pub fn get_theme() -> Theme { // returns only the theme
    let path = config_dir().unwrap().join("tudus").join("config.toml");
    let config: Config = toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    config.theme
}

pub fn toggle_theme() { // main for now
    let path = config_dir().unwrap().join("tudus").join("config.toml");
    let mut config: Config = toml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    match config.theme.variant {
        Some(ref variant) => {
            if variant == "dark" {
                config.theme.variant = Some("light".to_string());
            } else {
                config.theme.variant = Some("dark".to_string());
            }
        },
        None => { // If the variant is not set, set it to dark
            config.theme.variant = Some("dark".to_string());
        }
    }

    let toml = toml::to_string(&config).unwrap();
    // Write to file
    std::fs::write(&path, toml).unwrap();
}

fn default_path() -> Result<PathBuf, std::io::Error> {
    let mut path = config_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Data directory not found",
    ))?;
    // Create a directory for the app
    path.push("tudus");
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&path)?;
    Ok(path)
}
