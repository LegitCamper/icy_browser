use iced::widget::{row, text::LineHeight, text_input, tooltip, tooltip::Position, Button, Space};
use iced::{Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_on_focus_widget::hoverable;

use super::{BrowserEngine, State};

#[derive(Debug, Clone)]
pub enum Action {
    None,
}

#[derive(Debug, Clone)]
pub enum Message {
    Backward,
    Forward,
    Refresh,
    Home,
    UrlChanged(String),
    UrlPasted(String),
    UrlSubmitted,
    OnFocus,
    OnUnfocus,
}

// helper function to create navigation bar
pub fn nav_bar<Engine: BrowserEngine>(state: State<Engine>) -> NavBar<Engine> {
    NavBar::new(state)
}

// Simple navigation bar widget
pub struct NavBar<Engine: BrowserEngine> {
    search_focused: bool,
    state: State<Engine>,
    url: String,
}

impl<Engine: BrowserEngine> NavBar<Engine> {
    pub fn new(state: State<Engine>) -> Self {
        let (_, tab) = state.webengine.borrow().current_tab();

        Self {
            search_focused: false,
            state,
            url: tab.url,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        let webengine = self.state.webengine.borrow();

        match message {
            Message::Backward => webengine.go_back(),
            Message::Forward => webengine.go_forward(),
            Message::Refresh => webengine.refresh(),
            Message::Home => webengine.goto_url(&self.state.config.start_page),
            Message::UrlChanged(url) => self.url = url,
            Message::UrlPasted(url) => {
                webengine.goto_url(&url);
                self.url = url;
            }
            Message::UrlSubmitted => webengine.goto_url(&self.url),
            Message::OnFocus => self.search_focused = true,
            Message::OnUnfocus => self.search_focused = false,
        }

        if !self.search_focused {
            self.url = webengine.get_url().unwrap()
        }

        Action::None
    }

    pub fn view(&self) -> Element<Message> {
        let back = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ChevronBarLeft))
                .on_press(Message::Backward)
                .into(),
            "Go Back",
        );
        let forward = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ChevronBarRight))
                .on_press(Message::Forward)
                .into(),
            "Go Forward",
        );
        let home = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::HouseDoor))
                .on_press(Message::Home)
                .into(),
            "Go Home",
        );
        let refresh = tooltip_helper(
            Button::new(icon_to_text(Bootstrap::ArrowCounterclockwise))
                .on_press(Message::Refresh)
                .into(),
            "Refresh",
        );
        let space = Space::new(Length::Fill, Length::Shrink);
        let space2 = Space::new(Length::Fill, Length::Shrink);
        let search = hoverable(
            text_input("https://site.com", &self.url)
                .on_input(Message::UrlChanged)
                .on_paste(Message::UrlPasted)
                .on_submit(Message::UrlSubmitted)
                .line_height(LineHeight::Relative(2.0))
                .into(),
        )
        .on_focus(Message::OnFocus)
        .on_unfocus(Message::OnUnfocus);

        row!(back, forward, home, refresh, space, search, space2).into()
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
