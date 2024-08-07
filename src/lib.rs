use iced::widget::image::{Handle, Image};

pub mod engines;
mod widgets;
pub use widgets::{browser_view, nav_bar, tab_bar, State};

fn bgr_to_rgb(image: Vec<u8>) -> Vec<u8> {
    assert_eq!(image.len() % 4, 0);
    image
        .chunks(4)
        .map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
        .flatten()
        .collect()
}

fn create_image(image: Vec<u8>, w: u32, h: u32, bgr: bool) -> Image<Handle> {
    let image = if bgr { bgr_to_rgb(image) } else { image };
    let handle = Handle::from_pixels(w, h, image);
    Image::new(handle)
}

// fn get_website_icon(url: &str) -> Image<Handle> {
//     let res = reqwest::get(format!("{}/favicon.ico", url));
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;
// }
