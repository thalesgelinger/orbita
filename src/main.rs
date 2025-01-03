pub mod add;
pub mod config;
pub mod init;
pub mod resolve;
pub mod run;
pub mod utils;

use add::add;
use clap::{Parser, Subcommand};
use init::init;
use resolve::resolve;
use run::run;

/// Lua package manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a project
    Init {
        /// Skip and use default config
        #[arg(short, long)]
        yes: bool,
    },

    /// Add a resource to the project
    Add {
        /// Resource Name
        #[arg(required = true)]
        resource_name: String,
    },
    /// Resolve dependencies
    Resolve,

    /// Run the project with an optional Lua script
    Run {
        /// Lua script to run (optional, default uses Orbita file)
        #[arg(value_name = "FILE", required = false)]
        script: Option<String>,
    },
}

pub const DEPENDENCY_FILE: &str = "orbita.lua";

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init { yes } => init(yes),
        Command::Add { resource_name } => add(resource_name),
        Command::Resolve => resolve(),
        Command::Run { script } => match run(script) {
            Ok(_) => (),
            Err(e) => eprintln!("Error executing Lua script: {}", e),
        },
    }
}
