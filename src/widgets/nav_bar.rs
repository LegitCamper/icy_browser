use iced::widget::{row, text::LineHeight, text_input, tooltip, tooltip::Position, Button, Space};
use iced::{Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_on_focus_widget::hoverable;

#[derive(Debug, Clone)]
pub enum Message {
    Backward,
    Forward,
    Refresh,
    Home,
    UrlChanged(String),
    UrlPasted(String),
    UrlSubmitted,
    OnUnfocus,
}

pub enum Action {
    GoBackward,
    GoForward,
    Refresh,
    GoHome,
    GoUrl(String),
    None,
}

// helper function to create navigation bar
pub fn nav_bar() -> NavBar {
    NavBar::new()
}

// Simple navigation bar widget
pub struct NavBar {
    url: String,
}

impl NavBar {
    pub fn new() -> Self {
        Self { url: String::new() }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Backward => Action::GoBackward,
            Message::Forward => Action::GoForward,
            Message::Refresh => Action::Refresh,
            Message::Home => Action::GoHome,
            Message::UrlChanged(url) => {
                self.url = url.clone();
                Action::GoUrl(url)
            }
            Message::UrlPasted(url) => {
                self.url = url.clone();
                Action::GoUrl(url)
            }
            Message::UrlSubmitted => Action::GoUrl(self.url.clone()),
            Message::OnUnfocus => {
                // TODO: get new url and update it here?
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let back = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ChevronBarLeft))
                .on_press(Message::Backward)
                .into(),
            "Go Back",
        );
        let forward = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ChevronBarRight))
                .on_press(Message::Forward)
                .into(),
            "Go Forward",
        );
        let home = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::HouseDoor))
                .on_press(Message::Home)
                .into(),
            "Go Home",
        );
        let refresh = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ArrowCounterclockwise))
                .on_press(Message::Refresh)
                .into(),
            "Refresh",
        );
        let space_left = Space::new(Length::Fill, Length::Shrink);
        let space_right = Space::new(Length::Fill, Length::Shrink);
        let search = hoverable(
            text_input("https://site.com", self.url.as_ref())
                .on_input(Message::UrlChanged)
                .on_paste(Message::UrlPasted)
                .on_submit(Message::UrlSubmitted)
                .line_height(LineHeight::Relative(2.0))
                .into(),
        )
        .on_unfocus(Message::OnUnfocus);

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
