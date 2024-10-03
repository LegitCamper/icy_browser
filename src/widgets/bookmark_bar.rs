use iced::{widget::Row, Element};

use super::Message;
use crate::Bookmark;

/// Creates bookmark bar widget
pub fn bookmark_bar(bookmarks: &[Bookmark]) -> Element<Message> {
    Row::from_vec(
        bookmarks
            .iter()
            .map(|bookmark| bookmark.as_button().into())
            .collect(),
    )
    .padding(5)
    .spacing(5)
    .into()
}
