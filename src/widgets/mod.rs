use crate::engines::{self, BrowserEngine};
#[cfg(feature = "webkit")]
use engines::ultralight::Ultralight;

use std::sync::{Arc, Mutex};

mod browser_view;
pub use browser_view::{browser_view, BrowserView};
mod nav_bar;
pub use nav_bar::{nav_bar, NavBar};

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
#[derive(Clone)]
pub struct State {
    config: Config,
    #[cfg(feature = "webkit")]
    webengine: Arc<Mutex<Ultralight>>,
}

impl State {
    // TODO: this should be generic
    pub fn new() -> Self {
        let config = Config::default();
        let mut webengine = Ultralight::new(800, 800);
        webengine.new_tab(&config.start_page);

        State {
            config,
            #[cfg(feature = "webkit")]
            webengine: Arc::new(Mutex::new(webengine)),
        }
    }

    pub fn do_work(&self) {
        self.webengine.lock().unwrap().do_work()
    }
}
