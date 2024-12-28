use std::{fs, path::Path, sync::Arc};

use mlua::{Error, Lua, Table};

pub fn run(script: &str) -> Result<(), Error> {
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
