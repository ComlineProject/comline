// Relative Modules
pub mod client;
pub mod manager;
pub mod tui;

#[cfg(test)]
pub mod tests;


// Standard Uses
use std::path::Path;

// Crate Uses
use crate::client::{create_empty_package_at, dependency};

// External Uses
use comline_core::project;
use comline_core::project::ir::frozen as project_frozen;

use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(name = "Comline Package Manager")]
#[command(author = "Comline")]
#[command(version = "0.0.1")]
#[command(
    about = "Package Manager and Publisher tool for Comline (also compiles things)",
    long_about = None)
]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// Create new package
    New {
        name: String,

        #[arg(short, long, default_value="false")]
        force: bool
    },

    /// Add dependency to package
    Add {
        dependency: String,

        #[arg(short, long, default_value="false")]
        local: bool
    },

    /// Build package
    Build,

    /// Publish package
    Publish {
        /// The registries on where to publish, divided by space
        #[arg(long)]
        registries: String,
    }
}


pub fn main() {
    let cli = Cli::parse();

    use Commands::*;
    match &cli.command {
        New { name, force } => {
            println!("Creating new package named '{name}'");
            let cur_path = std::env::current_dir().unwrap();

            let new_package_path = cur_path.join(name);
            if let Err(e) = create_empty_package_at(&new_package_path, name, force) {
                eprintln!("Could not create package: {}", e);
                return
            };

            println!("Package created at {}", new_package_path.display());
        },
        Add { dependency, local } => {
            println!("Looking for dependency...");
            let package_path = std::env::current_dir().unwrap();

            if *local {
                println!("Searching for local package at \"{}\"", dependency);

                let dependency_path = Path::new(dependency);
                if dependency_path.exists() {
                    eprintln!("Given package path does not exist")
                }

                if !project::is_package_path(dependency_path) {
                    eprintln!("Given package path does not seem to be a valid package")
                }

                return;
            }

            if let Err(e) = dependency::add_remote_dependency(
                dependency.clone(), &package_path
            ) {
                eprintln!("Cannot add dependency: \n - {}", e)
            }

            todo!()
        }
        Build => {
            println!("Starting build process...");
            let package_path = std::env::current_dir().unwrap();

            match project::build::build(&package_path) {
                Ok(ctx) => {
                    println!(
                        "Built package to latest version ({}) 🐸",
                        project::ir::frozen::version(&ctx.config_frozen.unwrap())
                            .unwrap()
                    )
                }
                Err(e) => eprintln!("Couldn't build package: \n - {}", e)
            }
        },
        Publish { registries } => {
            let parts: Vec<String> = registries.split_whitespace()
                .map(|p| p.to_owned()).collect();
            println!(
                "Starting process to publish into '{}' registries: {}",
                parts.len(), registries
            );

            println!("Building package before trying to publish");
            let package_path = std::env::current_dir().unwrap();
            let package_ctx = project::build::build(&package_path)
                .unwrap();

            let package_config = package_ctx.config_frozen.as_ref().unwrap();
            let package_name = project_frozen::namespace(package_config).unwrap();
            let package_version = project_frozen::version(package_config).unwrap();
            println!(
                "Package '{}'#'{}' is okay, starting publish in registries",
                package_name, package_version
            );
            if let Err(error) = client::publish::publish_to_registries(
                &package_ctx, parts
            ) {
                eprintln!("{}", error);
                return
            }
        }
    }
}
