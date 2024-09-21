use iced::keyboard::{self, key};
use iced::widget::{self, column};
use iced::{event::Event, mouse, Element, Point, Size, Task};
use iced_on_focus_widget::hoverable;
use url::Url;

mod browser_view;
pub use browser_view::browser_view;

mod nav_bar;
pub use nav_bar::nav_bar;

mod tab_bar;
pub use tab_bar::tab_bar;

mod command_window;
pub use command_window::command_window;

use crate::{check_shortcut, engines::BrowserEngine, to_url, ImageInfo, Shortcuts};

#[derive(Debug, Clone)]
pub enum Message {
    GoBackward,
    GoForward,
    Refresh,
    GoHome,
    GoToUrl(String),
    ChangeTab(TabSelectionType),
    CloseTab(TabSelectionType),
    CloseCurrentTab,
    CreateTab,
    UrlChanged(String),
    UpdateUrl,
    QueryChanged(String),
    SendKeyboardEvent(keyboard::Event),
    SendMouseEvent(Point, mouse::Event),
    UpdateViewSize(Size<u32>),
    Event(Event),
    ShowOverlay,
    HideOverlay,
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
    url: String,   // State of url bar
    query: String, // State of Command window
    with_tab_bar: bool,
    with_nav_bar: bool,
    show_overlay: bool,
    shortcuts: Shortcuts,
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
            query: String::new(),
            with_tab_bar: false,
            with_nav_bar: false,
            show_overlay: false,
            shortcuts: Shortcuts::default(),
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
        self.with_tab_bar = true;
        self
    }

    pub fn with_nav_bar(mut self) -> Self {
        self.with_nav_bar = true;
        self
    }

    pub fn with_custom_shortcuts(mut self, shortcuts: Shortcuts) -> Self {
        self.shortcuts = shortcuts;
        self
    }

    pub fn build(self) -> Self {
        assert!(self.engine.is_some());

        let mut build = Self { ..self };
        let _ = build.update(Message::CreateTab); // disregaurd task::none() for update
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

    fn update_engine(&mut self) {
        self.engine().do_work();
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

    /// This is used to periodically update browserview
    pub fn force_update(&mut self) -> Task<Message> {
        self.engine().do_work();
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
            .set_view(view);

        Task::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = match message {
            Message::UpdateViewSize(size) => {
                self.view_size = size;
                self.engine_mut().resize(size);
                Task::none()
            }
            Message::SendKeyboardEvent(event) => {
                self.engine().handle_keyboard_event(event);
                Task::none()
            }
            Message::SendMouseEvent(point, event) => {
                self.engine_mut().handle_mouse_event(point, event);
                Task::none()
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
                Task::none()
            }
            Message::CloseCurrentTab => Task::done(Message::CloseTab(TabSelectionType::Id(
                self.engine().get_tabs().get_current_id(),
            ))),
            Message::CloseTab(index_type) => {
                // ensure there is always at least one tab
                if self.engine().get_tabs().tabs().len() == 1 {
                    let _ = self.update(Message::CreateTab); // ignore task
                }

                let id = match index_type {
                    TabSelectionType::Id(id) => id,
                    TabSelectionType::Index(index) => {
                        self.engine_mut().get_tabs().index_to_id(index)
                    }
                };
                self.engine_mut().get_tabs_mut().remove(id);
                self.url = self.engine().get_tabs().get_current().url();
                Task::none()
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
                Task::none()
            }
            Message::GoBackward => {
                self.engine().go_back();
                self.url = self.engine().get_tabs().get_current().url();
                Task::none()
            }
            Message::GoForward => {
                self.engine().go_forward();
                self.url = self.engine().get_tabs().get_current().url();
                Task::none()
            }
            Message::Refresh => {
                self.engine().refresh();
                Task::none()
            }
            Message::GoHome => {
                self.engine().goto_url(&self.home);
                Task::none()
            }
            Message::GoToUrl(url) => {
                self.engine().goto_url(&to_url(&url).unwrap());
                Task::none()
            }
            Message::UpdateUrl => {
                self.url = self.engine().get_tabs().get_current().url();
                Task::none()
            }
            Message::UrlChanged(url) => {
                self.url = url;
                Task::none()
            }
            Message::QueryChanged(query) => {
                self.query = query;
                Task::none()
            }
            Message::ShowOverlay => {
                self.show_overlay = true;
                widget::focus_next()
            }
            Message::HideOverlay => {
                self.show_overlay = false;
                Task::none()
            }
            Message::Event(event) => {
                match event {
                    Event::Keyboard(key) => {
                        if let iced::keyboard::Event::KeyPressed {
                            key,
                            modified_key: _,
                            physical_key: _,
                            location: _,
                            modifiers,
                            text: _,
                        } = key
                        {
                            // Default behaviors
                            if key == keyboard::Key::Named(key::Named::Escape) && self.show_overlay
                            {
                                return Task::done(Message::HideOverlay);
                            }

                            // Shortcut (Customizable) behaviors
                            for shortcut in self.shortcuts.iter() {
                                if check_shortcut(shortcut, &key, &modifiers) {
                                    match shortcut.0 {
                                        crate::ShortcutType::GoBackward => {
                                            return Task::done(Message::GoBackward)
                                        }
                                        crate::ShortcutType::GoForward => {
                                            return Task::done(Message::GoForward)
                                        }
                                        crate::ShortcutType::Refresh => {
                                            return Task::done(Message::Refresh)
                                        }
                                        crate::ShortcutType::GoHome => {
                                            return Task::done(Message::GoHome)
                                        }
                                        crate::ShortcutType::CloseCurrentTab => {
                                            return Task::done(Message::CloseCurrentTab)
                                        }
                                        crate::ShortcutType::CreateTab => {
                                            return Task::done(Message::CreateTab)
                                        }
                                        crate::ShortcutType::ToggleOverlay => {
                                            if self.show_overlay {
                                                return Task::done(Message::HideOverlay);
                                            } else {
                                                return Task::done(Message::ShowOverlay);
                                            }
                                        }
                                        crate::ShortcutType::ShowOverlay => {
                                            return Task::done(Message::ShowOverlay)
                                        }
                                        crate::ShortcutType::HideOverlay => {
                                            return Task::done(Message::HideOverlay)
                                        }
                                    }
                                }
                            }
                        }
                        Task::none()
                    }
                    // Other unwatched events
                    _ => Task::none(),
                }
            }
        };

        self.update_engine();

        task
    }

    pub fn view(&self) -> Element<Message> {
        let mut column = column![];

        if self.with_tab_bar {
            column = column.push(tab_bar(self.engine().get_tabs()))
        }
        if self.with_nav_bar {
            column = column.push(hoverable(nav_bar(&self.url)).on_focus_change(Message::UpdateUrl))
        }

        let browser_view = browser_view(
            self.view_size,
            self.engine().get_tabs().get_current().get_view(),
            Box::new(Message::UpdateViewSize),
            Box::new(Message::SendKeyboardEvent),
            Box::new(Message::SendMouseEvent),
            !self.show_overlay,
        );
        if self.show_overlay {
            column = column.push(command_window(browser_view, &self.query))
        } else {
            column = column.push(browser_view);
        }

        column.into()
    }
}
