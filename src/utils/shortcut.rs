#[derive(Clone, Copy, Debug)]
pub enum ShortcutAction {
    OpenFile,
    ZoomIn,
    ZoomOut,
    ZoomReset,
    PrevPage,
    NextPage,
    FullScreen,
    AddBookmark,
    Exit,
}

pub struct ShortcutHandler;

impl ShortcutHandler {
    pub fn handle(action: ShortcutAction) {
        match action {
            ShortcutAction::OpenFile => {
                log::info!("快捷键: 打开文件");
            }
            ShortcutAction::ZoomIn => {
                log::info!("快捷键: 放大");
            }
            ShortcutAction::ZoomOut => {
                log::info!("快捷键: 缩小");
            }
            ShortcutAction::ZoomReset => {
                log::info!("快捷键: 重置缩放");
            }
            ShortcutAction::PrevPage => {
                log::info!("快捷键: 上一页");
            }
            ShortcutAction::NextPage => {
                log::info!("快捷键: 下一页");
            }
            ShortcutAction::FullScreen => {
                log::info!("快捷键: 全屏");
            }
            ShortcutAction::AddBookmark => {
                log::info!("快捷键: 添加书签");
            }
            ShortcutAction::Exit => {
                log::info!("快捷键: 退出");
            }
        }
    }
}
