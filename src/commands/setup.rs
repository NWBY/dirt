// src/commands/setup.rs
use crate::utils::config::Config;
use crate::utils::ssh::run_command;
use crate::utils::file::write_file;
use ssh2::Session;
use std::error::Error;

pub fn setup_server(session: &Session, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Setting up server environment...");

    // Update package lists
    run_command(session, "sudo apt update")?;

    // Install common dependencies
    run_command(session, "sudo apt install -y software-properties-common curl zip unzip")?;

    // Install PHP and required extensions
    run_command(session, "sudo add-apt-repository ppa:ondrej/php -y")?;
    run_command(session, "sudo apt update")?;
    run_command(session, "sudo apt install -y php8.1 php8.1-fpm php8.1-cli php8.1-mbstring php8.1-xml php8.1-zip php8.1-pgsql php8.1-curl")?;

    // Install Composer
    run_command(session, "curl -sS https://getcomposer.org/installer | sudo php -- --install-dir=/usr/local/bin --filename=composer")?;

    // Install PostgreSQL
    run_command(session, "sudo apt install -y postgresql postgresql-contrib")?;
    
    // Setup PostgreSQL for Laravel
    run_command(session, &format!("sudo -u postgres psql -c \"CREATE USER {} WITH PASSWORD '{}';\"", config.db_user, config.db_password))?;
    run_command(session, &format!("sudo -u postgres psql -c \"CREATE DATABASE {} OWNER {}; \"", config.db_name, config.db_user))?;

    // Install Caddy
    run_command(session, "sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https")?;
    run_command(session, "curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg")?;
    run_command(session, "curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list")?;
    run_command(session, "sudo apt update")?;
    run_command(session, "sudo apt install -y caddy")?;

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