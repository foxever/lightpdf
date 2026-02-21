use dirs_next::config_dir;
use std::path::PathBuf;

const APP_NAME: &str = "lingpdf";

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(APP_NAME);
    let _ = std::fs::create_dir_all(&path);
    path.push("config.json");
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
}
