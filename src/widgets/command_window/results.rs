use iced::widget::{scrollable, text, Column};
use iced::{Element, Length, Renderer, Theme};
use strum_macros::Display;

use super::super::{Bookmark, Message};

#[derive(Clone, Display, PartialEq)]
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

pub fn results_list<'a>(
    results: &[ResultType],
    selected_item: Option<&String>,
) -> Element<'a, Message, Theme, Renderer> {
    let mut list = Vec::new();
    let mut result_types = Vec::new();

    for result in results {
        if !result_types.contains(&result.to_string()) {
            result_types.push(result.to_string());
            list.push(text(result.to_string()).size(20).into())
        }
        let result_text = match result {
            ResultType::Commands(command) => command.to_string(),
            ResultType::Bookmarks(bookmark) => bookmark.to_string(),
        };

        let mut text = text(format!("   {}", result_text)).size(15);
        if let Some(selected_item) = selected_item {
            if result_text == *selected_item {
                // text.
            }
        }
        list.push(text.into())
    }

    scrollable(Column::from_vec(list))
        .width(Length::Fill)
        .spacing(10)
        .into()
}
