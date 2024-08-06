use super::{BrowserEngine, State};

use iced::widget::text_input;
use iced::{
    theme::Theme,
    widget::{component, container, row, text, text::LineHeight, Button, Component, Space},
    Element, Length, Size,
};

#[derive(Debug, Clone)]
pub enum Event {
    Backward,
    Forward,
    Refresh,
    Home,
    UrlChanged(String),
    UrlPasted(String),
    UrlSubmitted,
}

// helper function to create navigation bar
pub fn nav_bar(state: &State) -> Option<NavBar> {
    NavBar::new(state)
}

// Simple navigation bar widget
pub struct NavBar {
    state: State,
    url: String,
}

impl NavBar {
    pub fn new(state: &State) -> Option<Self> {
        let state = state.clone();
        let url = state.config.start_page.clone();
        Some(Self { state, url })
    }
}

impl<Message> Component<Message> for NavBar {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        let webengine = self.state.webengine.lock().unwrap();
        match event {
            Event::Backward => webengine.go_back(),
            Event::Forward => webengine.go_forward(),
            Event::Refresh => webengine.refresh(),
            Event::Home => webengine.goto_url(&self.state.config.start_page),
            Event::UrlChanged(url) => self.url = url,
            Event::UrlPasted(url) => {
                webengine.goto_url(&url);
                self.url = url;
            }
            Event::UrlSubmitted => webengine.goto_url(&self.url),
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Event, Theme> {
        row!(
            container(row!(
                container(Button::new(text("<")).on_press(Event::Backward)).padding(2),
                container(Button::new(text(">")).on_press(Event::Forward)).padding(2),
                container(Button::new(text("H")).on_press(Event::Home)).padding(2),
                container(Button::new(text("R")).on_press(Event::Refresh)).padding(2)
            ))
            .center_y()
            .center_x(),
            Space::new(Length::Fill, Length::Shrink),
            container(
                text_input("https://site.com", &self.url.as_str())
                    // .on
                    .on_input(Event::UrlChanged)
                    .on_paste(Event::UrlPasted)
                    .on_submit(Event::UrlSubmitted)
                    .line_height(LineHeight::Relative(2.0))
            )
            .padding(2)
            .center_x()
            .center_y(),
            Space::new(Length::Fill, Length::Shrink),
        )
        .into()
    }

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }
}

impl<'a, Message: 'a> From<NavBar> for Element<'a, Message> {
    fn from(nav_bar: NavBar) -> Self {
        component(nav_bar)
    }
}
