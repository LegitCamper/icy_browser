use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::widget::{scrollable, text, Column};
use iced::{Element, Length, Size};
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
    results: &'a [ResultType],
    selected_item: Option<&'a String>,
) -> ResultsList<'a> {
    ResultsList {
        results,
        selected_item,
    }
}

pub struct ResultsList<'a> {
    results: &'a [ResultType],
    selected_item: Option<&'a String>,
}

impl<'a> ResultsList<'a> {
    pub fn new(results: &'a [ResultType], selected_item: Option<&'a String>) -> Self {
        ResultsList {
            results,
            selected_item,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ResultsList<'_>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer,
    Theme: iced::widget::scrollable::Catalog + iced::widget::text::Catalog,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let mut list: Vec<Element<'_, Message, Theme, Renderer>> = Vec::new();
        let mut result_types = Vec::new();

        for result in self.results {
            if !result_types.contains(&result.to_string()) {
                result_types.push(result.to_string());
                list.push(text(result.to_string()).size(20).into())
            }
            let result_text = match result {
                ResultType::Commands(command) => command.to_string(),
                ResultType::Bookmarks(bookmark) => bookmark.to_string(),
            };

            let mut text = text(format!("   {}", result_text)).size(15);
            if let Some(selected_item) = self.selected_item {
                if result_text == *selected_item {
                    // text.
                }
            }
            list.push(text.into())
        }

        scrollable(Column::from_vec(list))
            .width(Length::Fill)
            .spacing(10)
            .draw(tree, renderer, theme, style, layout, cursor, viewport)
    }
}

impl<'a, Message, Theme, Renderer> From<ResultsList<'a>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer,
    Theme: iced::widget::scrollable::Catalog + iced::widget::text::Catalog,
{
    fn from(results_list: ResultsList) -> Self {
        Self::new(results_list)
    }
}
