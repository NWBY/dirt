// src/main.rs
mod commands;
mod utils;

use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;
use utils::ssh::DirtSshRunner;

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
    Connect {},
    Init {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Read configuration
    let config = utils::config::read_config(cli.config)?;
    let ssh_runner = DirtSshRunner::new();

    match &cli.command {
        Commands::Deploy {
            repo,
            server,
            zero_downtime,
        } => {
            println!("Deploying application...");
            let session = utils::ssh::connect_ssh(&config)?;
            commands::deploy::deploy_app(&session, &config, repo, *zero_downtime)?;
        }
        Commands::Setup { server } => {
            println!("Setting up server environment...");
            let session = utils::ssh::connect_ssh(&config)?;
            commands::setup::setup_server(&ssh_runner, &session, &config)?;
        }
        Commands::Rollback { server } => {
            println!("Rolling back to previous deployment...");
            let session = utils::ssh::connect_ssh(&config)?;
            commands::rollback::rollback(&session, &config)?;
        }
        Commands::Connect {} => {
            let session = utils::ssh::connect_ssh(&config)?;
            commands::connect::test_connection(&ssh_runner, &session)?;
        }
        Commands::Init {  } => {
            commands::init::init()?;
        }
    }

    Ok(())
}
