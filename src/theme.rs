use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub background: Rgba,
    pub background_secondary: Rgba,
    pub background_tertiary: Rgba,
    pub text: Rgba,
    pub text_secondary: Rgba,
    pub border: Rgba,
    pub toolbar: Rgba,
    pub status_bar: Rgba,
    pub pdf_view: Rgba,
    pub moon_color: Rgba,
    pub sun_color: Rgba,
}

impl ThemeColors {
    pub fn for_theme(theme: Theme) -> Self {
        match theme {
            Theme::Light => Self::light(),
            Theme::Dark => Self::dark(),
        }
    }

    fn light() -> Self {
        Self {
            background: rgb(0xffffff).into(),
            background_secondary: rgb(0xf5f5f5).into(),
            background_tertiary: rgb(0xe8e8e8).into(),
            text: rgb(0x1a1a1a).into(),
            text_secondary: rgb(0x666666).into(),
            border: rgb(0xd0d0d0).into(),
            toolbar: rgb(0xe0e0e0).into(),
            status_bar: rgb(0xe0e0e0).into(),
            pdf_view: rgb(0xf0f0f0).into(),
            moon_color: rgb(0x1a1a1a).into(),
            sun_color: rgb(0xffcc00).into(),
        }
    }

    fn dark() -> Self {
        Self {
            background: rgb(0x333333).into(),
            background_secondary: rgb(0x2b2b2b).into(),
            background_tertiary: rgb(0x404040).into(),
            text: rgb(0xcccccc).into(),
            text_secondary: rgb(0x888888).into(),
            border: rgb(0x1a1a1a).into(),
            toolbar: rgb(0x2b2b2b).into(),
            status_bar: rgb(0x2b2b2b).into(),
            pdf_view: rgb(0x404040).into(),
            moon_color: rgb(0xcccccc).into(),
            sun_color: rgb(0xffdd44).into(),
        }
    }
}
