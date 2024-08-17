use iced::widget::image::{Handle, Image};
use std::fs::{remove_file, File};
use std::io::{self, copy, Write};

use std::path::PathBuf;
use tempfile::Builder;
use url::{ParseError, Url};

mod engines;
pub use engines::BrowserEngine;

#[cfg(feature = "ultralight")]
pub use engines::ultralight::Ultralight;

mod widgets;
pub use widgets::{browser_widgets, nav_bar, tab_bar, BrowserView, BrowserWidget};

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

    fn new(pixels: Vec<u8>, width: u32, height: u32) -> Self {
        assert_eq!(pixels.len() % 4, 0);
        Self {
            pixels,
            width,
            height,
        }
    }

    fn new_from_bgr(pixels: Vec<u8>, width: u32, height: u32) -> Self {
        let pixels = pixels
            .chunks(4)
            .flat_map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
            .collect();
        Self::new(pixels, width, height)
    }

    fn as_image(&self) -> Image<Handle> {
        let handle = Handle::from_pixels(self.width, self.height, self.pixels.clone());
        Image::new(handle)
    }
}

fn copy_bytes_to_file(name: &str, bytes: &[u8], mut path: PathBuf) -> Result<(), io::Error> {
    path.push(name);
    let _ = remove_file(path.clone()); // file already exists
    let mut file = std::fs::File::create(&path)?;
    file.write_all(&bytes[..])?;
    Ok(())
}

// This function has to be called in a iced Command/Task future
// if path is false, its downloaded to a temp dir
async fn download_file(path: Option<PathBuf>, url: &str) -> Option<()> {
    let tmp_dir = Builder::new().prefix("Rust-Browser_Cache").tempdir().ok()?;
    let response = reqwest::get(url).await.ok()?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        let path = match path {
            Some(path) => tmp_dir.path().join(path),
            None => tmp_dir.path().join(fname),
        };

        File::create(path).ok()?
    };

    let content = response.text().await.ok()?;
    copy(&mut content.as_bytes(), &mut dest).ok()?;
    Some(())
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
