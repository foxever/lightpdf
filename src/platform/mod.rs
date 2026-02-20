#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[allow(dead_code)]
use std::path::PathBuf;

#[allow(dead_code)]
pub trait PlatformAdapter {
    fn open_file_dialog() -> Option<PathBuf>;
    fn register_shortcuts();
}
