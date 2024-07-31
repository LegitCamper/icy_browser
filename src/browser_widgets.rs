use crate::browser_engines::{self, BrowserEngine};
#[cfg(feature = "webkit")]
use browser_engines::ultralight::Ultralight;

use iced::{
    advanced::{
        graphics::core::event,
        layout, mouse,
        renderer::{self},
        widget::Tree,
        Clipboard, Layout, Shell, Widget,
    },
    theme::Theme,
    widget::{
        component, container,
        image::{Handle, Image},
        row, text,
        text::LineHeight,
        Button, Component, Space,
    },
    Element, Event, Length, Rectangle, Size,
};
use std::sync::{Arc, Mutex};

// Configures the Browser Widget
#[derive(Debug, Clone)]
pub struct Config {
    start_page: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_page: String::from("https://google.com"),
        }
    }
}

// Holds the State of the Browser Widgets
pub struct State {
    config: Config,
    #[cfg(feature = "webkit")]
    webengine: Ultralight,
}

impl State {
    pub fn new() -> Arc<Mutex<Self>> {
        #[cfg(feature = "webkit")]
        // size does not matter since it is resized to fit window
        let mut webengine = Ultralight::new(1600, 1600);

        let config = Config::default();
        webengine.new_tab(&config.start_page);

        Arc::new(Mutex::new(Self { config, webengine }))
    }

    pub fn do_work(&self) {
        self.webengine.do_work()
    }
}

pub use nav_bar::nav_bar;
pub mod nav_bar {
    use iced::widget::text_input;

    use super::*;

    #[derive(Debug, Clone)]
    pub enum Event {
        Backward,
        Forward,
        Refresh,
        Home,
        UrlEdited(String),
        UrlChanged(String),
    }

    // helper function to create navigation bar
    pub fn nav_bar(state: Arc<Mutex<State>>) -> NavBar {
        NavBar::new(state)
    }

    // Simple navigation bar widget
    pub struct NavBar(Arc<Mutex<State>>, String);

    impl NavBar {
        pub fn new(state: Arc<Mutex<State>>) -> Self {
            let url = state.lock().unwrap().config.start_page.clone();
            Self(state, url)
        }
    }

    impl<Message> Component<Message> for NavBar {
        type State = ();
        type Event = Event;

        fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
            match event {
                Event::Backward => self.0.lock().unwrap().webengine.go_back(),
                Event::Forward => self.0.lock().unwrap().webengine.go_forward(),
                Event::Refresh => self.0.lock().unwrap().webengine.refresh(),
                Event::Home => {
                    let start_page = self.0.lock().unwrap().config.start_page.clone();
                    self.0.lock().unwrap().webengine.goto_url(&start_page)
                }
                Event::UrlEdited(url) => self.1 = url,
                Event::UrlChanged(url) => self.0.lock().unwrap().webengine.goto_url(&url),
            }
            None
        }

        fn view(&self, _state: &Self::State) -> Element<'_, Event, Theme> {
            let url = match self.0.lock().unwrap().webengine.get_url() {
                Some(url) => url,
                None => "Homepage".to_string(),
            };
            let bar = row!(
                container(Button::new(text("<")).on_press(Event::Backward)).padding(2),
                container(Button::new(text(">")).on_press(Event::Forward)).padding(2),
                container(Button::new(text("H")).on_press(Event::Home)).padding(2),
                container(Button::new(text("R")).on_press(Event::Refresh)).padding(2),
                Space::new(Length::Fill, Length::Shrink),
                container(
                    text_input("https://site.com", &url)
                        .on_input(Event::UrlEdited)
                        .on_submit(Event::UrlChanged(self.1.clone()))
                        .line_height(LineHeight::Relative(2.0))
                )
                .padding(2)
                .center_x()
                .center_y(),
                Space::new(Length::Fill, Length::Shrink),
            );
            bar.into()
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
    use super::*;

    // helper function to create browser view
    pub fn browser_view(state: Arc<Mutex<State>>) -> BrowserView {
        BrowserView::new(state)
    }

    pub struct BrowserView(Arc<Mutex<State>>);

    impl BrowserView {
        pub fn new(state: Arc<Mutex<State>>) -> Self {
            Self(state)
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
            let mut state = self.0.lock().unwrap();

            let (w, h) = {
                let current_size = state.webengine.size();
                let allowed_size = layout.bounds().size();
                if current_size.0 == allowed_size.width as u32
                    && current_size.1 == allowed_size.height as u32
                {
                    current_size
                } else {
                    state
                        .webengine
                        .resize(allowed_size.width as u32, allowed_size.height as u32);
                    (allowed_size.width as u32, allowed_size.height as u32)
                }
            };

            state.webengine.render();
            let handle = match state.webengine.pixel_buffer() {
                Some(image) => {
                    let image = bgr_to_rgb(image);
                    Handle::from_pixels(w, h, image)
                }
                None => {
                    let palatte = theme.palette().background;
                    let mut image: Vec<u8> = Vec::new();
                    for _ in 0..((w * h) / 4) {
                        image.push(palatte.r as u8);
                        image.push(palatte.g as u8);
                        image.push(palatte.b as u8);
                        image.push(palatte.a as u8);
                    }
                    Handle::from_pixels(w, h, image)
                }
            };
            // Image::<Handle>::new(handle).draw(tree, renderer, theme, style, layout, cursor, viewport)
            <Image<Handle> as Widget<Message, Theme, Renderer>>::draw(
                &Image::<Handle>::new(handle),
                tree,
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
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
            _layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _renderer: &Renderer,
            _clipboard: &mut dyn Clipboard,
            _shell: &mut Shell<'_, Message>,
            _viewport: &Rectangle,
        ) -> event::Status {
            match event {
                Event::Keyboard(keyboard_event) => self
                    .0
                    .lock()
                    .unwrap()
                    .webengine
                    .handle_keyboard_event(keyboard_event),
                Event::Mouse(mouse_event) => self
                    .0
                    .lock()
                    .unwrap()
                    .webengine
                    .handle_mouse_event(mouse_event),

                // ignore all unhandled events
                _ => event::Status::Ignored,
            }
        }
    }

    impl<'a, Message, Renderer> From<BrowserView> for Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer
            + iced::advanced::image::Renderer<Handle = iced::advanced::image::Handle>,
    {
        fn from(widget: BrowserView) -> Self {
            Self::new(widget)
        }
    }

    fn bgr_to_rgb(image: Vec<u8>) -> Vec<u8> {
        image
            .chunks(4)
            .map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
            .flatten()
            .collect()
    }
}
