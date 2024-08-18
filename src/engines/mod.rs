use iced::keyboard;
use iced::mouse::{self, Interaction};
use iced::Size;
use std::sync::{Arc, RwLock};
// use iced::widget::image::{Handle, Image};
use iced::Point;
use rand::Rng;
use url::Url;

#[cfg(feature = "webkit")]
pub mod ultralight;

pub enum PixelFormat {
    Rgba,
    Bgra,
}

#[allow(unused)]
pub trait BrowserEngine {
    type TabInfo: TabInfo;

    fn new() -> Self;

    fn do_work(&self);
    fn need_render(&self) -> bool;
    fn render(&mut self);
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, size: Size);
    fn pixel_buffer(&mut self) -> (PixelFormat, Vec<u8>);

    fn get_cursor(&self) -> Interaction;
    // fn get_icon(&self) -> Image<Handle>;
    fn get_title(&self) -> Option<String>;
    fn get_url(&self) -> Option<Url>;
    fn goto_url(&self, url: &Url);
    fn has_loaded(&self) -> bool;
    fn new_tab(&mut self, url: &Url) -> u32;
    fn get_tabs(&self) -> &Tabs<Self::TabInfo>;
    fn get_tabs_mut(&mut self) -> &mut Tabs<Self::TabInfo>;

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
pub trait TabInfo {}

/// Stores Tab info like url & title
// Some browser engines take a closure to the url and title
// to automatically update it when it changes
pub struct Tab<TabInfo> {
    id: u32,
    url: Arc<RwLock<String>>,
    title: Arc<RwLock<String>>,
    tab_info: TabInfo,
}

impl<TabInfo> Tab<TabInfo> {
    pub fn new(url: Arc<RwLock<String>>, title: Arc<RwLock<String>>, tab_info: TabInfo) -> Self {
        let id = rand::thread_rng().gen();
        Self {
            id,
            url,
            title,
            tab_info,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn url(&self) -> String {
        self.url.read().unwrap().to_string()
    }

    pub fn title(&self) -> String {
        self.title.read().unwrap().to_string()
    }
}

pub struct Tabs<TabInfo> {
    tabs: Vec<Tab<TabInfo>>,
    current: u32,
}

impl<TabInfo> Default for Tabs<TabInfo> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TabInfo> Tabs<TabInfo> {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            current: 0,
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
        self.current
    }

    pub fn set_current_id(&mut self, id: u32) {
        self.current = id
    }

    pub fn tabs(&self) -> &Vec<Tab<TabInfo>> {
        &self.tabs
    }

    pub fn insert(&mut self, tab: Tab<TabInfo>) -> u32 {
        let id = tab.id;
        self.tabs.push(tab);
        id
    }

    /// Returns the newly active tab
    pub fn remove(&mut self, id: u32) -> u32 {
        // TODO: have list of prevous tabs instead
        if self.current == id {
            for tab in self.tabs.iter().rev() {
                if tab.id != id {
                    self.current = tab.id;
                    break;
                }
            }
        }

        self.tabs.retain(|tab| tab.id != id);
        self.current
    }

    pub fn get_current(&self) -> &Tab<TabInfo> {
        self.get(self.current)
    }

    pub fn get_current_mut(&mut self) -> &mut Tab<TabInfo> {
        self.get_mut(self.current)
    }

    pub fn get(&self, id: u32) -> &Tab<TabInfo> {
        for tab in self.tabs.iter() {
            if tab.id == id {
                return tab;
            }
        }
        panic!("Unable to find Tab with id: {}", id);
    }

    pub fn get_mut(&mut self, id: u32) -> &mut Tab<TabInfo> {
        for tab in self.tabs.iter_mut() {
            if tab.id == id {
                return tab;
            }
        }
        panic!("Unable to find Tab with id: {}", id);
    }
}
