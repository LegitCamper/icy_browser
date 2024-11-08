// Simple keyboard driven browser using the ultralight(webkit) webengine as a backend

use iced::widget::{center, container, text, Space};
use iced::{event, keyboard, Element, Settings, Subscription, Task, Theme};
use icy_browser::iced_webview::{
    advanced::{Action, WebView},
    Ultralight, ViewId,
};
use icy_browser::{
    bookmark_bar, command_palette, get_fonts, nav_bar, tab_bar, Bookmark, CommandPaletteState,
    Shortcut, ShortcutModifier,
};
use std::time::Duration;
use strum_macros::Display;
use url::Url;

const HOME: &'static str = "https://google.com";

fn main() -> iced::Result {
    let settings = Settings {
        fonts: get_fonts(),
        ..Default::default()
    };

    println!("Press 'Crtl + E' to open to Command palette");

    iced::application("Keyboard Driven Browser", Browser::update, Browser::view)
        .subscription(Browser::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run_with(Browser::new)
}

#[derive(Debug, Default)]
struct Tab {
    id: ViewId,
    url: String,
    title: String,
}

#[derive(Debug, Clone, Display)]
enum Message {
    UpdateWebview,
    Webview(Action),
    Event(event::Event),
    TitleChanged(ViewId, String),
    UrlChanged(ViewId, String),
    InitTab, // Called after the first tab is created, to set tab to 0
    CreateTab(String),
    CreateDefaultTab,
    TabCreated(ViewId),
    CloseTab(ViewId),
    ChangeTab(ViewId),
    Gotourl(String),
    GoBack,
    GoForward,
    GoHome,
    Refresh,
    ToggleCommandPalette,
    HideCommandPalette,
}

struct Browser<'a> {
    webview: WebView<Ultralight, Message>,
    tab: Option<ViewId>,
    tabs: Vec<Tab>,
    bookmarks: Vec<Bookmark>,
    shortcuts: Vec<Shortcut<'a, Message>>,
    command_palette_state: CommandPaletteState<Message>,
    show_palette: bool,
}

impl<'a> Browser<'_> {
    fn new() -> (Self, Task<Message>) {
        (
            Browser {
                webview: WebView::new()
                    .on_title_change(Message::TitleChanged)
                    .on_url_change(Message::UrlChanged)
                    .on_create_view(Message::TabCreated),
                tab: None,
                tabs: Vec::new(),
                bookmarks: vec![
                    Bookmark::new("https://www.rust-lang.org", "rust-lang.org"),
                    Bookmark::new(
                        "https://github.com/LegitCamper/icy_browser",
                        "icy_browser github",
                    ),
                    Bookmark::new("https://docs.rs/iced/latest/iced/", "iced docs"),
                ],
                shortcuts: vec![
                    Shortcut::new(Message::CreateDefaultTab, ShortcutModifier::Ctrl, "t"),
                    Shortcut::new(Message::ToggleCommandPalette, ShortcutModifier::Ctrl, "e"),
                ],
                command_palette_state: CommandPaletteState::default(),
                show_palette: false,
            },
            Task::done(Message::CreateTab(HOME.to_string())),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Webview(msg) => return self.webview.update(msg),
            Message::UpdateWebview => {
                if let Some(tab) = self.tab {
                    return self.webview.update(Action::Update(tab));
                }
            }
            Message::TitleChanged(id, title) => {
                if let Some(tab) = self.tabs.get_mut(id) {
                    tab.title = title
                } else {
                    self.tabs.push(Tab {
                        title,
                        ..Default::default()
                    });
                }
            }
            Message::UrlChanged(id, url) => {
                if let Some(tab) = self.tabs.get_mut(id) {
                    tab.url = url
                } else {
                    self.tabs.push(Tab {
                        url,
                        ..Default::default()
                    });
                }
            }
            Message::InitTab => self.tab = Some(0),
            Message::CreateDefaultTab => return Task::done(Message::CreateTab(HOME.to_string())),
            Message::CreateTab(url) => {
                return self
                    .webview
                    .update(Action::CreateView(iced_webview::PageType::Url(url)));
            }
            Message::TabCreated(id) => self.tab = Some(id),
            Message::CloseTab(id) => return self.webview.update(Action::CloseView(id)),
            Message::ChangeTab(id) => self.tab = Some(id),
            Message::Gotourl(url) => {
                return self.webview.update(Action::GoToUrl(
                    self.tab.unwrap(),
                    Url::parse(&url).unwrap(),
                ))
            }
            Message::GoBack => return self.webview.update(Action::GoBackward(self.tab.unwrap())),
            Message::GoForward => return self.webview.update(Action::GoForward(self.tab.unwrap())),
            Message::GoHome => return self.update(Message::Gotourl(HOME.to_string())),
            Message::Refresh => return self.webview.update(Action::Refresh(self.tab.unwrap())),
            Message::ToggleCommandPalette => self.show_palette = !self.show_palette,
            Message::HideCommandPalette => self.show_palette = false,
            Message::Event(event) => match event {
                iced::Event::Keyboard(event) => match event {
                    keyboard::Event::KeyPressed {
                        key,
                        modified_key: _,
                        physical_key: _,
                        location: _,
                        modifiers,
                        text: _,
                    } => {
                        for shortcut in self.shortcuts.iter() {
                            if shortcut.is_pressed(&key, &modifiers) {
                                return Task::done(shortcut.action.clone());
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        if let Some(tab) = self.tab {
            let webview = self.webview.view(tab).map(Message::Webview);
            if self.show_palette {
                command_palette(
                    webview,
                    &self.command_palette_state,
                    Message::HideCommandPalette,
                )
            } else {
                webview.into()
            }
        } else {
            center(text("loading...")).into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_millis(10)).map(|_| Message::UpdateWebview),
            event::listen().map(Message::Event),
        ])
    }
}
