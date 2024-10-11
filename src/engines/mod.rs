use iced::keyboard;
use iced::mouse::{self, Interaction};
use iced::Size;
// use iced::widget::image::{Handle, Image};
use iced::Point;
use rand::Rng;
use url::Url;

use crate::ImageInfo;

#[cfg(feature = "ultralight")]
pub mod ultralight;

pub enum PixelFormat {
    Rgba,
    Bgra,
}

#[allow(unused)]
pub trait BrowserEngine {
    type Info: TabInfo;

    fn new() -> Self;

    fn do_work(&self);
    fn need_render(&self) -> bool;
    fn force_need_render(&self);
    fn render(&mut self);
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, size: Size<u32>);
    fn pixel_buffer(&mut self) -> (PixelFormat, Vec<u8>);

    fn get_cursor(&self) -> Interaction;
    // fn get_icon(&self) -> Image<Handle>;
    fn goto_url(&self, url: &Url);
    fn has_loaded(&self) -> bool;
    fn new_tab(&mut self, url: Url, size: Size<u32>) -> Tab<Self::Info>;
    fn get_tabs(&self) -> &Tabs<Self::Info>;
    fn get_tabs_mut(&mut self) -> &mut Tabs<Self::Info>;

    fn refresh(&self);
    fn go_forward(&self);
    fn go_back(&self);
    fn focus(&self);
    fn unfocus(&self);

    fn scroll(&self, delta: mouse::ScrollDelta);
    fn handle_keyboard_event(&self, event: keyboard::Event);
    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event);
}

/// Engine specific tab information
pub trait TabInfo {
    fn url(&self) -> String;
    fn title(&self) -> String;
}

/// Stores Tab info like url & title
// Some browser engines take a closure to the url and title
// to automatically update it when it changes
pub struct Tab<Info: TabInfo> {
    id: u32,
    view: ImageInfo,
    info: Info,
}

impl<Info: TabInfo> Tab<Info> {
    pub fn new(info: Info) -> Self {
        let id = rand::thread_rng().gen();
        Self {
            id,
            view: ImageInfo::default(),
            info,
        }
    }

    pub fn get_view(&self) -> &ImageInfo {
        &self.view
    }

    pub fn set_view(&mut self, view: ImageInfo) {
        self.view = view;
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn url(&self) -> String {
        self.info.url()
    }

    pub fn title(&self) -> String {
        self.info.title()
    }
}

pub struct Tabs<Info: TabInfo> {
    tabs: Vec<Tab<Info>>,
    history: Vec<u32>,
}

impl<Info: TabInfo> Default for Tabs<Info> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Info: TabInfo> Tabs<Info> {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn id_to_index(&self, id: u32) -> usize {
        for (idx, tab) in self.tabs.iter().enumerate() {
            if tab.id == id {
                return idx;
            }
        }
        panic!("Id: {} was not found", id);
    }

    pub fn index_to_id(&self, index: usize) -> u32 {
        self.tabs
            .get(index)
            .unwrap_or_else(|| panic!("Index {} was not found", index))
            .id
    }

    pub fn get_current_id(&self) -> u32 {
        self.history
            .last()
            .expect("No tab in history to get current from")
            .to_owned()
    }

    pub fn set_current_id(&mut self, id: u32) {
        self.history.push(id)
    }

    pub fn tabs(&self) -> &Vec<Tab<Info>> {
        &self.tabs
    }

    pub fn insert(&mut self, tab: Tab<Info>) -> u32 {
        let id = tab.id;
        self.tabs.push(tab);
        id
    }

    /// Returns the newly active tab
    pub fn remove(&mut self, id: u32) -> u32 {
        self.history.retain(|tab_id| *tab_id != id);

        self.tabs.retain(|tab| tab.id != id);
        self.get_current_id()
    }

    pub fn get_current(&self) -> &Tab<Info> {
        self.get(self.get_current_id())
    }

    pub fn get_current_mut(&mut self) -> &mut Tab<Info> {
        self.get_mut(self.get_current_id())
    }

    pub fn get(&self, id: u32) -> &Tab<Info> {
        for tab in self.tabs.iter() {
            if tab.id == id {
                return tab;
            }
        }
        panic!("Unable to find Tab with id: {}", id);
    }

    pub fn get_mut(&mut self, id: u32) -> &mut Tab<Info> {
        for tab in self.tabs.iter_mut() {
            if tab.id == id {
                return tab;
            }
        }
        panic!("Unable to find Tab with id: {}", id);
    }
}
