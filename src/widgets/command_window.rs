use iced::widget::{center, column, container, mouse_area, opaque, stack, text_input};
use iced::{border, Color, Element, Theme};

use super::Message;

pub fn command_window<'a>(
    base: impl Into<Element<'a, Message>>,
    query: &str,
) -> Element<'a, Message> {
    let window = container(column![
        text_input("Command Menu", query).on_input(Message::QueryChanged),
    ])
    .padding(10)
    .center(600)
    .style(|theme: &Theme| container::Style {
        background: Some(theme.palette().background.into()),
        border: border::rounded(10),
        ..container::Style::default()
    });

    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(window)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(Message::HideOverlay)
        )
    ]
    .into()
}
