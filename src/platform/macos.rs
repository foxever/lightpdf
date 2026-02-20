use super::PlatformAdapter;
use std::path::PathBuf;

pub struct MacOSPlatform;

impl PlatformAdapter for MacOSPlatform {
    fn open_file_dialog() -> Option<PathBuf> {
        // TODO: 实现 macOS 文件对话框
        None
    }
    
    fn register_shortcuts() {
        // TODO: 注册 macOS 快捷键
    }
}
