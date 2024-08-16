use iced::keyboard;
use iced::mouse::{self, Interaction};
// use iced::widget::image::{Handle, Image};
use iced::Point;
use url::Url;

#[cfg(feature = "webkit")]
pub mod ultralight;

#[derive(Debug, Clone)]
pub struct Tab {
    pub url: Url,
    pub title: String,
    // icon: Image<Handle>,
}

pub enum PixelFormat {
    RGBA,
    BGRA,
}

#[allow(unused)]
pub trait BrowserEngine {
    fn new() -> Self;

    fn do_work(&self);
    fn need_render(&self) -> bool;
    fn render(&mut self);
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, width: u32, height: u32);
    fn pixel_buffer(&mut self) -> (PixelFormat, Vec<u8>);

    fn get_cursor(&self) -> Interaction;
    // fn get_icon(&self) -> Image<Handle>;
    fn get_title(&self) -> Option<String>;
    fn get_url(&self) -> Option<Url>;
    fn goto_url(&self, url: &Url);
    fn has_loaded(&self) -> bool;
    fn new_tab(&mut self, url: &Url);
    fn goto_tab(&mut self, idx: u32) -> Option<()>;
    fn get_tabs(&self) -> Vec<Tab>;
    fn current_tab(&self) -> (usize, Tab);
    fn close_tab(&mut self, idx: u32);

    fn refresh(&self);
    fn go_forward(&self);
    fn go_back(&self);
    fn focus(&self);
    fn unfocus(&self);

    fn scroll(&self, delta: mouse::ScrollDelta);
    fn handle_keyboard_event(&self, event: keyboard::Event);
    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event);
}
