use iced::widget::{row, text_input, tooltip, tooltip::Position, Button, Space};
use iced::{Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};

pub fn backward<Message: 'static + Clone>(on_backward: Message) -> Element<'static, Message> {
    tooltip_helper(
        Button::new(icon_to_text(Bootstrap::ChevronBarLeft))
            .on_press(on_backward)
            .into(),
        "Go Back",
    )
}

pub fn forward<Message: 'static + Clone>(on_forward: Message) -> Element<'static, Message> {
    tooltip_helper(
        Button::new(icon_to_text(Bootstrap::ChevronBarRight))
            .on_press(on_forward)
            .into(),
        "Go Forward",
    )
}

pub fn home<Message: 'static + Clone>(on_home: Message) -> Element<'static, Message> {
    tooltip_helper(
        Button::new(icon_to_text(Bootstrap::HouseDoor))
            .on_press(on_home)
            .into(),
        "Go Home",
    )
}

pub fn refresh<Message: 'static + Clone>(on_refresh: Message) -> Element<'static, Message> {
    tooltip_helper(
        Button::new(icon_to_text(Bootstrap::ArrowCounterclockwise))
            .on_press(on_refresh)
            .into(),
        "Refresh",
    )
}

pub fn search<Message: 'static + Clone>(
    url: String,
    url_placeholder: String,
    on_url_change: Box<dyn Fn(String) -> Message>,
    on_url_submit: Box<dyn Fn(String) -> Message>,
) -> Element<'static, Message> {
    row![
        if url.contains("https://") {
            icon_to_text(Bootstrap::Lock)
        } else {
            icon_to_text(Bootstrap::Unlock)
        },
        text_input(&url_placeholder, &url)
            .on_input(on_url_change)
            .on_submit((on_url_submit)(url))
            .on_paste(on_url_submit)
            .size(18)
            .padding(2.5)
    ]
    .into()
}

/// Creates Navigation bar widget
pub fn nav_bar<Message: 'static + Clone>(
    url: String,
    on_backward: Message,
    on_forward: Message,
    on_home: Message,
    on_refresh: Message,
    on_url_change: Box<dyn Fn(String) -> Message>,
    on_url_submit: Box<dyn Fn(String) -> Message>,
) -> Element<'static, Message> {
    row![
        backward(on_backward),
        forward(on_forward),
        home(on_home),
        refresh(on_refresh),
        Space::new(Length::Fill, Length::Shrink),
        search(
            url,
            String::from("https://site.com"),
            on_url_change,
            on_url_submit
        ),
        Space::new(Length::Fill, Length::Shrink)
    ]
    .padding(5)
    .spacing(2)
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
