use iced::widget::{button, Button};
pub use iced_fonts::BOOTSTRAP_FONT_BYTES;
pub use iced_on_focus_widget::hoverable;
pub use iced_webview;
use std::{borrow::Cow, fmt, str::FromStr};
use url::{ParseError, Url};

pub mod bookmark_bar;
pub use bookmark_bar::bookmark_bar;
// pub mod command_palette;
pub mod nav_bar;
pub mod tab_bar;

// mod shortcut;
// pub use shortcut::{
//     shortcut_pressed, KeyType, Shortcut, ShortcutBuilder, ShortcutModifier, Shortcuts,
// };

// Helper function to ensure required icons are imported
pub fn get_fonts() -> Vec<Cow<'static, [u8]>> {
    vec![BOOTSTRAP_FONT_BYTES.into()]
}

pub fn to_url(url: &str) -> Option<Url> {
    match Url::parse(url) {
        Ok(url) => Some(url),
        Err(error) => {
            if let ParseError::RelativeUrlWithoutBase = error {
                let mut base = String::from("https://");
                base.push_str(url);
                Url::parse(&base).ok()
            } else {
                None
            }
        }
    }
}

pub type Bookmarks = Vec<Bookmark>;

#[derive(Debug, Clone, PartialEq)]
pub struct Bookmark {
    url: Url,
    name: String,
    // icon: Optional<>
}

impl Bookmark {
    pub fn new(url: &str, name: &str) -> Self {
        Bookmark {
            url: Url::from_str(url).expect("Failed to parse url from bookmark url"),
            name: name.to_string(),
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
