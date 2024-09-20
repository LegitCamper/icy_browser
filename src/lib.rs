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

pub struct ShortcutBuilder(Shortcuts);
impl ShortcutBuilder {
    pub fn new() -> Self {
        ShortcutBuilder(Vec::new())
    }

    pub fn add_shortcut(
        mut self,
        shortcut_action: ShortcutType,
        shortcut_keys: Vec<KeyType>,
    ) -> Self {
        if self.0.iter().filter(|sc| sc.0 == shortcut_action).count() != 0 {
            panic!("Tried to add a duplicated shortcut");
        }

        // Must have 1 char key
        if shortcut_keys
            .iter()
            .map(|item| {
                if let KeyType::Key(_) = item {
                    return true;
                } else if let KeyType::Modifier(_) = item {
                    return false;
                }
                unreachable!()
            })
            .filter(|item| *item == true)
            .count()
            != 1
        {
            panic!("Shortcuts MUST have ONLY one Charecter key")
        }

        // Must have at least one modifier key
        if shortcut_keys
            .iter()
            .map(|item| {
                if let KeyType::Key(_) = item {
                    return false;
                } else if let KeyType::Modifier(_) = item {
                    return true;
                }
                unreachable!()
            })
            .filter(|item| *item == true)
            .count()
            < 1
        {
            panic!("Shortcuts MUST have at least 1 Modifier key")
        }

        self.0.push((shortcut_action, shortcut_keys));
        self
    }

    pub fn build(self) -> Shortcuts {
        self.0
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
/// Configures Widget Keyboard Shortcut
pub type Shortcut = (ShortcutType, Vec<KeyType>);

/// Configures Widget Keyboard Shortcuts
pub type Shortcuts = Vec<Shortcut>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShortcutType {
    GoBackward,
    GoForward,
    Refresh,
    GoHome,
    CloseCurrentTab,
    CreateTab,
    ToggleOverlay,
    ShowOverlay,
    HideOverlay,
}

fn check_shortcut(shortcut: &Shortcut, key: &Key, modifiers: &Modifiers) -> bool {
    shortcut
        .1
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
