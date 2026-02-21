use super::PlatformAdapter;
use std::path::PathBuf;

pub struct WindowsPlatform;

impl PlatformAdapter for WindowsPlatform {
    fn open_file_dialog() -> Option<PathBuf> {
        // TODO: 实现 Windows 文件对话框
        None
    }

    fn register_shortcuts() {
        // TODO: 注册 Windows 快捷键
    }
}
