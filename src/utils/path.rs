use dirs_next::{cache_dir, config_dir};
use std::path::PathBuf;

const APP_NAME: &str = "lightpdf";

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(APP_NAME);
    let _ = std::fs::create_dir_all(&path);
    path.push("config.json");
    path
}

pub fn get_cache_dir() -> PathBuf {
    let mut path = cache_dir().unwrap_or_else(|| PathBuf::from("./cache"));
    path.push(APP_NAME);
    let _ = std::fs::create_dir_all(&path);
    path
}

pub fn get_bookmarks_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(APP_NAME);
    let _ = std::fs::create_dir_all(&path);
    path.push("bookmarks.json");
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path() {
        let path = get_config_path();
        assert!(path.to_string_lossy().contains(APP_NAME));
        assert!(path.to_string_lossy().contains("config.json"));
    }

    #[test]
    fn test_cache_dir() {
        let path = get_cache_dir();
        assert!(path.to_string_lossy().contains(APP_NAME));
    }
}
