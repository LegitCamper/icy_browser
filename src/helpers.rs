use iced::widget::{
    button,
    image::{Handle, Image},
    Button,
};
pub use iced_fonts::BOOTSTRAP_FONT_BYTES;
use std::{borrow::Cow, fmt, str::FromStr};
use url::{ParseError, Url};

use super::{Message, PixelFormat};

// Helper function to ensure required icons are imported
pub fn get_fonts() -> Vec<Cow<'static, [u8]>> {
    vec![BOOTSTRAP_FONT_BYTES.into()]
}

// Image details for passing the view around
#[derive(Clone, Debug, PartialEq)]
pub struct ImageInfo {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
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

    pub fn as_image(&self) -> Image<Handle> {
        Image::new(Handle::from_rgba(
            self.width,
            self.height,
            self.pixels.clone(),
        ))
    }
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

    pub fn as_button(&self) -> Button<Message> {
        button(self.name.as_str()).on_press(Message::GoToUrl(self.url.to_string()))
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
