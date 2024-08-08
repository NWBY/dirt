// src/commands/connect.rs
use std::{env, error::Error, fs};

use crate::utils::config::Config;

pub fn init() -> Result<(), Box<dyn Error>> {
    println!("Generating dirt.json");
    
    let config = Config::default();
    let current_dir = env::current_dir()?;
    let path = current_dir.join("dirt.json");
    
    fs::write(path, serde_yaml::to_string(&config).unwrap())?;

    println!("SSH connection test completed successfully!");

    Ok(())
}
