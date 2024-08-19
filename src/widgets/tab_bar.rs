use iced::widget::{row, tooltip, Button};
use iced::{self, Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_aw::{TabBar as TB, TabLabel};

use super::{Message, TabSelectionType};
use crate::engines::{TabInfo, Tabs};

// helper function to create navigation bar
pub fn tab_bar<Info: TabInfo>(tabs: &Tabs<Info>) -> Element<'static, Message> {
    let current_id = tabs.get_current_id();
    let active_tab = tabs
        .tabs()
        .iter()
        .position(|tab| tab.id() == current_id)
        .expect("Failed to find tab with that id");

    let tab_bar = tabs
        .tabs()
        .iter()
        .fold(
            TB::new(|index| Message::ChangeTab(TabSelectionType::Index(index))),
            |tab_bar, tab| {
                let id = tab_bar.size();
                let title = if tab.title().is_empty() {
                    String::from("New Tab")
                } else {
                    tab.title()
                };
                tab_bar.push(id, TabLabel::Text(title))
            },
        )
        .set_active_tab(&active_tab)
        .on_close(|index| Message::CloseTab(TabSelectionType::Index(index)))
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
