use crate::utils::config::Config;
use crate::utils::ssh::run_command;
use crate::utils::ssh::run_command_with_output;
use ssh2::Session;
use std::error::Error;

pub fn rollback(session: &Session, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Starting rollback process...");

    let releases_path = "/var/www/laravel/releases";
    let current_path = "/var/www/laravel/current";

    // Get the list of releases
    let output = run_command_with_output(session, &format!("ls -1t {}", releases_path))?;
    let releases: Vec<&str> = output.split_whitespace().collect();

    if releases.len() < 2 {
        return Err("Not enough releases to perform a rollback".into());
    }

    let current_release = releases[0];
    let previous_release = releases[1];

    println!("Rolling back from {} to {}", current_release, previous_release);

    // Update the symlink to point to the previous release
    run_command(session, &format!("ln -sfn {}/{} {}", releases_path, previous_release, current_path))?;

    // Restart PHP-FPM
    run_command(session, "sudo systemctl restart php8.1-fpm")?;

    // Run database migrations rollback
    run_command(session, &format!("cd {} && php artisan migrate:rollback --step=1", current_path))?;

    // Remove the current (now outdated) release
    run_command(session, &format!("rm -rf {}/{}", releases_path, current_release))?;

    println!("Rollback completed successfully!");
    Ok(())
}