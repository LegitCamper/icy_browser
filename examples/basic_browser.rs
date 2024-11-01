// Simple browser with familiar browser widgets and the ultralight(webkit) webengine as a backend

use iced::widget::{center, column, container, text, Space};
use iced::{Element, Length, Settings, Subscription, Task, Theme};
use icy_browser::iced_webview::{Action, Ultralight, WebView};
use icy_browser::{bookmark_bar, get_fonts, nav_bar, tab_bar, Bookmark};
use std::time::Duration;
use url::Url;

const HOME: &'static str = "https://google.com";

fn main() -> iced::Result {
    let settings = Settings {
        fonts: get_fonts(),
        ..Default::default()
    };
    iced::application("Basic Browser", App::update, App::view)
        .subscription(App::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run_with(App::new)
}

#[derive(Debug)]
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
    GoFoward,
    GoHome,
    Refresh,
}

struct App {
    webview: WebView<Ultralight, Message>,
    tab: Option<u32>,
    tabs: Vec<Tab>,
    bookmarks: Vec<Bookmark>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            App {
                webview: WebView::new()
                    .on_title_change(Message::TitleChanged)
                    .on_title_change(Message::UrlChanged)
                    .on_create_view(Message::TabCreated),
                tab: None,
                tabs: Vec::new(),
                bookmarks: vec![Bookmark::new("https://www.rust-lang.org", "rust-lang.org")],
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
                            url: String::new(),
                            title,
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
                            title: String::new(),
                            url,
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
                self.tabs.push(Tab {
                    url: String::new(),
                    title: String::new(),
                });
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
            Message::GoFoward => return self.webview.update(Action::GoForward),
            Message::GoHome => {
                return self
                    .webview
                    .update(Action::GoToUrl(Url::parse(HOME).unwrap()))
            }
            Message::Refresh => return self.webview.update(Action::Refresh),
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        if let Some(current_tab) = self.tab {
            let tabs: Vec<String> = self.tabs.iter().map(|tab| tab.title.clone()).collect();
            let tab_bar = if !tabs.is_empty() {
                container(tab_bar(
                    tabs,
                    current_tab,
                    Box::new(Message::ChangeTab),
                    Box::new(Message::CloseTab),
                    Message::CreateDefaultTab,
                ))
            } else {
                container(Space::new(Length::Fill, 10))
            };

            let url = if let Some(tab) = self.tabs.get(current_tab as usize) {
                tab.url.clone()
            } else {
                "loading...".to_string()
            };
            let nav_bar = nav_bar(
                url,
                Message::GoBack,
                Message::GoFoward,
                Message::GoHome,
                Message::Refresh,
                Box::new(Message::UrlChanged),
                Box::new(Message::Gotourl),
            );
            column![
                tab_bar,
                nav_bar,
                bookmark_bar(self.bookmarks.as_slice(), Box::new(Message::Gotourl)),
                self.webview.view().map(Message::Webview)
            ]
            .into()
        } else {
            center(text("Loading page")).into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(10)).map(|_| Message::UpdateWebview)
    }
}
