use std::env;
use std::path::{PathBuf};

pub fn to_absolute(path: PathBuf) -> PathBuf {
  if path.is_absolute() {
    return path;
  }

  env::current_dir()
    .expect("Failed to get current directory").join(path)
}
