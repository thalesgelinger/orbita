use std::{fs, path::Path, sync::Arc};

use clap::{Parser, Subcommand};
use mlua::{Error, Lua, Table};

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
            Some(s) => match run_lua_script(&s) {
                Ok(_) => (),
                Err(e) => eprintln!("Error executing Lua script: {}", e),
            },
            None => todo!(),
        },
    }
}

fn run_lua_script(script: &str) -> Result<(), Error> {
    let lua = Lua::new();

    let script_dir = Path::new(script).parent().unwrap_or(Path::new("."));
    let lua_path = format!("{}/?.lua", script_dir.display());

    let _ = lua.globals().set("package", lua.create_table()?);
    let package = lua.globals().get::<Table>("package")?;
    package.set("path", lua_path)?;

    let script_content = match fs::read_to_string(script) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading Lua script file: {}", e);
            return Err(Error::ExternalError(Arc::new(e)));
        }
    };

    match lua.load(&script_content).exec() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error executing Lua code: {}", e);
            Err(e)
        }
    }
}
