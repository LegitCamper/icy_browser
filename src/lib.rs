pub use iced_fonts::BOOTSTRAP_FONT_BYTES;
pub use iced_on_focus_widget::hoverable;

mod engines;
pub use engines::{BrowserEngine, PixelFormat, Tab, TabInfo, Tabs};

#[cfg(feature = "ultralight")]
pub use engines::ultralight::Ultralight;

pub mod widgets;
pub use widgets::{browser_view, command_palatte, nav_bar, tab_bar, IcyBrowser, Message};

mod helpers;
pub use helpers::{get_fonts, to_url, Bookmark, Bookmarks, ImageInfo};

mod shortcut;
pub use shortcut::{KeyType, Shortcut, ShortcutBuilder, ShortcutModifier, Shortcuts};
