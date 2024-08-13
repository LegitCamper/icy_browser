// Simple browser with familiar browser widget with the ultralight(webkit) webengine as a backend

use iced::{executor, Application, Command, Settings, Subscription, Theme};
use iced_aw::BOOTSTRAP_FONT_BYTES;
use std::time::Duration;

use icy_browser::{browser_widgets, BrowserWidget, Ultralight};

fn main() -> Result<(), iced::Error> {
    // This imports `icons` for tab_bar and nav_bar
    let bootstrap_font = BOOTSTRAP_FONT_BYTES.into();
    let settings = Settings {
        fonts: vec![bootstrap_font],
        ..Default::default()
    };
    Browser::run(settings)
}

struct Browser {
    widgets: BrowserWidget<Ultralight>,
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowserWidget(browser_widgets::Message),
    DoWork,
}

impl Application for Browser {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let (widgets, command) = BrowserWidget::new_ultralight()
            .with_homepage("https://duckduckgo.com")
            .with_tab_bar()
            .with_nav_bar()
            // .with_browsesr_view()
            .build();

        (Self { widgets }, command.map(Message::BrowserWidget))
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
            Message::BrowserWidget(msg) => self.widgets.update(msg).map(Message::BrowserWidget),
            Message::DoWork => self
                .widgets
                .update(browser_widgets::Message::DoWork)
                .map(Message::BrowserWidget),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        self.widgets.view().map(Message::BrowserWidget).into()
    }
}
