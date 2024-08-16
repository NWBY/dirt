use crate::utils::caddy::install_caddy;
// src/commands/setup.rs
use crate::utils::config::Config;
use crate::utils::file::write_file;
use crate::utils::ssh::{run_command, SshRunner};
use ssh2::Session;
use std::error::Error;

pub fn setup_server<T: SshRunner>(
    ssh_runner: &T,
    session: &Session,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    println!("Setting up server environment...");
    
    todo!("check if php version is supported before proceeding");

    // Update package lists
    ssh_runner.run_command(session, "sudo apt update")?;

    // Install common dependencies
    ssh_runner.run_command(
        session,
        "sudo apt install -y software-properties-common curl zip unzip",
    )?;
    
    let php_version = format!("php{}", config.php_version);
    let php_install_script = format!("sudo apt install -y {php_version} {php_version}-fpm {php_version}-cli {php_version}-mbstring {php_version}-xml {php_version}-zip {php_version}-pgsql {php_version}-curl");

    // Install PHP and required extensions
    run_command(session, "sudo add-apt-repository ppa:ondrej/php -y")?;
    run_command(session, "sudo apt update")?;
    run_command(session, &php_install_script)?;

    // Install Composer
    run_command(session, "curl -sS https://getcomposer.org/installer | sudo php -- --install-dir=/usr/local/bin --filename=composer")?;

    // Install PostgreSQL
    run_command(session, "sudo apt install -y postgresql postgresql-contrib")?;

    // Setup PostgreSQL for Laravel
    run_command(
        session,
        &format!(
            "sudo -u postgres psql -c \"CREATE USER {} WITH PASSWORD '{}';\"",
            config.db_user, config.db_password
        ),
    )?;
    run_command(
        session,
        &format!(
            "sudo -u postgres psql -c \"CREATE DATABASE {} OWNER {}; \"",
            config.db_name, config.db_user
        ),
    )?;
    
    install_caddy(ssh_runner, session).expect("Unable to install Caddy");
    println!("Installed Caddy!");

    // Setup Caddy configuration
    let caddy_config = r#"
:80 {
    root * /var/www/laravel/public
    php_fastcgi unix//var/run/php/php8.1-fpm.sock
    file_server
    encode gzip
    log {
        output file /var/log/caddy/access.log
        format json
    }
}
    "#;
    write_file(session, "/etc/caddy/Caddyfile", caddy_config)?;

    // Restart services
    run_command(session, "sudo systemctl restart php8.1-fpm")?;
    run_command(session, "sudo systemctl restart caddy")?;

    // Setup Laravel directory
    run_command(session, "sudo mkdir -p /var/www/laravel")?;
    run_command(session, "sudo chown -R www-data:www-data /var/www/laravel")?;

    println!("Server setup completed successfully!");
    Ok(())
}
