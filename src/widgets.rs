use crate::engines::{self, BrowserEngine};
#[cfg(feature = "webkit")]
use engines::ultralight::Ultralight;

use std::thread;
use std::time::Duration;

use crate::engines::{Commands, CommandsRecv, Engine};

// Configures the Browser Widget
#[derive(Debug, Clone)]
pub struct Config {
    pub start_page: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_page: String::from("https://google.com"),
        }
    }
}

// Holds the State of the Browser Widgets
#[derive(Clone)]
pub struct State {
    config: Config,
    #[cfg(feature = "webkit")]
    webengine: Ultralight,
}

impl State {
    pub fn new() -> Self {
        #[cfg(feature = "webkit")]
        let webengine = Ultralight::new();

        let config = Config::default();
        webengine.0.lock().unwrap().new_tab(&config.start_page);

        let bg_webengine = webengine.clone();
        thread::spawn(move || loop {
            bg_webengine.0.lock().unwrap().do_work();
            thread::sleep(Duration::from_millis(150));
        });

        State { config, webengine }
    }
}

pub use nav_bar::nav_bar;
pub mod nav_bar {
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
            if let Ok(webengine) = self.state.webengine.0.lock() {
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
            }
            None
        }

        fn view(&self, _state: &Self::State) -> Element<'_, Event, Theme> {
            row!(
                container(Button::new(text("<")).on_press(Event::Backward)).padding(2),
                container(Button::new(text(">")).on_press(Event::Forward)).padding(2),
                container(Button::new(text("H")).on_press(Event::Home)).padding(2),
                container(Button::new(text("R")).on_press(Event::Refresh)).padding(2),
                Space::new(Length::Fill, Length::Shrink),
                container(
                    text_input("https://site.com", &self.url.as_str())
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
}

pub use browser_view::browser_view;
pub mod browser_view {
    use super::{engines::create_empty_view, BrowserEngine, State};

    use iced::advanced::{
        self,
        graphics::core::event,
        layout, mouse,
        renderer::{self},
        widget::Tree,
        Clipboard, Layout, Shell, Widget,
    };
    use iced::event::Status;
    use iced::widget::image::{Handle, Image};
    use iced::{theme::Theme, Element, Event, Length, Rectangle, Size};

    // helper function to create browser view
    pub fn browser_view(state: &State) -> BrowserView {
        BrowserView::new(state)
    }

    pub struct BrowserView(State);

    impl BrowserView {
        pub fn new(state: &State) -> Self {
            Self(state.clone())
        }
    }

    impl<Message, Renderer> Widget<Message, Theme, Renderer> for BrowserView
    where
        Renderer: iced::advanced::image::Renderer<Handle = iced::advanced::image::Handle>,
    {
        fn size(&self) -> Size<Length> {
            Size {
                width: Length::Fill,
                height: Length::Fill,
            }
        }

        fn layout(
            &self,
            _tree: &mut Tree,
            _renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(limits.max())
        }

        fn draw(
            &self,
            tree: &Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            let mut webengine = self.0.webengine.0.lock().unwrap();
            // webengine.do_work();

            let (current_size, allowed_size) = (webengine.size(), layout.bounds().size());
            if current_size.0 != allowed_size.width as u32
                || current_size.1 != allowed_size.height as u32
            {
                webengine.resize(allowed_size.width as u32, allowed_size.height as u32);
                let image = match webengine.get_image() {
                    Some(image) => image,
                    None => create_empty_view(current_size.0, current_size.1),
                };
                webengine.get_tab_mut().unwrap().last_view = image;
            }

            if webengine.need_render() {
                webengine.render();
                let image = match webengine.get_image() {
                    Some(image) => image,
                    None => create_empty_view(current_size.0, current_size.1),
                };
                webengine.get_tab_mut().unwrap().last_view = image;
            };

            let image = &webengine.get_tab().unwrap().last_view;

            <Image<Handle> as Widget<Message, Theme, Renderer>>::draw(
                &image, tree, renderer, theme, style, layout, cursor, viewport,
            )
        }

        fn mouse_interaction(
            &self,
            _state: &Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            _viewport: &Rectangle,
            _renderer: &Renderer,
        ) -> mouse::Interaction {
            if cursor.is_over(layout.bounds()) {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Idle
            }
        }

        fn on_event(
            &mut self,
            _state: &mut Tree,
            event: Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            _renderer: &Renderer,
            _clipboard: &mut dyn Clipboard,
            _shell: &mut Shell<'_, Message>,
            _viewport: &Rectangle,
        ) -> event::Status {
            let mut webengine = self.0.webengine.0.lock().unwrap();

            match event {
                Event::Keyboard(keyboard_event) => webengine.handle_keyboard_event(keyboard_event),
                Event::Mouse(mouse_event) => {
                    if let Some(point) = cursor.position_in(layout.bounds()) {
                        webengine.handle_mouse_event(point, mouse_event)
                    } else {
                        Status::Ignored
                    }
                }
                _ => Status::Ignored,
            }
        }
    }

    impl<'a, Message, Renderer> From<BrowserView> for Element<'a, Message, Theme, Renderer>
    where
        Renderer: advanced::Renderer + advanced::image::Renderer<Handle = advanced::image::Handle>,
    {
        fn from(widget: BrowserView) -> Self {
            Self::new(widget)
        }
    }
}
