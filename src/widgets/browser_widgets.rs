use iced::{keyboard, mouse, widget::column, Element, Point, Rectangle};
use url::Url;

use super::{nav_bar, tab_bar, BrowserView, NavBar, TabBar};
use crate::{
    engines::{BrowserEngine, PixelFormat},
    to_url, ImageInfo,
};

#[cfg(feature = "ultralight")]
use crate::engines::ultralight::Ultralight;

#[derive(Debug, Clone)]
pub enum Message {
    // Pass messages to children
    TabBar(tab_bar::Message),
    NavBar(nav_bar::Message),

    // Handle engine events & update widgets
    GoBackward,
    GoForward,
    Refresh,
    GoHome,
    GoUrl(String),
    ChangeTab(usize),
    CloseTab(usize),
    CreateTab,
    UrlChanged(String),
    UrlSubmitted(String),
    SendKeyboardEvent(keyboard::Event),
    SendMouseEvnt(Point, mouse::Event),
    UpdateBounds(Rectangle),
    DoWork,

    None,
}

pub struct BrowserWidget<Engine: BrowserEngine> {
    engine: Option<Engine>,
    home: Url,
    tab_bar: Option<TabBar>,
    nav_bar: Option<NavBar>,
    browser_view: bool,
    image: ImageInfo,
    view_bounds: Rectangle,
}

impl<Engine: BrowserEngine> Default for BrowserWidget<Engine> {
    fn default() -> Self {
        let home = Url::parse(Self::HOME).unwrap();
        Self {
            engine: None,
            home,
            tab_bar: None,
            nav_bar: None,
            browser_view: false,
            image: ImageInfo::default(),
            view_bounds: Rectangle::default(),
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
        self.tab_bar = Some(tab_bar());
        self
    }

    pub fn with_nav_bar(mut self) -> Self {
        self.nav_bar = Some(nav_bar(self.home.as_str()));
        self
    }

    pub fn with_browsesr_view(mut self) -> Self {
        self.browser_view = true;
        self
    }

    pub fn build(mut self) -> Self {
        assert_eq!(self.engine.is_none(), false);

        // Create new tab for widgets to init with
        if let Some(engine) = self.engine.as_mut() {
            engine.new_tab(&self.home)
        }

        Self {
            engine: self.engine,
            home: self.home,
            tab_bar: self.tab_bar,
            nav_bar: self.nav_bar,
            browser_view: self.browser_view,
            image: self.image,
            view_bounds: Rectangle::default(),
        }
    }

    fn update_nav_bar_maybe(&mut self, message: nav_bar::Message) {
        if let Some(nav_bar) = self.nav_bar.as_mut() {
            nav_bar.update(message);
        }
    }

    fn update_tab_bar_maybe(&mut self, message: tab_bar::Message) {
        if let Some(tab_bar) = self.tab_bar.as_mut() {
            tab_bar.update(message);
        }
    }

    pub fn update(&mut self, message: Message) {
        if let Some(engine) = self.engine.as_mut() {
            if engine.need_render() {
                let (format, image_data) = engine.pixel_buffer();
                self.image = match format {
                    PixelFormat::RGBA => ImageInfo::new(
                        image_data,
                        self.view_bounds.x as u32,
                        self.view_bounds.y as u32,
                    ),
                    PixelFormat::BGRA => ImageInfo::new_from_bgr(
                        image_data,
                        self.view_bounds.x as u32,
                        self.view_bounds.y as u32,
                    ),
                };
            }

            match message {
                Message::UpdateBounds(bounds) => {
                    let (current_size, allowed_size) = (engine.size(), bounds.size());
                    if current_size.0 != allowed_size.width as u32
                        || current_size.1 != allowed_size.height as u32
                    {
                        engine.resize(allowed_size.width as u32, allowed_size.height as u32);
                        self.view_bounds = bounds;
                    }
                }
                Message::DoWork => engine.do_work(),
                Message::SendKeyboardEvent(event) => {
                    engine.handle_keyboard_event(event);
                }
                Message::SendMouseEvnt(point, event) => {
                    engine.handle_mouse_event(point, event);
                }
                Message::ChangeTab(index) => engine.goto_tab(index as u32).unwrap(),
                Message::CloseTab(index) => {
                    engine.close_tab(index as u32);
                    self.update_tab_bar_maybe(tab_bar::Message::TabClosed(index))
                }
                Message::CreateTab => {
                    engine.new_tab(&Url::parse(self.home.as_str()).unwrap());
                    self.update_nav_bar_maybe(nav_bar::Message::UrlChanged(self.home.to_string()))
                }
                Message::GoBackward => {
                    engine.go_back();
                    let url = engine.get_url().unwrap();
                    self.update_nav_bar_maybe(nav_bar::Message::UrlChanged(url.to_string()))
                }
                Message::GoForward => {
                    engine.go_forward();
                    let url = engine.get_url().unwrap();
                    self.update_nav_bar_maybe(nav_bar::Message::UrlChanged(url.to_string()))
                }
                Message::Refresh => engine.refresh(),
                Message::GoHome => {
                    engine.goto_url(&self.home);
                    self.update_nav_bar_maybe(nav_bar::Message::UrlChanged(self.home.to_string()))
                }
                Message::GoUrl(url) => engine.goto_url(&to_url(&url).unwrap()),
                Message::UrlChanged(url) => {
                    self.update_nav_bar_maybe(nav_bar::Message::UrlChanged(url.to_string()))
                }
                Message::UrlSubmitted(url) => {
                    if let Ok(url) = Url::parse(url.as_str()) {
                        engine.goto_url(&url)
                    }
                }
                Message::None => (),

                // Relay messages to children
                Message::TabBar(msg) => {
                    if let Some(tab_bar) = self.tab_bar.as_mut() {
                        tab_bar.update(msg);
                    }
                }
                Message::NavBar(msg) => {
                    if let Some(nav_bar) = self.nav_bar.as_mut() {
                        nav_bar.update(msg);
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut column = column![];

        if let Some(nav_bar) = self.nav_bar.as_ref() {
            column = column.push(nav_bar.view())
        }
        if let Some(tab_bar) = self.tab_bar.as_ref() {
            column = column.push(tab_bar.view())
        }
        if self.browser_view == true {
            column = column.push(BrowserView::new(
                &self.image,
                Box::new(Message::UpdateBounds),
                Box::new(Message::SendKeyboardEvent),
                Box::new(Message::SendMouseEvnt),
            ))
        }

        column.into()
    }

    // fn create_first_tab() -> Command<Message> {
    //        iced::Task::future(async {
    //            // Fetch a joke from the internet
    //            let client = reqwest::Client::new();
    //            let response: serde_json::Value = client
    //                .get("https://icanhazdadjoke.com")
    //                .header("Accept", "application/json")
    //                .send()
    //                .await
    //                .unwrap()
    //                .json()
    //                .await
    //                .unwrap();

    //            // Parse the response
    //            let joke = response["joke"].as_str().unwrap();

    //            // Return the joke as a message
    //            Message::Initialize(joke.to_owned())
    //        })
}
