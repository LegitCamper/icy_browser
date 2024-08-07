use iced::keyboard;
use iced::mouse::{self, Interaction};
// use iced::widget::image::{Handle, Image};
use iced::{event::Status, Point};

#[cfg(feature = "webkit")]
pub mod ultralight;

pub fn create_engine<B: BrowserEngine>() -> impl BrowserEngine {
    #[cfg(feature = "webkit")]
    <ultralight::Ultralight as BrowserEngine>::new()
}

pub struct Tab {
    pub url: String,
    pub title: String,
    // icon: Image<Handle>,
}

#[allow(unused)]
pub trait BrowserEngine {
    fn new() -> Self;

    fn do_work(&self);
    fn need_render(&self) -> bool;
    fn render(&mut self);
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, width: u32, height: u32);
    fn pixel_buffer(&mut self) -> Option<Vec<u8>>;

    fn get_cursor(&self) -> Interaction;
    // fn get_icon(&self) -> Image<Handle>;
    fn get_title(&self) -> Option<String>;
    fn get_url(&self) -> Option<String>;
    fn goto_url(&self, url: &str);
    fn has_loaded(&self) -> bool;
    fn new_tab(&mut self, url: &str);
    fn goto_tab(&mut self, idx: u32) -> Option<()>;
    fn get_tabs(&self) -> Vec<Tab>;
    fn current_tab(&self) -> usize;
    fn close_tab(&mut self, idx: u32);

    fn refresh(&self);
    fn go_forward(&self);
    fn go_back(&self);
    fn focus(&self);
    fn unfocus(&self);

    fn scroll(&self, delta: mouse::ScrollDelta) -> Status;
    fn handle_keyboard_event(&self, event: keyboard::Event) -> Status;
    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event) -> Status;
}
