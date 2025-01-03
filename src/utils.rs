use std::path::PathBuf;

pub fn get_orbita_base_dir() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    home_dir.join(".orbita")
}
