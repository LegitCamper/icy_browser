use iced::widget::{row, text::LineHeight, text_input, tooltip, tooltip::Position, Button, Space};
use iced::{Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};

/// Creates Navigation bar widget
pub fn nav_bar<Message: 'static + Clone>(
    url: String,
    on_go_backward: Message,
    on_go_forward: Message,
    on_go_home: Message,
    on_refresh: Message,
    on_url_change: Box<dyn Fn(String) -> Message>,
    on_url_submit: Box<dyn Fn(String) -> Message>,
) -> Element<'static, Message> {
    let back = tooltip_helper(
        Button::new(icon_to_text(Bootstrap::ChevronBarLeft))
            .on_press(on_go_backward)
            .into(),
        "Go Back",
    );
    let forward = tooltip_helper(
        Button::new(icon_to_text(Bootstrap::ChevronBarRight))
            .on_press(on_go_forward)
            .into(),
        "Go Forward",
    );
    let home = tooltip_helper(
        Button::new(icon_to_text(Bootstrap::HouseDoor))
            .on_press(on_go_home)
            .into(),
        "Go Home",
    );
    let refresh = tooltip_helper(
        Button::new(icon_to_text(Bootstrap::ArrowCounterclockwise))
            .on_press(on_refresh)
            .into(),
        "Refresh",
    );
    let space_left = Space::new(Length::Fill, Length::Shrink);
    let space_right = Space::new(Length::Fill, Length::Shrink);
    let search = text_input("https://site.com", url.as_str())
        .on_input(on_url_change)
        .on_submit((on_url_submit)(url))
        .on_paste(on_url_submit)
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
