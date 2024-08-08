// src/commands/connect.rs
use crate::utils::ssh::SshRunner;
use ssh2::Session;
use std::error::Error;

pub fn test_connection<T: SshRunner>(
    ssh_runner: &T,
    session: &Session,
) -> Result<(), Box<dyn Error>> {
    println!("Connected successfully. Testing command execution...");

    // Run a simple command to test
    let output =
        ssh_runner.run_command_with_output(session, "echo 'SSH connection test successful'")?;

    println!("Command output: {}", output);
    println!("SSH connection test completed successfully!");

    Ok(())
}
