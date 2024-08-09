use super::{BrowserEngine, State};

use iced::widget::{
    component, row, text::LineHeight, text_input, tooltip, tooltip::Position, Button, Component,
    Space,
};
use iced::{theme::Theme, Element, Length, Size};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};

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
pub fn nav_bar<Engine: BrowserEngine>(state: State<Engine>) -> NavBar<Engine> {
    NavBar::new(state)
}

// Simple navigation bar widget
pub struct NavBar<Engine: BrowserEngine> {
    state: State<Engine>,
    url: String,
}

impl<Engine: BrowserEngine> NavBar<Engine> {
    pub fn new(state: State<Engine>) -> Self {
        let (_, tab) = state.webengine.borrow().current_tab();

        Self {
            state,
            url: tab.url,
        }
    }
}

impl<Message, Engine: BrowserEngine> Component<Message> for NavBar<Engine> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        let webengine = self.state.webengine.borrow();

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
        let back = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ChevronBarLeft))
                .on_press(Event::Backward)
                .into(),
            "Go Back",
        );
        let forward = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ChevronBarRight))
                .on_press(Event::Forward)
                .into(),
            "Go Forward",
        );
        let home = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::HouseDoor))
                .on_press(Event::Home)
                .into(),
            "Go Home",
        );
        let refresh = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ArrowCounterclockwise))
                .on_press(Event::Refresh)
                .into(),
            "Refresh",
        );
        let space = Space::new(Length::Fill, Length::Shrink);
        let space2 = Space::new(Length::Fill, Length::Shrink);
        let search = text_input("https://site.com", &self.url)
            .on_input(Event::UrlChanged)
            .on_paste(Event::UrlPasted)
            .on_submit(Event::UrlSubmitted)
            .line_height(LineHeight::Relative(2.0));

        row!(back, forward, home, refresh, space, search, space2).into()
    }

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }
}
impl<'a, Message: 'a, Engine: BrowserEngine + 'a> From<NavBar<Engine>> for Element<'a, Message> {
    fn from(widget: NavBar<Engine>) -> Self {
        component(widget)
    }
}

fn tooltip_helper<'a, Message: 'a>(
    element: Element<'a, Message>,
    tooltip_str: &'a str,
) -> Element<'a, Message> {
    tooltip(element, tooltip_str, Position::Bottom)
        .padding(5)
        .into()
}
