use iced::widget::{row, text::LineHeight, text_input, tooltip, tooltip::Position, Button, Space};
use iced::{Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_on_focus_widget::hoverable;

// handle actions of all widgets in central place
use super::browser_widgets::Message as Action;

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
}

// helper function to create navigation bar
pub fn nav_bar(url: &str) -> NavBar {
    NavBar::new(url)
}

// Simple navigation bar widget
pub struct NavBar {
    url: String,
}

impl NavBar {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UrlChanged(url) => self.url = url,
        }
    }

    pub fn view(&self) -> Element<Action> {
        let back = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ChevronBarLeft))
                .on_press(Action::GoBackward)
                .into(),
            "Go Back",
        );
        let forward = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ChevronBarRight))
                .on_press(Action::GoForward)
                .into(),
            "Go Forward",
        );
        let home = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::HouseDoor))
                .on_press(Action::GoHome)
                .into(),
            "Go Home",
        );
        let refresh = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ArrowCounterclockwise))
                .on_press(Action::Refresh)
                .into(),
            "Refresh",
        );
        let space_left = Space::new(Length::Fill, Length::Shrink);
        let space_right = Space::new(Length::Fill, Length::Shrink);
        let search = hoverable(
            text_input("https://site.com", self.url.as_ref())
                .on_input(Action::UrlChanged)
                .on_paste(Action::UrlSubmitted)
                .on_submit(Action::UrlSubmitted(self.url.clone()))
                .line_height(LineHeight::Relative(2.0))
                .into(),
        );
        // .on_unfocus(Action::PushNewUrl);

        row!(
            back,
            forward,
            home,
            refresh,
            space_left,
            search,
            space_right
        )
        .into()
    }
}

fn tooltip_helper<'a, Message: 'a>(
    element: Element<'a, Message>,
    tooltip_str: &'a str,
) -> Element<'a, Message> {
    tooltip(element, tooltip_str, Position::Bottom)
        .padding(5)
        .into()
}
