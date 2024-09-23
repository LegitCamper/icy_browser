use iced::widget::{row, text::LineHeight, text_input, tooltip, tooltip::Position, Button, Space};
use iced::{Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};

use super::Message;

/// Holds the state of infomation in nav_bar
pub struct NavBarState(pub String);

impl NavBarState {
    pub fn new() -> Self {
        NavBarState(String::new())
    }
}

impl Default for NavBarState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn nav_bar(state: &NavBarState) -> Element<Message> {
    let back = tooltip_helper(
        Button::new(icon_to_text(Bootstrap::ChevronBarLeft))
            .on_press(Message::GoBackward)
            .into(),
        "Go Back",
    );
    let forward = tooltip_helper(
        Button::new(icon_to_text(Bootstrap::ChevronBarRight))
            .on_press(Message::GoForward)
            .into(),
        "Go Forward",
    );
    let home = tooltip_helper(
        Button::new(icon_to_text(Bootstrap::HouseDoor))
            .on_press(Message::GoHome)
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
    let search = text_input("https://site.com", &state.0)
        .on_input(Message::UrlChanged)
        .on_paste(Message::GoToUrl)
        .on_submit(Message::GoToUrl(state.0.to_string()))
        .line_height(LineHeight::Relative(2.0));

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

fn tooltip_helper<'a, Message: 'a>(
    element: Element<'a, Message>,
    tooltip_str: &'a str,
) -> Element<'a, Message> {
    tooltip(element, tooltip_str, Position::Bottom)
        .padding(5)
        .into()
}
