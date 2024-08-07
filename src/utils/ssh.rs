// src/utils/ssh.rs
use crate::utils::config::Config;
use ssh2::Session;
use std::error::Error;
use std::net::TcpStream;
use std::io::prelude::*;

pub fn connect_ssh(server: &str, config: &Config) -> Result<Session, Box<dyn Error>> {
    let address = format!("{}:22", server);
    let tcp = TcpStream::connect(address)?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    
    sess.userauth_pubkey_file(
        &config.ssh_user,
        None,
        &config.ssh_key_path,
        None,
    )?;
    
    if !sess.authenticated() {
        return Err("SSH authentication failed".into());
    }
    
    Ok(sess)
}

pub fn run_command(session: &Session, command: &str) -> Result<(), Box<dyn Error>> {
    println!("Running command: {}", command);
    let mut channel= session.channel_session()?;
    channel.exec(command)?;
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;
    println!("Command output: {}", output);
    Ok(())
}


pub fn run_command_with_output(session: &Session, command: &str) -> Result<String, Box<dyn Error>> {
    println!("Running command: {}", command);
    let mut channel = session.channel_session()?;
    channel.exec(command)?;
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;
    Ok(output.trim().to_string())
}