use command_window::CommandWindowState;
use iced::keyboard::{self, key};
use iced::widget::{self, column};
use iced::{event::Event, mouse, Element, Point, Size, Task};
use iced_on_focus_widget::hoverable;
use nav_bar::NavBarState;
use std::string::ToString;
use strum_macros::{Display, EnumIter};
use url::Url;

mod browser_view;
pub use browser_view::browser_view;

mod nav_bar;
pub use nav_bar::nav_bar;

mod tab_bar;
pub use tab_bar::tab_bar;

mod command_window;
pub use command_window::command_window;

use crate::{engines::BrowserEngine, shortcut::check_shortcut, to_url, ImageInfo, Shortcuts};

// Options exist only to have defaults for EnumIter
#[derive(Debug, Clone, PartialEq, Display, EnumIter)]
pub enum Message {
    // Commands
    #[strum(to_string = "Go Backward")]
    GoBackward,
    #[strum(to_string = "Go Forward")]
    GoForward,
    Refresh,
    #[strum(to_string = "Go Home")]
    GoHome,
    #[strum(to_string = "Go To Url")]
    GoToUrl(String),
    #[strum(to_string = "Change Tab")]
    ChangeTab(TabSelectionType),
    #[strum(to_string = "Close Tab")]
    CloseTab(TabSelectionType),
    #[strum(to_string = "Close Tab")]
    CloseCurrentTab,
    #[strum(to_string = "New Tab")]
    CreateTab,
    #[strum(to_string = "Toggle Command Palatte")]
    ToggleOverlay,
    #[strum(to_string = "Show Command Palatte")]
    ShowOverlay,
    #[strum(to_string = "Hide Command Palatte")]
    HideOverlay,

    // Internal only - for widgets
    UrlChanged(String),
    UpdateUrl,
    QueryChanged(String),
    CommandSelectionChanged(usize, String),
    SendKeyboardEvent(Option<keyboard::Event>),
    SendMouseEvent(Point, Option<mouse::Event>),
    UpdateViewSize(Size<u32>),
    Event(Option<Event>),
}

/// Allows different widgets to interact in their native way
#[derive(Debug, Clone, PartialEq)]
pub enum TabSelectionType {
    Id(u32),
    Index(usize),
}
impl Default for TabSelectionType {
    fn default() -> Self {
        TabSelectionType::Index(0)
    }
}

pub struct BrowserWidget<Engine: BrowserEngine> {
    engine: Option<Engine>,
    home: Url,
    nav_bar_state: NavBarState,
    command_window_state: CommandWindowState,
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
            nav_bar_state: NavBarState::new(),
            command_window_state: CommandWindowState::new(),
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
                self.engine()
                    .handle_keyboard_event(event.expect("Value cannot be none"));
                Task::none()
            }
            Message::SendMouseEvent(point, event) => {
                self.engine_mut()
                    .handle_mouse_event(point, event.expect("Value cannot be none"));
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
                self.nav_bar_state.0 = self.engine().get_tabs().get_current().url();
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
                self.nav_bar_state.0 = self.engine().get_tabs().get_current().url();
                Task::none()
            }
            Message::CreateTab => {
                self.nav_bar_state.0 = self.home.to_string();
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
                self.nav_bar_state.0 = self.engine().get_tabs().get_current().url();
                Task::none()
            }
            Message::GoForward => {
                self.engine().go_forward();
                self.nav_bar_state.0 = self.engine().get_tabs().get_current().url();
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
                self.nav_bar_state.0 = self.engine().get_tabs().get_current().url();
                Task::none()
            }
            Message::UrlChanged(url) => {
                self.nav_bar_state.0 = url;
                Task::none()
            }
            Message::QueryChanged(query) => {
                self.command_window_state.query = query;
                Task::none()
            }
            Message::CommandSelectionChanged(index, name) => {
                self.command_window_state.selected_index = index;
                self.command_window_state.selected_action = name;
                Task::none()
            }
            Message::ToggleOverlay => {
                if self.show_overlay {
                    Task::done(Message::HideOverlay)
                } else {
                    Task::done(Message::ShowOverlay)
                }
            }
            Message::ShowOverlay => {
                self.show_overlay = true;
                widget::focus_next()
            }
            Message::HideOverlay => {
                self.show_overlay = false;
                widget::focus_next()
            }
            Message::Event(event) => {
                match event {
                    Some(Event::Keyboard(key)) => {
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
                                    return Task::done(shortcut.0.clone());
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
            column = column
                .push(hoverable(nav_bar(&self.nav_bar_state)).on_focus_change(Message::UpdateUrl))
        }

        let browser_view = browser_view(
            self.view_size,
            self.engine().get_tabs().get_current().get_view(),
            !self.show_overlay,
        );
        if self.show_overlay {
            column = column.push(command_window(browser_view, &self.command_window_state))
        } else {
            column = column.push(browser_view);
        }

        column.into()
    }
}
