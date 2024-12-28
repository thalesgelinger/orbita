use clap::{Parser, Subcommand};

/// Simple program to perform various tasks
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Subcommands
    #[command(subcommand)]
    command: Command,
}

/// Enum to represent different commands
#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a project
    Init {
        /// Path to initialize the project at
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Add a resource to the project
    Add {
        /// Name of the resource to add
        #[arg(short, long)]
        resource_name: String,
    },

    /// Resolve dependencies or tasks
    Resolve {
        /// Dependency to resolve
        #[arg(short, long)]
        dependency: String,
    },

    /// Run the project
    Run {
        /// Run in debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init { path } => {
            println!("Initializing project{}...", path.unwrap_or_else(|| " in the current directory".to_string()));
        }
        Command::Add { resource_name } => {
            println!("Adding resource: {}", resource_name);
        }
        Command::Resolve { dependency } => {
            println!("Resolving dependency: {}", dependency);
        }
        Command::Run { debug } => {
            if debug {
                println!("Running in debug mode...");
            } else {
                println!("Running the project...");
            }
        }
    }
}
