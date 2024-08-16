use iced::widget::{row, tooltip, Button};
use iced::{self, Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_aw::{TabBar as TB, TabLabel};

use crate::engines::Tab;
// handle actions of all widgets in central place
use super::browser_widgets::Message as Action;

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(usize),
    TabClosed(usize),
    TitleChange(String),
    NewTab(Tab),
}

// helper function to create navigation bar
pub fn tab_bar() -> TabBar {
    TabBar::new()
}

// Simple navigation bar widget
pub struct TabBar {
    tabs: Vec<Tab>,
    active_tab: usize,
}

impl TabBar {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: 0,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => self.active_tab = tab,
            Message::TabClosed(tab) => {
                self.tabs.remove(tab);
                self.active_tab -= 1
            }
            Message::NewTab(tab) => {
                self.tabs.push(tab);
                self.active_tab += 1
            }
            Message::TitleChange(title) => {
                self.tabs.get_mut(self.active_tab).unwrap().title = title
            }
        }
    }

    pub fn view(&self) -> Element<Action> {
        let tab_bar = self
            .tabs
            .iter()
            .fold(TB::new(Action::ChangeTab), |tab_bar, tab| {
                let idx = tab_bar.size();
                tab_bar.push(idx, TabLabel::Text(tab.title.to_owned()))
            })
            .set_active_tab(&self.active_tab)
            .on_close(Action::CloseTab)
            .tab_width(Length::Shrink)
            .spacing(5.0)
            .padding(5.0);

        let new_tab = tooltip(
            Button::new(icon_to_text(Bootstrap::Plus))
                .on_press(Action::CreateTab)
                .padding(5.0),
            "New Tab",
            tooltip::Position::Bottom,
        );

        row!(tab_bar, new_tab).into()
    }
}
