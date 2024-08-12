use iced::widget::{row, tooltip, Button};
use iced::{self, Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_aw::{TabBar as TB, TabLabel};
use url::Url;

use super::{BrowserEngine, State};

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(usize),
    TabClosed(usize),
    NewTab,
}

// helper function to create navigation bar
pub fn tab_bar<Engine: BrowserEngine>(state: State<Engine>) -> TabBar<Engine> {
    TabBar::new(state)
}

// Simple navigation bar widget
pub struct TabBar<Engine: BrowserEngine> {
    state: State<Engine>,
}

impl<Engine: BrowserEngine> TabBar<Engine> {
    pub fn new(state: State<Engine>) -> Self {
        Self { state }
    }

    pub fn update(&mut self, event: Message) {
        let mut webengine = self.state.webengine.borrow_mut();

        match event {
            Message::TabSelected(index) => webengine.goto_tab(index as u32).unwrap(),
            Message::TabClosed(index) => webengine.close_tab(index as u32),
            Message::NewTab => {
                webengine.new_tab(&Url::parse(&self.state.config.start_page).unwrap())
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let webengine = self.state.webengine.borrow();

        let tab_bar = webengine
            .get_tabs()
            .iter()
            .fold(TB::new(Message::TabSelected), |tab_bar, tab| {
                let idx = tab_bar.size();
                tab_bar.push(idx, TabLabel::Text(tab.title.to_owned()))
            })
            .set_active_tab(&webengine.current_tab().0)
            .on_close(Message::TabClosed)
            .tab_width(Length::Shrink)
            .spacing(5.0)
            .padding(5.0);

        let new_tab = tooltip(
            Button::new(icon_to_text(Bootstrap::Plus))
                .on_press(Message::NewTab)
                .padding(5.0),
            "New Tab",
            tooltip::Position::Bottom,
        );

        row!(tab_bar, new_tab).into()
    }
}
