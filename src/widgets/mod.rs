use std::{cell::RefCell, rc::Rc};
use url::Url;

use crate::engines::{self, BrowserEngine};

pub mod browser_view;
#[allow(unused)]
pub use browser_view::{browser_view, BrowserView};

pub mod nav_bar;
#[allow(unused)]
pub use nav_bar::{nav_bar, NavBar};

pub mod tab_bar;
#[allow(unused)]
pub use tab_bar::{tab_bar, TabBar};

// Configures the Browser Widget
#[derive(Debug, Clone)]
pub struct Config {
    pub start_page: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_page: String::from("https://google.com"),
        }
    }
}

// Holds the State of the Browser Widgets
#[derive(Debug)]
pub struct State<Engine: BrowserEngine> {
    config: Config,
    webengine: Rc<RefCell<Engine>>,
}

impl<Engine: BrowserEngine> Default for State<Engine> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Engine: BrowserEngine> Clone for State<Engine> {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            webengine: self.webengine.clone(),
        }
    }
}

impl<Engine: BrowserEngine> State<Engine> {
    pub fn new() -> Self {
        let config = Config::default();
        let mut webengine = Engine::new();
        webengine.new_tab(&Url::parse(&config.start_page).unwrap());

        State {
            config,
            webengine: Rc::new(RefCell::new(webengine)),
        }
    }

    pub fn do_work(&self) {
        self.webengine.borrow().do_work()
    }
}

#[cfg(feature = "ultralight")]
impl State<engines::ultralight::Ultralight> {
    pub fn new_ultralight() -> State<engines::ultralight::Ultralight> {
        State::new()
    }
}
