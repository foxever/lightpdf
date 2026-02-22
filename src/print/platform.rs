#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::PlatformPrinter;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::PlatformPrinter;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::PlatformPrinter;

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
mod stub;
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub use stub::PlatformPrinter;
