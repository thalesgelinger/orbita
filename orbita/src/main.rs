use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,

    Add {
        #[arg(short, long)]
        resource_name: String,
    },

    Resolve,

    Run {
        #[arg(value_name = "FILE", required = false)]
        script: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init   => {
            println!("Initializing project...")
        }
        Command::Add { resource_name } => {
            println!("Adding resource: {}", resource_name);
        }
        Command::Resolve  => {
            println!("Resolving dependency:");
        }
        Command::Run { script } => {
            println!("Script: {:?}", script)
        }
    }
}
