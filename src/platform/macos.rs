use super::PlatformAdapter;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct MacOSPlatform;

#[allow(dead_code)]
impl PlatformAdapter for MacOSPlatform {
    fn open_file_dialog() -> Option<PathBuf> {
        // TODO: 实现 macOS 文件对话框
        None
    }

    fn register_shortcuts() {
        // TODO: 注册 macOS 快捷键
    }
}
