use iced::widget::{center, column, container, mouse_area, opaque, stack, text_input};
use iced::widget::{scrollable, text, Column};
use iced::{border, Color, Element, Length, Theme};
use iced_event_wrapper::wrapper;
use strum_macros::Display;

use super::Message;
use crate::Bookmark;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ResultType {
    Commands(Message),
    Bookmarks(Bookmark),
}

impl ResultType {
    pub fn inner_name(&self) -> String {
        match self {
            ResultType::Commands(command) => command.to_string(),
            ResultType::Bookmarks(bookmark) => format!("Go to: {}", bookmark.url()),
        }
    }
}

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

    pub fn next_item(&mut self) {
        match &self.selected_item {
            None => {
                self.selected_item = self
                    .filtered_results
                    .get(0)
                    .map(|res| res.inner_name())
                    .or_else(|| None)
            }
            Some(selected_item) => {
                if let Some(last) = self.filtered_results.last() {
                    if *selected_item != last.inner_name() {
                        let pos = self
                            .filtered_results
                            .iter()
                            .position(|res| res.inner_name() == *selected_item)
                            .unwrap();
                        self.selected_item = Some(self.filtered_results[pos + 1].inner_name());
                    }
                }
            }
        }
    }

    pub fn previous_item(&mut self) {
        match &self.selected_item {
            None => {
                self.selected_item = self
                    .filtered_results
                    .get(0)
                    .map(|res| res.inner_name())
                    .or_else(|| None)
            }
            Some(selected_item) => {
                if let Some(first) = self.filtered_results.first() {
                    if *selected_item != first.inner_name() {
                        let pos = self
                            .filtered_results
                            .iter()
                            .position(|res| res.inner_name() == *selected_item)
                            .unwrap();
                        self.selected_item = Some(self.filtered_results[pos - 1].inner_name());
                    }
                }
            }
        }
    }
}

impl Default for CommandWindowState {
    fn default() -> Self {
        Self::new(None)
    }
}

pub fn command_palatte<'a>(
    base: impl Into<Element<'a, Message>>,
    state: &'a CommandWindowState,
) -> Element<'a, Message> {
    let window = container(column![
        text_input("Command Menu", &state.query)
            .on_input(Message::CommandPalatteQueryChanged)
            .size(25),
        container(results_list(
            state.filtered_results.as_slice(),
            state.selected_item.clone(),
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

    let stack = stack![
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
    ];

    wrapper(stack)
        .on_keyboard_event(|event| Message::CommandPalatteKeyboardEvent(Some(event)))
        .into()
}

fn results_list<'a>(
    results: &'a [ResultType],
    selected_item: Option<String>,
) -> Element<'a, Message> {
    let mut list = Vec::new();
    let mut result_types = Vec::new();

    for result in results {
        if !result_types.contains(&result.to_string()) {
            result_types.push(result.to_string());
            list.push(text(result.to_string()).size(20).into())
        }

        let mut text = text(format!("   {}", result.inner_name())).size(15);
        if let Some(selected_item) = selected_item.as_ref() {
            if result.inner_name() == *selected_item {
                // highlight currently selected element
                // text = text.color(iced::Color::new(50., 138., 176., 100.));
                text = text.size(30)
            }
        }
        list.push(text.into())
    }

    scrollable(Column::from_vec(list))
        .width(Length::Fill)
        .spacing(10)
        .into()
}
