use iced::widget::{center, column, container, mouse_area, opaque, stack, text_input};
use iced::{border, Color, Element, Length, Theme};

use super::Message;
use crate::Bookmark;

mod results;
pub use results::{results_list, ResultType, ResultsList};

pub struct CommandWindowState {
    pub query: String,
    pub possible_results: Vec<ResultType>,
    pub filtered_results: Vec<ResultType>,
    pub selected_item: Option<String>,
}

impl CommandWindowState {
    pub fn new(bookmarks: Option<Vec<Bookmark>>) -> Self {
        // This may need to be extended in the future
        let mut results: Vec<ResultType> = Vec::new();
        results.extend(
            vec![
                Message::GoBackward,
                Message::GoForward,
                Message::Refresh,
                Message::GoHome,
                Message::CloseCurrentTab,
            ]
            .into_iter()
            .map(|msg| ResultType::Commands(msg)),
        );
        if let Some(bookmarks) = bookmarks {
            results.extend(
                bookmarks
                    .into_iter()
                    .map(|bookmark| ResultType::Bookmarks(bookmark)),
            );
        };

        Self {
            query: String::new(),
            possible_results: results.clone(),
            filtered_results: results,
            selected_item: None,
        }
    }
}

impl Default for CommandWindowState {
    fn default() -> Self {
        Self::new(None)
    }
}

pub fn command_window<'a>(
    base: impl Into<Element<'a, Message>>,
    state: &CommandWindowState,
) -> Element<'a, Message> {
    let window: iced::widget::Container<'_, super::Message, Theme, iced::Renderer> =
        container(column![
            text_input("Command Menu", &state.query)
                .on_input(Message::CommandPalatteQueryChanged)
                .size(25),
            container(results_list(
                state.filtered_results.as_slice(),
                state.selected_item.as_ref(),
            ))
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
            .on_press(Message::HideOverlay),
        )
    ]
    .into()
}
