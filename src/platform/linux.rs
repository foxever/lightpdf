use super::PlatformAdapter;
use std::path::PathBuf;

pub struct LinuxPlatform;

impl PlatformAdapter for LinuxPlatform {
    fn open_file_dialog() -> Option<PathBuf> {
        // TODO: 实现 Linux 文件对话框
        None
    }
    
    fn register_shortcuts() {
        // TODO: 注册 Linux 快捷键
    }
}
