use iced::keyboard;
use iced::mouse;
use iced::{event::Status, Point};

#[allow(unused)]
pub trait BrowserEngine {
    fn new(width: u32, height: u32) -> Self;

    fn do_work(&self);
    fn render(&self);
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, width: u32, height: u32);
    fn pixel_buffer(&mut self) -> Option<Vec<u8>>;

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

    fn scroll(&self, delta: iced::mouse::ScrollDelta) -> Status;
    fn handle_keyboard_event(&self, event: keyboard::Event) -> Status;
    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event) -> Status;
}

#[cfg(feature = "webkit")]
pub mod ultralight;
