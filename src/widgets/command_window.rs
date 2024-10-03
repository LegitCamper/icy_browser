use iced::widget::{center, column, container, mouse_area, opaque, stack, text_input};
use iced::{border, Color, Element, Font, Length, Theme};
use iced_aw::SelectionList;
use strum::IntoEnumIterator;

use super::Message;

// pub enum ResultType {
//     Command(Message),
//     // Bookmark,
// }

pub struct CommandWindowState {
    pub query: String,
    commands: Vec<String>,
    pub selected_action: String,
    pub selected_index: usize,
}

impl CommandWindowState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            commands: Message::iter().map(|e| e.clone().to_string()).collect(),
            selected_action: String::new(),
            selected_index: 0,
        }
    }
}

impl Default for CommandWindowState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn command_window<'a>(
    base: impl Into<Element<'a, Message>>,
    state: &'a CommandWindowState,
) -> Element<'a, Message> {
    let window = container(column![
        text_input("Command Menu", &state.query)
            .on_input(Message::QueryChanged)
            .size(25),
        SelectionList::new_with(
            &state.commands,
            Message::CommandSelectionChanged,
            15.,
            5,
            |theme: &Theme, _| iced_aw::style::selection_list::Style {
                text_color: theme.palette().text,
                background: theme.palette().background.into(),
                ..Default::default()
            },
            None,
            Font::DEFAULT
        )
        .width(Length::Fill)
        .height(Length::Fill)
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
