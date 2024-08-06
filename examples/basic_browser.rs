use iced::{executor, Command, Subscription};
use iced::{widget::column, Application, Settings, Theme};

use icy_browser::{browser_view, nav_bar, State};

use std::time::Duration;

fn main() -> Result<(), iced::Error> {
    Browser::run(Settings::default())
}

struct Browser(State);

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
        (Self(State::new()), Command::none())
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
            Message::DoWork => self.0.do_work(),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let browser = browser_view(&self.0);
        let nav_bar = nav_bar(&self.0).unwrap();

        column!(nav_bar, browser).into()
    }
}
