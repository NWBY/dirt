use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub name: String,
    pub ip_address: String,
    pub php_version: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub ssh_user: String,
    pub ssh_key_path: PathBuf,
    pub commands: Vec<String>,
}

pub fn read_config(config_path: Option<PathBuf>) -> Result<Config, Box<dyn Error>> {
    let config_path = if let Some(path) = config_path {
        path
    } else {
        find_config_file()?
    };

    let mut file = File::open(&config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = match config_path.extension().and_then(|s| s.to_str()) {
        Some("toml") => toml::from_str(&contents)?,
        Some("yaml") | Some("yml") => serde_yaml::from_str(&contents)?,
        Some("json") => serde_json::from_str(&contents)?,
        _ => return Err("Unsupported config file format".into()),
    };
    Ok(config)
}

fn find_config_file() -> Result<PathBuf, Box<dyn Error>> {
    let base_name = "dirt";
    let extensions = ["toml", "yaml", "yml", "json"];

    for ext in extensions.iter() {
        let file_name = format!("{}.{}", base_name, ext);
        let path = PathBuf::from(&file_name);
        if path.exists() {
            return Ok(path);
        }
    }

    Err("No config file found. Searched for dirt.toml, .yaml, .yml, and .json".into())
}
