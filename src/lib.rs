use iced::keyboard::{Key, Modifiers};
use iced::widget::image::{Handle, Image};
use url::{ParseError, Url};

mod engines;
pub use engines::{BrowserEngine, PixelFormat, Tab, TabInfo, Tabs};

#[cfg(feature = "ultralight")]
pub use engines::ultralight::Ultralight;

pub mod widgets;
pub use widgets::{nav_bar, tab_bar, BrowserWidget};

// Image details for passing the view around
#[derive(Debug, Clone)]
pub struct ImageInfo {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

impl Default for ImageInfo {
    fn default() -> Self {
        Self {
            pixels: vec![255; (Self::WIDTH as usize * Self::HEIGHT as usize) * 4],
            width: Self::WIDTH,
            height: Self::HEIGHT,
        }
    }
}

impl ImageInfo {
    // The default dimentions
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;

    pub fn new(pixels: Vec<u8>, format: PixelFormat, width: u32, height: u32) -> Self {
        // R, G, B, A
        assert_eq!(pixels.len() % 4, 0);

        let pixels = match format {
            PixelFormat::Rgba => pixels,
            PixelFormat::Bgra => pixels
                .chunks(4)
                .flat_map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
                .collect(),
        };

        Self {
            pixels,
            width,
            height,
        }
    }

    fn as_image(&self) -> Image<Handle> {
        Image::new(Handle::from_rgba(
            self.width,
            self.height,
            self.pixels.clone(),
        ))
    }
}

fn to_url(url: &str) -> Option<Url> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShortcutModifier {
    Shift,
    Ctrl,
    Alt,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyType {
    Key(iced::keyboard::Key),
    Modifier(ShortcutModifier),
}
/// Configures Widget Keyboard Shortcuts
pub type Shortcuts = Vec<Shortcut>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shortcut {
    GoBackward(Vec<KeyType>),
    GoForward(Vec<KeyType>),
    Refresh(Vec<KeyType>),
    GoHome(Vec<KeyType>),
    CloseTab(Vec<KeyType>),
    CreateTab(Vec<KeyType>),
    ShowOverlay(Vec<KeyType>),
    HideOverlay(Vec<KeyType>),
}

fn check_shortcut(shortcut: &Vec<KeyType>, key: &Key, modifiers: &Modifiers) -> bool {
    shortcut
        .iter()
        .map(|s| match s {
            KeyType::Key(s_key) => {
                if let iced::keyboard::Key::Character(s_char) = s_key {
                    if let iced::keyboard::Key::Character(key_char) = key {
                        key_char == s_char
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            KeyType::Modifier(s_mod) => match s_mod {
                ShortcutModifier::Shift => modifiers.shift(),
                ShortcutModifier::Ctrl => modifiers.control(),
                ShortcutModifier::Alt => modifiers.alt(),
            },
        })
        .all(|s| s == true)
}
