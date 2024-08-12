// Simple browser with familiar browser widget with the ultralight(webkit) webengine as a backend

use iced::{executor, Command, Subscription};
use iced::{widget::column, Application, Settings, Theme};
use iced_aw::BOOTSTRAP_FONT_BYTES;

use icy_browser::{browser_view, nav_bar, tab_bar, BrowserView, NavBar, State, TabBar, Ultralight};

use std::borrow::Borrow;
use std::time::Duration;

fn main() -> Result<(), iced::Error> {
    let bootstrap_font = BOOTSTRAP_FONT_BYTES.into();
    let settings = Settings {
        fonts: vec![bootstrap_font],
        ..Default::default()
    };
    Browser::run(settings)
}

struct Browser {
    state: State<Ultralight>,
    tab_bar: TabBar<Ultralight>,
    nav_bar: NavBar<Ultralight>,
    browser_view: BrowserView<Ultralight>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabBar(tab_bar::Message),
    NavBar(nav_bar::Message),
    BrowserView(browser_view::Message),
    DoWork,
}

impl Application for Browser {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let state = State::new_ultralight();
        let tab_bar = tab_bar(state.clone());
        let nav_bar = nav_bar(state.clone());
        let browser_view = browser_view(state.clone());

        (
            Self {
                state,
                tab_bar,
                nav_bar,
                browser_view,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Basic Browser")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(100)).map(move |_| Message::DoWork)
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::DoWork => self.state.borrow().do_work(),
            Message::NavBar(nav_bar_message) => match self.nav_bar.update(nav_bar_message) {
                nav_bar::Action::None => {}
            },
            Message::TabBar(tab_bar_message) => match self.tab_bar.update(tab_bar_message) {
                tab_bar::Action::None => {}
            },
            Message::BrowserView(browser_view) => match self.browser_view.update(browser_view) {
                browser_view::Action::None => {}
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let tab_bar = self.tab_bar.view().map(Message::TabBar);
        let nav_bar = self.nav_bar.view().map(Message::NavBar);
        let browser_view = self.browser_view.view().map(Message::BrowserView);

        column!(tab_bar, nav_bar, browser_view).into()
    }
}
