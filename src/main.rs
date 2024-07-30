#![feature(ascii_char)]

pub mod browser_widgets;
use browser_widgets::{Browser, BrowserState, Config, State};
pub mod browser_engines;
use browser_engines::BrowserEngine;

use iced::{
    executor,
    widget::{column, container, text},
    Application, Command, Settings, Subscription, Theme,
};
use std::time::Duration;

struct App {
    state: State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    BrowserDoWork,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let state = BrowserState::new();
        (Self { state }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Test Browser")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::BrowserDoWork => self.state.lock().unwrap().do_work(),
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let update =
            iced::time::every(Duration::from_millis(10)).map(move |_| Message::BrowserDoWork);

        update
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        container(browser_widgets::Browser::new(self.state.clone())).into()
    }
}

fn main() -> Result<(), iced::Error> {
    App::run(Settings::default())
}
