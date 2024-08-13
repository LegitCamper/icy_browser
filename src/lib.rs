use iced::widget::image::{Handle, Image};
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use tempfile::Builder;
use url::{ParseError, Url};

mod engines;
pub use engines::BrowserEngine;

#[cfg(feature = "ultralight")]
pub use engines::ultralight::Ultralight;

mod widgets;
pub use widgets::{
    browser_view, browser_widgets, nav_bar, tab_bar, BrowserView, BrowserWidget, NavBar, TabBar,
};

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

fn bgr_to_rgb(image: Vec<u8>) -> Vec<u8> {
    assert_eq!(image.len() % 4, 0);
    image
        .chunks(4)
        .flat_map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
        .collect()
}

fn create_image(image: Vec<u8>, w: u32, h: u32, bgr: bool) -> Image<Handle> {
    let image = if bgr { bgr_to_rgb(image) } else { image };
    let handle = Handle::from_pixels(w, h, image);
    Image::new(handle)
}

fn create_empty_view(w: u32, h: u32) -> Image<Handle> {
    let image = vec![255; w as usize * h as usize];
    create_image(image, w, h, false)
}
