use super::{BrowserEngine, State};

use iced::widget::{component, row, tooltip, Button, Component};
use iced::{self, theme::Theme, Element, Length, Size};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_aw::{TabBar as TB, TabLabel};

#[derive(Debug, Clone)]
pub enum Event {
    TabSelected(usize),
    TabClosed(usize),
    NewTab,
}

// helper function to create navigation bar
pub fn tab_bar(state: &State) -> TabBar {
    TabBar::new(state)
}

// Simple navigation bar widget
pub struct TabBar {
    state: State,
}

impl TabBar {
    pub fn new(state: &State) -> Self {
        let state = state.clone();
        Self { state }
    }
}

impl<Message> Component<Message> for TabBar {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        let mut webengine = self.state.webengine.lock().unwrap();

        match event {
            Event::TabSelected(index) => webengine.goto_tab(index as u32).unwrap(),
            Event::TabClosed(index) => webengine.close_tab(index as u32),
            Event::NewTab => webengine.new_tab(&self.state.config.start_page),
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Event, Theme> {
        let webengine = self.state.webengine.lock().unwrap();

        let tab_bar = webengine
            .get_tabs()
            .iter()
            .fold(TB::new(Event::TabSelected), |tab_bar, tab| {
                let idx = tab_bar.size();
                tab_bar.push(idx, TabLabel::Text(tab.title.to_owned()))
            })
            .set_active_tab(&webengine.current_tab().0)
            .on_close(Event::TabClosed)
            .tab_width(Length::Shrink)
            .spacing(5.0)
            .padding(5.0);

        let new_tab = tooltip(
            Button::new(icon_to_text(Bootstrap::Plus))
                .on_press(Event::NewTab)
                .padding(5.0),
            "New Tab",
            tooltip::Position::Bottom,
        );

        row!(tab_bar, new_tab).into()
    }

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }
}
impl<'a, Message: 'a> From<TabBar> for Element<'a, Message> {
    fn from(widget: TabBar) -> Self {
        component(widget)
    }
}
