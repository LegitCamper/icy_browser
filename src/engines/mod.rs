use iced::keyboard;
use iced::mouse;
use iced::widget::image::{Handle, Image};
use iced::{event::Status, Point};

#[cfg(feature = "webkit")]
pub mod ultralight;

#[allow(unused)]
pub trait BrowserEngine {
    fn new(width: u32, height: u32) -> Self;

    fn do_work(&self);
    fn need_render(&self) -> bool;
    fn render(&mut self);
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, width: u32, height: u32);
    fn pixel_buffer(&mut self) -> Option<Vec<u8>>;
    fn get_image(&mut self) -> Option<Image<Handle>>;

    fn get_title(&self) -> Option<String>;
    fn get_url(&self) -> Option<String>;
    fn goto_url(&self, url: &str);
    fn has_loaded(&self) -> bool;
    fn new_tab(&mut self, url: &str);
    fn goto_tab(&mut self, url: &str) -> Option<()>;

    fn refresh(&self);
    fn go_forward(&self);
    fn go_back(&self);
    fn focus(&self);
    fn unfocus(&self);

    fn scroll(&self, delta: mouse::ScrollDelta) -> Status;
    fn handle_keyboard_event(&self, event: keyboard::Event) -> Status;
    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event) -> Status;
}

fn bgr_to_rgb(image: Vec<u8>) -> Vec<u8> {
    image
        .chunks(4)
        .map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
        .flatten()
        .collect()
}

pub fn create_image(image: Vec<u8>, w: u32, h: u32, bgr: bool) -> Image<Handle> {
    let image = if bgr { bgr_to_rgb(image) } else { image };
    let handle = Handle::from_pixels(w, h, image);
    Image::new(handle)
}

pub fn create_empty_view(w: u32, h: u32) -> Image<Handle> {
    let mut image: Vec<u8> = Vec::new();
    for _ in 0..(w * h) {
        image.push(255);
    }
    create_image(image, w, h, false)
}
