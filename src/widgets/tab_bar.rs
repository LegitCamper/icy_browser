use iced::widget::{row, tooltip, Button};
use iced::{self, Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_aw::{TabBar as TB, TabLabel};

use super::browser_widgets::Message;
use crate::engines::Tab;

// helper function to create navigation bar
pub fn tab_bar<TabInfo>(tabs: Vec<Tab<TabInfo>>, active_tab: usize) -> Element<'static, Message> {
    let tab_bar = tabs
        .iter()
        .fold(TB::new(Message::ChangeTab), |tab_bar, tab| {
            let idx = tab_bar.size();
            tab_bar.push(idx, TabLabel::Text(tab.title()))
        })
        .set_active_tab(&active_tab)
        .on_close(Message::CloseTab)
        .tab_width(Length::Shrink)
        .spacing(5.0)
        .padding(5.0);

    let new_tab = tooltip(
        Button::new(icon_to_text(Bootstrap::Plus))
            .on_press(Message::CreateTab)
            .padding(5.0),
        "New Tab",
        tooltip::Position::Bottom,
    );

    row!(tab_bar, new_tab).into()
}
