use iced::{widget::column, Sandbox, Settings, Theme};

use icy_browser::{browser_view, nav_bar, State};

fn main() -> Result<(), iced::Error> {
    Browser::run(Settings::default())
}

struct Browser(State);

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Sandbox for Browser {
    type Message = Message;

    fn new() -> Self {
        Self(State::new())
    }

    fn title(&self) -> String {
        String::from("Basic Browser")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let browser = browser_view(&self.0);
        let nav_bar = nav_bar(&self.0).unwrap();

        column!(nav_bar, browser).into()
    }
}
