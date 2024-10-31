use iced::{
    widget::{button, Row},
    Element, Length,
};

use crate::Bookmark;

/// Creates bookmark bar widget
pub fn bookmark_bar<'a, Message: Clone + 'a>(
    bookmarks: &'a [Bookmark],
    on_press: Box<dyn Fn(String) -> Message>,
) -> Element<'a, Message> {
    Row::from_vec(
        bookmarks
            .iter()
            .map(|bookmark| {
                button(bookmark.name().as_str())
                    .on_press(on_press(bookmark.url().to_string()))
                    .into()
            })
            .collect(),
    )
    .padding(5)
    .spacing(5)
    .into()
}
