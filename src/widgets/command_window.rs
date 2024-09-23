use iced::widget::{center, column, container, mouse_area, opaque, stack, text_input};
use iced::{border, Color, Element, Length, Theme};
use iced_aw::SelectionList;
use strum::IntoEnumIterator;

use super::Message;

pub struct CommandWindowState {
    pub query: String,
    actions: Vec<String>,
    pub selected_action: String,
    pub selected_index: usize,
}

impl CommandWindowState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            actions: Message::iter().map(|e| e.clone().to_string()).collect(),
            selected_action: String::new(),
            selected_index: 0,
        }
    }
}

pub fn command_window<'a>(
    base: impl Into<Element<'a, Message>>,
    state: &'a CommandWindowState,
) -> Element<'a, Message> {
    let window = container(column![
        text_input("Command Menu", &state.query).on_input(Message::QueryChanged),
        SelectionList::new(&state.actions, Message::CommandSelectionChanged)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme, _| iced_aw::style::selection_list::Style {
                text_color: theme.palette().text.into(),
                background: theme.palette().background.into(),
                ..Default::default()
            }),
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
