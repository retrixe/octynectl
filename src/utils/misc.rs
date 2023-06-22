use std::{path::{Path, PathBuf}, env};

pub fn default_octyne_path() -> PathBuf {
    Path::new(&env::temp_dir()).join("octyne.sock.42069")
}
