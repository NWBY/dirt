use std::error::Error;

use ssh2::Session;

use super::ssh::SshRunner;

pub fn install_caddy<T: SshRunner>(
    ssh_runner: &T,
    session: &Session,
) -> Result<(), Box<dyn Error>> {
    ssh_runner.run_command(
        session,
        "sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https",
    )?;
    ssh_runner.run_command(session, "curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg")?;
    ssh_runner.run_command(session, "curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list")?;
    ssh_runner.run_command(session, "sudo apt update")?;
    ssh_runner.run_command(session, "sudo apt install -y caddy")?;

    Ok(())
}
