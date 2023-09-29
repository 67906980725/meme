use std::path::Path;

pub fn is_txt(path: &Path) -> bool {
  path.extension()
    .map(|suffix| suffix.to_str().unwrap())
    .map(|str| str.to_lowercase() == "txt")
    .unwrap_or(false)
}
