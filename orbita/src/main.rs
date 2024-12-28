pub mod run;

use clap::{Parser, Subcommand};
use run::run;

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
        Command::Init => {
            println!("Initializing project...")
        }
        Command::Add { resource_name } => {
            println!("Adding resource: {}", resource_name);
        }
        Command::Resolve => {
            println!("Resolving dependency:");
        }
        Command::Run { script } => match script {
            Some(s) => match run(&s) {
                Ok(_) => (),
                Err(e) => eprintln!("Error executing Lua script: {}", e),
            },
            None => todo!(),
        },
    }
}
