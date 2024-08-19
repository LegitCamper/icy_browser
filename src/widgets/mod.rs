use iced::{keyboard, mouse, widget::column, Element, Point, Size};
use iced_on_focus_widget::hoverable;
use url::Url;

mod browser_view;
pub use browser_view::browser_view;

mod nav_bar;
pub use nav_bar::nav_bar;

mod tab_bar;
pub use tab_bar::tab_bar;

use crate::{engines::BrowserEngine, to_url, ImageInfo};

#[derive(Debug, Clone)]
pub enum Message {
    GoBackward,
    GoForward,
    Refresh,
    GoHome,
    GoUrl(String),
    ChangeTab(TabSelectionType),
    CloseTab(TabSelectionType),
    CreateTab,
    UrlChanged(String),
    UpdateUrl,
    SendKeyboardEvent(keyboard::Event),
    SendMouseEvent(Point, mouse::Event),
    UpdateViewSize(Size<u32>),
}

/// Allows different widgets to interact in their native way
#[derive(Debug, Clone)]
pub enum TabSelectionType {
    Id(u32),
    Index(usize),
}

pub struct BrowserWidget<Engine: BrowserEngine> {
    engine: Option<Engine>,
    home: Url,
    url: String,
    tab_bar: bool,
    nav_bar: bool,
    browser_view: bool,
    view_size: Size<u32>,
}

impl<Engine> Default for BrowserWidget<Engine>
where
    Engine: BrowserEngine,
{
    fn default() -> Self {
        let home = Url::parse(Self::HOME).unwrap();
        Self {
            engine: None,
            home,
            url: String::new(),
            tab_bar: false,
            nav_bar: false,
            browser_view: false,
            view_size: Size::new(800, 800),
        }
    }
}

#[cfg(feature = "ultralight")]
use crate::engines::ultralight::Ultralight;

#[cfg(feature = "ultralight")]
impl BrowserWidget<Ultralight> {
    pub fn new_with_ultralight() -> BrowserWidget<Ultralight> {
        BrowserWidget {
            engine: Some(Ultralight::new()),
            ..BrowserWidget::default()
        }
    }
}

impl<Engine> BrowserWidget<Engine>
where
    Engine: BrowserEngine,
{
    const HOME: &'static str = "https://google.com";

    pub fn new() -> Self {
        Self {
            engine: Some(Engine::new()),
            ..Default::default()
        }
    }

    pub fn with_homepage(mut self, homepage: &str) -> Self {
        self.home = Url::parse(homepage).expect("Failed to parse homepage as a url!");
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
        assert!(self.engine.is_some());

        let mut build = Self {
            engine: self.engine,
            home: self.home,
            tab_bar: self.tab_bar,
            nav_bar: self.nav_bar,
            url: self.url,
            browser_view: self.browser_view,
            view_size: self.view_size,
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
        self.engine().do_work();

        match message {
            Message::UpdateViewSize(size) => {
                self.view_size = size;
                self.engine_mut().resize(size);
            }
            Message::SendKeyboardEvent(event) => {
                self.engine().handle_keyboard_event(event);
            }
            Message::SendMouseEvent(point, event) => {
                self.engine_mut().handle_mouse_event(point, event);
            }
            Message::ChangeTab(index_type) => {
                let id = match index_type {
                    TabSelectionType::Id(id) => id,
                    TabSelectionType::Index(index) => {
                        self.engine_mut().get_tabs().index_to_id(index)
                    }
                };
                self.engine_mut().get_tabs_mut().set_current_id(id);
                self.url = self.engine().get_tabs().get_current().url();
            }
            Message::CloseTab(index_type) => {
                // ensure there is still a tab
                if self.engine().get_tabs().tabs().len() == 1 {
                    self.update(Message::CreateTab)
                }

                let id = match index_type {
                    TabSelectionType::Id(id) => id,
                    TabSelectionType::Index(index) => {
                        self.engine_mut().get_tabs().index_to_id(index)
                    }
                };
                self.engine_mut().get_tabs_mut().remove(id);
                self.url = self.engine().get_tabs().get_current().url();
            }
            Message::CreateTab => {
                self.url = self.home.to_string();
                let home = self.home.clone();
                let bounds = self.view_size;
                let tab = self.engine_mut().new_tab(
                    home.clone(),
                    Size::new(bounds.width + 10, bounds.height - 10),
                );
                let id = self.engine_mut().get_tabs_mut().insert(tab);
                self.engine_mut().get_tabs_mut().set_current_id(id);
                self.engine_mut().force_need_render();
                self.engine_mut().resize(bounds);
                self.engine().goto_url(&home);
            }
            Message::GoBackward => {
                self.engine().go_back();
                self.url = self.engine().get_tabs().get_current().url();
            }
            Message::GoForward => {
                self.engine().go_forward();
                self.url = self.engine().get_tabs().get_current().url();
            }
            Message::Refresh => self.engine().refresh(),
            Message::GoHome => {
                self.engine().goto_url(&self.home);
            }
            Message::GoUrl(url) => {
                self.engine().goto_url(&to_url(&url).unwrap());
            }
            Message::UpdateUrl => {
                self.url = self.engine().get_tabs().get_current().url();
            }
            Message::UrlChanged(url) => self.url = url,
        }

        if self.engine().has_loaded() {
            if self.engine().need_render() {
                let (format, image_data) = self.engine_mut().pixel_buffer();
                let view = ImageInfo::new(
                    image_data,
                    format,
                    self.view_size.width,
                    self.view_size.height,
                );
                self.engine_mut()
                    .get_tabs_mut()
                    .get_current_mut()
                    .set_view(view)
            }
        } else {
            let view = ImageInfo {
                width: self.view_size.width,
                height: self.view_size.height,
                ..Default::default()
            };
            self.engine_mut()
                .get_tabs_mut()
                .get_current_mut()
                .set_view(view)
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut column = column![];

        if self.tab_bar {
            column = column.push(tab_bar(self.engine().get_tabs()))
        }
        if self.nav_bar {
            column = column.push(hoverable(nav_bar(&self.url)).on_unfocus(Message::UpdateUrl))
        }
        if self.browser_view {
            column = column.push(browser_view(
                self.view_size,
                self.engine().get_tabs().get_current().get_view(),
                Box::new(Message::UpdateViewSize),
                Box::new(Message::SendKeyboardEvent),
                Box::new(Message::SendMouseEvent),
            ))
        }

        column.into()
    }
}
