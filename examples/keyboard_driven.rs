// Simple keybaord driven browser using the ultralight(webkit) webengine as a backend

use iced::widget::{center, column, container, text, Space};
use iced::{Element, Length, Settings, Subscription, Task, Theme};
use icy_browser::iced_webview::{Action, Ultralight, WebView};
use icy_browser::{
    bookmark_bar, get_fonts, nav_bar, tab_bar, Bookmark, Shortcut, ShortcutModifier,
};
use std::time::Duration;
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
    url: String,
    title: String,
}

#[derive(Debug, Clone)]
enum Message {
    UpdateWebview,
    Webview(Action),
    TitleChanged(String),
    UrlChanged(String),
    InitTab, // Called after the first tab is created, to set tab to 0
    CreateTab(String),
    CreateDefaultTab,
    TabCreated,
    CloseTab(u32),
    ChangeTab(u32),
    Gotourl(String),
    GoBack,
    GoForward,
    GoHome,
    Refresh,
    ToggleCommandPalette,
}

struct Browser<'a> {
    webview: WebView<Ultralight, Message>,
    tab: Option<u32>,
    tabs: Vec<Tab>,
    bookmarks: Vec<Bookmark>,
    shortcuts: Vec<Shortcut<'a, Message>>,
    show_palatte: bool,
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
                show_palatte: false,
            },
            Task::done(Message::CreateTab(HOME.to_string())),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Webview(msg) => return self.webview.update(msg),
            Message::UpdateWebview => return self.webview.update(Action::Update),
            Message::TitleChanged(title) => {
                if let Some(tab) = self.tab {
                    if let Some(tab) = self.tabs.get_mut(tab as usize) {
                        tab.title = title
                    } else {
                        self.tabs.push(Tab {
                            title,
                            ..Default::default()
                        });
                    }
                }
            }
            Message::UrlChanged(url) => {
                if let Some(tab) = self.tab {
                    if let Some(tab) = self.tabs.get_mut(tab as usize) {
                        tab.url = url
                    } else {
                        self.tabs.push(Tab {
                            url,
                            ..Default::default()
                        });
                    }
                }
            }
            Message::InitTab => self.tab = Some(0),
            Message::CreateDefaultTab => return Task::done(Message::CreateTab(HOME.to_string())),
            Message::CreateTab(url) => {
                return self
                    .webview
                    .update(Action::CreateView(iced_webview::PageType::Url(url)));
            }
            Message::TabCreated => {
                if self.tab.is_none() {
                    return Task::done(Action::ChangeView(0))
                        .map(Message::Webview)
                        .chain(Task::done(Message::InitTab));
                }
            }
            Message::CloseTab(index) => {
                return self.webview.update(Action::CloseView(index as u32))
            }
            Message::ChangeTab(index) => {
                return self.webview.update(Action::ChangeView(index as u32))
            }
            Message::Gotourl(url) => {
                return self
                    .webview
                    .update(Action::GoToUrl(Url::parse(&url).unwrap()))
            }
            Message::GoBack => return self.webview.update(Action::GoBackward),
            Message::GoForward => return self.webview.update(Action::GoForward),
            Message::GoHome => {
                return self
                    .webview
                    .update(Action::GoToUrl(Url::parse(HOME).unwrap()))
            }
            Message::Refresh => return self.webview.update(Action::Refresh),
            Message::ToggleCommandPalette => self.show_palatte = !self.show_palatte,
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![self.webview.view().map(Message::Webview)].into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(10)).map(|_| Message::UpdateWebview)
    }
}
