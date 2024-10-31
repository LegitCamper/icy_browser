// Simple browser with familiar browser widgets and the ultralight(webkit) webengine as a backend

use iced::widget::{center, column, text};
use iced::{Element, Settings, Subscription, Task, Theme};
use icy_browser::iced_webview::{Action, Ultralight, WebView};
use icy_browser::{bookmark_bar, get_fonts, Bookmark};
use std::time::Duration;
use url::Url;

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

#[derive(Debug, Clone)]
enum Message {
    UpdateWebview,
    Webview(Action),
    // TitleChanged(String),
    // UrlChanged(String),
    InitTab, // Called after the first tab is created, to set tab to 0
    CreateTab(String),
    TabCreated,
    Gotourl(String),
}

#[derive(Debug)]
struct Tab {
    url: String,
    title: String,
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
                    // .on_title_change(Message::TitleChanged)
                    // .on_title_change(Message::UrlChanged)
                    .on_create_view(Message::TabCreated),
                tab: None,
                tabs: Vec::new(),
                bookmarks: vec![Bookmark::new("https://www.rust-lang.org", "rust-lang.org")],
            },
            Task::done(Message::CreateTab("https://google.com".to_string())),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Webview(msg) => return self.webview.update(msg),
            Message::UpdateWebview => return self.webview.update(Action::Update),
            // Message::TitleChanged(title) => self.title = title,
            // Message::UrlChanged(url) => self.url = url,
            Message::InitTab => self.tab = Some(0),
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
            Message::Gotourl(url) => {
                return self
                    .webview
                    .update(Action::GoToUrl(Url::parse(&url).unwrap()))
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        if self.tab.is_some() {
            // let tabs = self.tabs.iter().enumerate().map(|(index, tab)| {
            //     (in)
            // });
            column![
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
