use command_window::CommandWindowState;
use iced::keyboard::{self, key};
use iced::widget::{self, column};
use iced::{mouse, Element, Event, Point, Size, Subscription, Task};
use iced_on_focus_widget::hoverable;
use nav_bar::NavBarState;
use std::string::ToString;
use std::time::Duration;
use strum_macros::{Display, EnumIter};
use url::Url;

mod browser_view;
pub use browser_view::browser_view;

pub mod nav_bar;
pub use nav_bar::nav_bar;

pub mod tab_bar;
pub use tab_bar::tab_bar;

pub mod bookmark_bar;
pub use bookmark_bar::bookmark_bar;

pub mod command_window;
pub use command_window::{command_window, results_list, ResultType};

use crate::{
    engines::BrowserEngine, shortcut::check_shortcut, to_url, Bookmark, Bookmarks, ImageInfo,
    Shortcuts,
};

/// Allows users to implement their own custom view view with custom widgets and configurations
pub trait CustomWidget<Message> {
    fn update(&mut self, message: Message);
    fn view(
        &self,
        nav_bar_state: NavBarState,
        command_window_state: CommandWindowState,
        bookmarks: Option<Vec<Bookmark>>,
        shortcuts: Shortcuts,
    );
    fn subscription(&self) -> Subscription<Message>;
}

// Options exist only to have defaults for EnumIter
#[derive(Debug, Clone, PartialEq, Display, EnumIter)]
pub enum Message {
    // Commands visible to user with shortcuts and command palatte
    #[strum(to_string = "Go Backward (Back)")]
    GoBackward,
    #[strum(to_string = "Go Forward (Forward)")]
    GoForward,
    Refresh,
    #[strum(to_string = "Go Home (Home)")]
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
    Update,
    UrlChanged(String),
    UpdateUrl,
    CommandPalatteQueryChanged(String),
    CommandPalatteSelected(String),
    SendKeyboardEvent(Option<keyboard::Event>),
    SendMouseEvent(Point, Option<mouse::Event>),
    UpdateViewSize(Size<u32>),
    IcedEvent(Option<iced::Event>),
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

pub struct IcyBrowser<Engine: BrowserEngine> {
    engine: Engine,
    home: Url,
    nav_bar_state: Option<NavBarState>,
    command_window_state: CommandWindowState,
    with_tab_bar: bool,
    with_nav_bar: bool,
    bookmarks: Option<Bookmarks>,
    show_overlay: bool,
    shortcuts: Shortcuts,
    view_size: Size<u32>,
}

impl<Engine: BrowserEngine> Default for IcyBrowser<Engine> {
    fn default() -> Self {
        let home = Url::parse(Self::HOME).unwrap();
        Self {
            engine: Engine::new(),
            home,
            nav_bar_state: None,
            command_window_state: CommandWindowState::new(None),
            with_tab_bar: false,
            with_nav_bar: false,
            bookmarks: None,
            show_overlay: false,
            shortcuts: Shortcuts::default(),
            view_size: Size::new(800, 800),
        }
    }
}

impl<Engine: BrowserEngine> IcyBrowser<Engine> {
    const HOME: &'static str = "https://google.com";

    pub fn new() -> Self {
        Self::default()
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
        self.nav_bar_state = Some(NavBarState::new());
        self
    }

    pub fn with_bookmark_bar(mut self, bookmarks: &[Bookmark]) -> Self {
        self.bookmarks = Some(bookmarks.to_vec());
        self.command_window_state = CommandWindowState::new(self.bookmarks.clone());
        self
    }

    pub fn with_custom_shortcuts(mut self, shortcuts: Shortcuts) -> Self {
        self.shortcuts = shortcuts;
        self
    }

    pub fn build(self) -> Self {
        let mut build = Self { ..self };
        let _ = build.update(Message::CreateTab); // disregaurd task::none() for update
        build
    }

    /// Allows creation of custom widgets that need interal info
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Allows creation of custom widgets that need interal info
    pub fn mut_engine(&mut self) -> &mut Engine {
        &mut self.engine
    }

    fn update_engine(&mut self) {
        self.engine.do_work();
        if self.engine.has_loaded() {
            if self.engine.need_render() {
                let (format, image_data) = self.engine.pixel_buffer();
                let view = ImageInfo::new(
                    image_data,
                    format,
                    self.view_size.width,
                    self.view_size.height,
                );
                self.engine.get_tabs_mut().get_current_mut().set_view(view)
            }
        } else {
            let view = ImageInfo {
                width: self.view_size.width,
                height: self.view_size.height,
                ..Default::default()
            };
            self.engine.get_tabs_mut().get_current_mut().set_view(view)
        }
    }

    /// This is used to periodically update browserview
    pub fn force_update(&mut self) -> Task<Message> {
        self.engine.do_work();
        let (format, image_data) = self.engine.pixel_buffer();
        let view = ImageInfo::new(
            image_data,
            format,
            self.view_size.width,
            self.view_size.height,
        );
        self.engine.get_tabs_mut().get_current_mut().set_view(view);

        Task::none()
    }

    pub fn update(&mut self, event: Message) -> Task<Message> {
        let task = match event {
            Message::Update => self.force_update(),
            Message::UpdateViewSize(size) => {
                self.view_size = size;
                self.engine.resize(size);
                Task::none()
            }
            Message::SendKeyboardEvent(event) => {
                self.engine
                    .handle_keyboard_event(event.expect("Value cannot be none"));
                Task::none()
            }
            Message::SendMouseEvent(point, event) => {
                self.engine
                    .handle_mouse_event(point, event.expect("Value cannot be none"));
                Task::none()
            }
            Message::ChangeTab(index_type) => {
                let id = match index_type {
                    TabSelectionType::Id(id) => id,
                    TabSelectionType::Index(index) => self.engine.get_tabs().index_to_id(index),
                };
                self.engine.get_tabs_mut().set_current_id(id);
                if let Some(state) = self.nav_bar_state.as_mut() {
                    state.0 = self.engine.get_tabs().get_current().url();
                }
                Task::none()
            }
            Message::CloseCurrentTab => Task::done(Message::CloseTab(TabSelectionType::Id(
                self.engine.get_tabs().get_current_id(),
            ))),
            Message::CloseTab(index_type) => {
                // ensure there is always at least one tab
                if self.engine.get_tabs().tabs().len() == 1 {
                    let _ = self.update(Message::CreateTab); // ignore task
                }

                let id = match index_type {
                    TabSelectionType::Id(id) => id,
                    TabSelectionType::Index(index) => self.engine.get_tabs().index_to_id(index),
                };
                self.engine.get_tabs_mut().remove(id);
                if let Some(state) = self.nav_bar_state.as_mut() {
                    state.0 = self.engine.get_tabs().get_current().url();
                }
                Task::none()
            }
            Message::CreateTab => {
                if let Some(state) = self.nav_bar_state.as_mut() {
                    state.0 = self.home.to_string();
                }
                let home = self.home.clone();
                let bounds = self.view_size;
                let tab = self.engine.new_tab(
                    home.clone(),
                    Size::new(bounds.width + 10, bounds.height - 10),
                );
                let id = self.engine.get_tabs_mut().insert(tab);
                self.engine.get_tabs_mut().set_current_id(id);
                self.engine.force_need_render();
                self.engine.resize(bounds);
                self.engine.goto_url(&home);
                Task::none()
            }
            Message::GoBackward => {
                self.engine.go_back();
                if let Some(state) = self.nav_bar_state.as_mut() {
                    state.0 = self.engine.get_tabs().get_current().url();
                }
                Task::none()
            }
            Message::GoForward => {
                self.engine.go_forward();
                if let Some(state) = self.nav_bar_state.as_mut() {
                    state.0 = self.engine.get_tabs().get_current().url();
                }
                Task::none()
            }
            Message::Refresh => {
                self.engine.refresh();
                Task::none()
            }
            Message::GoHome => {
                self.engine.goto_url(&self.home);
                Task::none()
            }
            Message::GoToUrl(url) => {
                self.engine.goto_url(&to_url(&url).unwrap());
                Task::none()
            }
            Message::UpdateUrl => {
                if let Some(state) = self.nav_bar_state.as_mut() {
                    state.0 = self.engine.get_tabs().get_current().url();
                }
                Task::none()
            }
            Message::UrlChanged(url) => {
                if let Some(state) = self.nav_bar_state.as_mut() {
                    state.0 = url;
                }
                Task::none()
            }
            Message::CommandPalatteQueryChanged(query) => {
                self.command_window_state.query = query.clone();
                self.command_window_state.filtered_results = self
                    .command_window_state
                    .possible_results
                    .clone()
                    .into_iter()
                    .filter(|command| {
                        command
                            .to_string()
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                            || command
                                .inner_name()
                                .to_lowercase()
                                .contains(&query.to_lowercase())
                    })
                    .collect();
                Task::none()
            }
            Message::CommandPalatteSelected(item) => {
                self.command_window_state.selected_item = Some(item);
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
            Message::IcedEvent(event) => {
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
                            // escape to exit command palatte
                            if self.show_overlay && key == keyboard::Key::Named(key::Named::Escape)
                            {
                                return Task::done(Message::HideOverlay);
                            }
                            // ctrl + R = refresh
                            else if modifiers.control() && key == key::Key::Character("r".into())
                            {
                                return Task::done(Message::Refresh);
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
            column = column.push(tab_bar(self.engine.get_tabs()))
        }
        if self.with_nav_bar {
            column = column.push(
                hoverable(nav_bar(self.nav_bar_state.as_ref().unwrap()))
                    .on_focus_change(Message::UpdateUrl),
            )
        }
        if let Some(bookmarks) = self.bookmarks.as_ref() {
            column = column.push(bookmark_bar(bookmarks))
        }

        let browser_view = browser_view(
            self.view_size,
            self.engine.get_tabs().get_current().get_view(),
            !self.show_overlay,
        );
        if self.show_overlay {
            column = column.push(command_window(browser_view, &self.command_window_state))
        } else {
            column = column.push(browser_view);
        }

        column.into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_millis(10)).map(move |_| Message::Update),
            iced::event::listen().map(|e: iced::Event| Message::IcedEvent(Some(e))),
        ])
    }
}
