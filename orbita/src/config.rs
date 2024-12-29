use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    sync::Arc,
};

use mlua::{Error, Lua, Table};

use crate::DEPENDENCY_FILE;

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub src: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub description: String,
    pub main: String,
    pub dependencies: Vec<Dependency>,
    pub author: String,
    pub license: String,
}

impl Config {
    pub fn new() -> Self {
        let current_directory = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        Config {
            name: current_directory.to_string_lossy().to_string(),
            version: "1.0.0".to_string(),
            description: String::new(),
            main: "main.lua".to_string(),
            dependencies: Vec::new(),
            author: String::new(),
            license: "ISC".to_string(),
        }
    }

    pub fn load() -> Result<Self, Error> {
        let lua = Lua::new();

        let lua_code =
            fs::read_to_string(DEPENDENCY_FILE).map_err(|e| Error::ExternalError(Arc::new(e)))?;

        let config_table: mlua::Table = lua.load(&lua_code).eval()?;

        let name: String = config_table.get("name")?;
        let version: String = config_table.get("version")?;
        let description: String = config_table.get("description")?;
        let main: String = config_table.get("main")?;
        let author: String = config_table.get("author")?;
        let license: String = config_table.get("license")?;

        let dependencies_table: Table = config_table.get("dependencies")?;

        let mut dependencies = Vec::new();

        for i in 1..=dependencies_table.len()? {
            let dep_table: mlua::Table = dependencies_table.get(i)?;
            let name: String = dep_table.get(1)?;
            let version: Option<String> = match dep_table.get(2) {
                Ok(val) => Some(val),
                Err(_) => None,
            };

            let src: Option<String> = match dep_table.get("src") {
                Ok(val) => Some(val),
                Err(_) => None,
            };

            dependencies.push(Dependency { name, src, version });
        }
        Ok(Config {
            name,
            version,
            description,
            main,
            dependencies,
            author,
            license,
        })
    }

    fn to_string(&self) -> String {
        let mut result = String::from("{\n");
        result.push_str(&format!("\tname = \"{}\",\n", self.name));
        result.push_str(&format!("\tversion = \"{}\",\n", self.version));
        result.push_str(&format!("\tdescription = \"{}\",\n", self.description));
        result.push_str(&format!("\tmain = \"{}\",\n", self.main));
        result.push_str("\tdependencies = {},\n");
        result.push_str(&format!("\tauthor = \"{}\",\n", self.author));
        result.push_str(&format!("\tlicense = \"{}\",\n", self.license));
        result.push_str("}\n");
        result
    }

    pub fn write(&self) -> io::Result<()> {
        let mut file = File::create(DEPENDENCY_FILE)?;
        file.write_all(format!("return {}\n", self.to_string()).as_bytes())?;
        Ok(())
    }
}
