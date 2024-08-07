// src/main.rs
mod commands;
mod utils;

use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long, value_parser, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy the Laravel application
    Deploy {
        #[clap(short, long, value_parser)]
        repo: String,

        #[clap(short, long, value_parser)]
        server: String,

        #[clap(short, long, action)]
        zero_downtime: bool,
    },
    /// Setup the server environment
    Setup {
        #[clap(short, long, value_parser)]
        server: String,
    },
    /// Rollback to the previous deployment
    Rollback {
        #[clap(short, long, value_parser)]
        server: String,
    },
    // Test SSH connect
    Connect {
        #[clap(short, long, value_parser)]
        server: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Read configuration
    let config = utils::config::read_config(cli.config)?;

    match &cli.command {
        Commands::Deploy { repo, server, zero_downtime } => {
            println!("Deploying application...");
            let session = utils::ssh::connect_ssh(server, &config)?;
            commands::deploy::deploy_app(&session, &config, repo, *zero_downtime)?;
        }
        Commands::Setup { server } => {
            println!("Setting up server environment...");
            let session = utils::ssh::connect_ssh(server, &config)?;
            commands::setup::setup_server(&session, &config)?;
        }
        Commands::Rollback { server } => {
            println!("Rolling back to previous deployment...");
            let session = utils::ssh::connect_ssh(server, &config)?;
            commands::rollback::rollback(&session, &config)?;
        }
        Commands::Connect { server } => {
            println!("Testing SSH connection...");
            let session = utils::ssh::connect_ssh(server, &config)?;
            commands::connect::test_connection(&session)?;
        }
    }

    Ok(())
}