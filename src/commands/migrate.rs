use crate::utils::config::Config;
use crate::utils::file::write_file;
use crate::utils::ssh::{run_command, SshRunner};
use chrono::Local;
use ssh2::Session;
use fast_rsync::{Signature, Hasher, Differ, ApplyDiff};
use std::error::Error;
use std::os::raw;
use walkdir::WalkDir;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read, Write, Seek, SeekFrom};

pub fn deploy_app<T: SshRunner>(
    ssh_runner: &T,
    session: &Session,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    println!("Starting deployment process...");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let release_path = format!("/var/www/{}/releases/{}", config.name, timestamp);
    let current_path = format!("/var/www/{}/current", config.name);

    fast_rsync_transfer(".", &current_path, &config.ip_address, "22", username, private_key_path)?;
    
    Ok(())
}

fn ensure_remote_dir_exists(sftp: &ssh2::Sftp, path: &Path) -> Result<(), ssh2::Error> {
    let mut current_path = PathBuf::new();
    for component in path.components() {
        current_path.push(component);
        match sftp.mkdir(&current_path, 0o755) {
            Ok(_) => {},
            Err(err) if err.code() == ssh2::ErrorCode::SFTP(7) => {
                // Directory might already exist, try to continue
            },
            Err(err) => return Err(err),
        }
    }
    Ok(())
}


fn fast_rsync_transfer(
    local_path: &str,
    remote_path: &str,
    host: &str,
    port: u16,
    username: &str,
    private_key_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Establish SSH connection
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_pubkey_file(username, None, Path::new(private_key_path), None)?;
    let sftp = sess.sftp()?;

    // Walk through local directory
    let local_base = PathBuf::from(local_path);
    for entry in WalkDir::new(local_path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(&local_base)?;
            let remote_file_path = Path::new(remote_path).join(relative_path);

            // Ensure remote directory exists
            if let Some(parent) = remote_file_path.parent() {
                ensure_remote_dir_exists(&sftp, parent)?;
            }

            // Generate signature of remote file
            let mut remote_file = match sftp.open(&remote_file_path) {
                Ok(file) => file,
                Err(_) => {
                    // If remote file doesn't exist, copy the whole local file
                    let mut local_file = File::open(path)?;
                    let mut remote_file = sftp.create(&remote_file_path)?;
                    io::copy(&mut local_file, &mut remote_file)?;
                    continue;
                }
            };

            let mut signature_data = Vec::new();
            remote_file.read_to_end(&mut signature_data)?;
            let signature = Signature::calculate(&signature_data, 16, 8);

            // Generate delta
            let mut local_file = File::open(path)?;
            let mut local_data = Vec::new();
            local_file.read_to_end(&mut local_data)?;
            let delta = Differ::for_data(&local_data).diff(&signature);

            // Apply delta to remote file
            let mut remote_file = sftp.open_mode(&remote_file_path, ssh2::OpenFlags::WRITE | ssh2::OpenFlags::TRUNCATE, 0o644, ssh2::OpenType::File)?;
            let mut output = Vec::new();
            ApplyDiff::from_zero_fill(&mut output).apply_diff(&delta)?;
            remote_file.write_all(&output)?;
        }
    }

    Ok(())
}