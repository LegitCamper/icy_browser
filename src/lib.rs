pub use iced_fonts::BOOTSTRAP_FONT_BYTES;
pub use iced_on_focus_widget::hoverable;

mod engines;
pub use engines::{BrowserEngine, PixelFormat, Tab, TabInfo, Tabs};

#[cfg(feature = "ultralight")]
pub use engines::ultralight::Ultralight;

pub mod widgets;
pub use widgets::{
    browser_view, command_palette, nav_bar, tab_bar, HomepageType, IcyBrowser, Message,
};

mod helpers;
pub use helpers::{get_fonts, to_url, Bookmark, Bookmarks, ImageInfo};

mod shortcut;
pub use shortcut::{
    shortcut_pressed, KeyType, Shortcut, ShortcutBuilder, ShortcutModifier, Shortcuts,
};

/// Allows different widgets to interact in their native way
#[derive(Debug, Clone, PartialEq)]
pub enum TabSelectionType {
    Id(u32),
    Index(usize),
}
impl Default for TabSelectionType {
    fn default() -> Self {
        TabSelectionType::Index(0)
    }
}
