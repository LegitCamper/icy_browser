#![feature(ascii_char)]

pub mod browser_widgets;
use browser_widgets::{browser_view, nav_bar, State};
pub mod browser_engines;

use iced::{executor, widget::column, Application, Command, Settings, Subscription, Theme};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

struct Browser(Arc<Mutex<State>>);

#[derive(Debug, Clone, Copy)]
pub enum Message {
    BrowserDoWork,
}

impl Application for Browser {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self(State::new()), Command::none())
    }

    fn title(&self) -> String {
        String::from("Test Browser")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::BrowserDoWork => self.0.lock().unwrap().do_work(),
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let update =
            iced::time::every(Duration::from_millis(100)).map(move |_| Message::BrowserDoWork);

        update
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let browser = browser_view(self.0.clone());
        let nav_bar = nav_bar(self.0.clone());
        column!(nav_bar, browser).into()
    }
}

fn main() -> Result<(), iced::Error> {
    Browser::run(Settings::default())
}
