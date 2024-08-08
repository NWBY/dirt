use crate::utils::config::Config;
use crate::utils::ssh::run_command;
use crate::utils::file::write_file;
use chrono::Local;
use ssh2::Session;
use std::error::Error;

pub fn deploy_app(session: &Session, config: &Config, repo: &str, zero_downtime: bool) -> Result<(), Box<dyn Error>> {
    println!("Starting deployment process...");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let release_path = format!("/var/www/{}/releases/{}", config.name, timestamp);
    let current_path = format!("/var/www/{}/current", config.name);

    // Create new release directory
    run_command(session, &format!("mkdir -p {}", release_path))?;

    // Clone repository
    run_command(session, &format!("git clone --depth 1 {} {}", repo, release_path))?;

    // Install composer dependencies
    run_command(session, &format!("cd {} && composer install --no-dev --optimize-autoloader", release_path))?;

    // Create .env file
    let env_contents = format!(
        "APP_ENV=production
        APP_DEBUG=false
        APP_KEY=
        DB_CONNECTION=pgsql
        DB_HOST=127.0.0.1
        DB_PORT=5432
        DB_DATABASE={}
        DB_USERNAME={}
        DB_PASSWORD={}",
        config.db_name, config.db_user, config.db_password
    );
    write_file(session, &format!("{}/.env", release_path), &env_contents)?;

    // Generate application key
    run_command(session, &format!("cd {} && php artisan key:generate", release_path))?;

    // Run database migrations
    run_command(session, &format!("cd {} && php artisan migrate --force", release_path))?;

    // Compile assets
    run_command(session, &format!("cd {} && npm install && npm run production", release_path))?;

    // Optimize Laravel
    run_command(session, &format!("cd {} && php artisan optimize", release_path))?;

    // Update symlinks
    if zero_downtime {
        // Create new symlink
        run_command(session, &format!("ln -sfn {} {}", release_path, current_path))?;
    } else {
        // Remove old symlink and create new one
        run_command(session, &format!("rm -f {} && ln -s {} {}", current_path, release_path, current_path))?;
    }

    // Restart PHP-FPM
    run_command(session, "sudo systemctl restart php8.1-fpm")?;

    // Clear old releases (keep last 5)
    run_command(session, "cd /var/www/laravel/releases && ls -t | tail -n +6 | xargs -r rm -rf")?;

    println!("Deployment completed successfully!");
    Ok(())
}