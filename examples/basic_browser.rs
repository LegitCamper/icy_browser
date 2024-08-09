// Simple browser with familiar browser widget with the ultralight(webkit) webengine as a backend

use iced::{executor, Command, Subscription};
use iced::{widget::column, Application, Settings, Theme};
use iced_aw::BOOTSTRAP_FONT_BYTES;

use icy_browser::{browser_view, nav_bar, tab_bar, State, Ultralight};

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

struct Browser(State<Ultralight>);

#[derive(Debug, Clone, Copy)]
pub enum Message {
    DoWork,
}

impl Application for Browser {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self(State::new_ultralight()), Command::none())
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
            Message::DoWork => self.0.borrow().do_work(),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let tab_bar = tab_bar(self.0.clone());
        let nav_bar = nav_bar(self.0.clone());
        let browser_view = browser_view(self.0.clone());

        column!(tab_bar, nav_bar, browser_view).into()
    }
}
