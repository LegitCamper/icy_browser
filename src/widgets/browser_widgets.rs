use iced::{keyboard, mouse, widget::column, Element, Point};
use url::Url;

use super::{nav_bar, tab_bar, BrowserView};
use crate::{
    engines::{BrowserEngine, PixelFormat},
    to_url, ImageInfo, ViewBounds,
};

#[cfg(feature = "ultralight")]
use crate::engines::ultralight::Ultralight;

#[derive(Debug, Clone)]
pub enum Message {
    GoBackward,
    GoForward,
    Refresh,
    GoHome,
    GoUrl(String),
    ChangeTab(usize),
    CloseTab(usize),
    CreateTab,
    UrlChanged(String),
    SendKeyboardEvent(keyboard::Event),
    SendMouseEvent(Point, mouse::Event),
    UpdateBounds(ViewBounds),
    DoWork,
}

pub struct BrowserWidget<Engine: BrowserEngine> {
    engine: Option<Engine>,
    home: Url,
    tab_bar: bool,
    nav_bar: bool,
    url: String,
    browser_view: bool,
    image: ImageInfo,
    view_bounds: ViewBounds,
}

impl<Engine: BrowserEngine> Default for BrowserWidget<Engine> {
    fn default() -> Self {
        let home = Url::parse(Self::HOME).unwrap();
        Self {
            engine: None,
            home,
            tab_bar: false,
            nav_bar: false,
            url: String::new(),
            browser_view: false,
            image: ImageInfo::default(),
            view_bounds: ViewBounds::default(),
        }
    }
}

#[cfg(feature = "ultralight")]
impl BrowserWidget<Ultralight> {
    pub fn new_with_ultralight() -> BrowserWidget<Ultralight> {
        BrowserWidget {
            engine: Some(Ultralight::new()),
            ..BrowserWidget::default()
        }
    }
}

impl<Engine: BrowserEngine> BrowserWidget<Engine> {
    const HOME: &'static str = "https://duckduckgo.com";

    pub fn new() -> Self {
        Self {
            engine: Some(Engine::new()),
            ..Default::default()
        }
    }

    pub fn with_homepage(mut self, homepage: &str) -> Self {
        self.home = Url::parse(homepage).unwrap();
        self
    }

    pub fn with_tab_bar(mut self) -> Self {
        self.tab_bar = true;
        self
    }

    pub fn with_nav_bar(mut self) -> Self {
        self.nav_bar = true;
        self
    }

    pub fn with_browsesr_view(mut self) -> Self {
        self.browser_view = true;
        self
    }

    pub fn build(self) -> Self {
        assert_eq!(self.engine.is_none(), false);

        let mut build = Self {
            engine: self.engine,
            home: self.home,
            tab_bar: self.tab_bar,
            nav_bar: self.nav_bar,
            url: self.url,
            browser_view: self.browser_view,
            image: self.image,
            view_bounds: self.view_bounds,
        };
        build.update(Message::CreateTab);
        build
    }

    fn engine(&self) -> &Engine {
        self.engine
            .as_ref()
            .expect("Browser was created without a backend engine!")
    }

    fn engine_mut(&mut self) -> &mut Engine {
        self.engine
            .as_mut()
            .expect("Browser was created without a backend engine!")
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::DoWork => self.engine().do_work(),
            Message::UpdateBounds(bounds) => {
                self.view_bounds = bounds;
                self.engine_mut().resize(bounds);
            }
            Message::SendKeyboardEvent(event) => {
                self.engine().handle_keyboard_event(event);
            }
            Message::SendMouseEvent(point, event) => {
                self.engine_mut().handle_mouse_event(point, event);
            }
            Message::ChangeTab(index) => self.engine_mut().goto_tab(index as u32).unwrap(),
            Message::CloseTab(index) => {
                self.engine_mut().close_tab(index as u32);
            }
            Message::CreateTab => {
                self.url = self.home.to_string();
                let home = self.home.clone();
                self.engine_mut().new_tab(&home);
            }
            Message::GoBackward => {
                self.engine().go_back();
                self.url = self.engine().get_url().unwrap().to_string();
            }
            Message::GoForward => {
                self.engine().go_forward();
                self.url = self.engine().get_url().unwrap().to_string();
            }
            Message::Refresh => self.engine().refresh(),
            Message::GoHome => {
                self.engine().goto_url(&self.home);
            }
            Message::GoUrl(url) => {
                self.engine().goto_url(&to_url(&url).unwrap());
            }
            Message::UrlChanged(url) => self.url = url,
        }

        if self.engine().need_render() {
            let (format, image_data) = self.engine_mut().pixel_buffer();
            self.image = match format {
                PixelFormat::RGBA => {
                    ImageInfo::new(image_data, self.view_bounds.width, self.view_bounds.height)
                }
                PixelFormat::BGRA => ImageInfo::new_from_bgr(
                    image_data,
                    self.view_bounds.width,
                    self.view_bounds.height,
                ),
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut column = column![];

        if self.tab_bar {
            let tabs = self.engine().get_tabs();
            let (active_tab, _) = self.engine().current_tab();
            column = column.push(tab_bar(tabs, active_tab))
        }
        if self.nav_bar {
            column = column.push(nav_bar(&self.url))
        }
        if self.browser_view {
            column = column.push(BrowserView::new(
                &self.image,
                Box::new(Message::UpdateBounds),
                Box::new(Message::SendKeyboardEvent),
                Box::new(Message::SendMouseEvent),
            ))
        }

        column.into()
    }
}
