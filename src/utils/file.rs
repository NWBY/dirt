// src/utils/file.rs
use ssh2::Session;
use std::error::Error;
use std::path::Path;
use std::io::Write;

pub fn write_file(session: &Session, path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    println!("Writing file: {}", path);
    let mut channel = session.scp_send(Path::new(path), 0o644, content.len() as u64, None)?;
    channel.write_all(content.as_bytes())?;
    channel.send_eof()?;
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;
    Ok(())
}