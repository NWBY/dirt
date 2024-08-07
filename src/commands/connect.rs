// src/commands/connect.rs
use ssh2::Session;
use std::error::Error;
use crate::utils::ssh::run_command_with_output;

pub fn test_connection(session: &Session) -> Result<(), Box<dyn Error>> {
    println!("Connected successfully. Testing command execution...");
    
    // Run a simple command to test
    let output = run_command_with_output(session, "echo 'SSH connection test successful'")?;
    
    println!("Command output: {}", output);
    println!("SSH connection test completed successfully!");
    
    Ok(())
}