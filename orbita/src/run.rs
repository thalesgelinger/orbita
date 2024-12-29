use std::{fs, sync::Arc};

use crate::config::Config;
use mlua::{Error, Lua, Table};

pub fn run(script: Option<String>) -> Result<(), Error> {
    let lua = Lua::new();
    let config = Config::load()?;

    include_dependencies_in_path(&lua, &config)?;

    match script {
        Some(s) => run_file(&lua, &s),
        None => run_file(&lua, &config.main),
    }
}

fn include_dependencies_in_path(lua: &Lua, config: &Config) -> Result<(), Error> {
    let package = lua.globals().get::<Table>("package")?;

    let mut package_path = package.get::<String>("path")?;

    for dep in &config.dependencies {
        if let Some(src) = &dep.src {
            package_path.push_str(&format!(";{}/?.lua", src));
            package_path.push_str(&format!(";{}/?/?.lua", src));
        }
    }

    package.set("path", package_path)?;

    Ok(())
}

fn run_file(lua: &Lua, script: &str) -> Result<(), Error> {
    let package = lua.globals().get::<Table>("package")?;
    let mut path = package.get::<String>("path")?;

    let current_dir = std::env::current_dir()?.display().to_string();
    path.push_str(&format!(";{}/?.lua", current_dir));
    path.push_str(&format!(";{}/?/?.lua", current_dir));

    package.set("path", path)?;

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
