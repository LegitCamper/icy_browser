use std::sync::{Arc, Mutex};

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
pub struct State {
    config: Config,
    engine: Arc<Mutex<Engine>>,
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            config: self.config.clone(),
            engine: self.engine.clone(),
        }
    }
}

impl State {
    pub fn new() -> Self {
        let engine = Engine::new();
        let config = Config::default();
        engine.send(Commands::NewTab(config.start_page.clone()));

        Self {
            config,
            engine: Arc::new(Mutex::new(engine)),
        }
    }
}

pub use nav_bar::nav_bar;
pub mod nav_bar {

    use super::{Commands, State};

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
            let engine = self.state.engine.lock().unwrap();

            match event {
                Event::Backward => engine.send(Commands::GoBackward),
                Event::Forward => engine.send(Commands::GoForward),
                Event::Refresh => engine.send(Commands::Refresh),
                Event::Home => engine.send(Commands::GotoUrl(self.state.config.start_page.clone())),
                Event::UrlChanged(url) => self.url = url,
                Event::UrlPasted(url) => {
                    engine.send(Commands::GotoUrl(url.clone()));
                    self.url = url;
                }
                Event::UrlSubmitted => engine.send(Commands::GotoUrl(self.url.clone())),
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
    use super::{Commands, CommandsRecv, State};

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
            let engine = self.0.engine.lock().unwrap();

            let (w, h) = {
                let current_size = if let CommandsRecv::Size(w, h) = engine.recv(Commands::Size) {
                    (w, h)
                } else {
                    (800, 800)
                };
                let allowed_size = layout.bounds().size();
                if current_size.0 == allowed_size.width as u32
                    && current_size.1 == allowed_size.height as u32
                {
                    current_size
                } else {
                    engine.send(Commands::Resize(
                        allowed_size.width as u32,
                        allowed_size.height as u32,
                    ));
                    (allowed_size.width as u32, allowed_size.height as u32)
                }
            };

            engine.send(Commands::DoWork);
            if let CommandsRecv::NeedRender(need) = engine.recv(Commands::NeedRender) {
                need.then(|| engine.send(Commands::Render));
            }
            let handle = match engine.recv(Commands::PixelBuffer) {
                CommandsRecv::PixelBuffer(image) => {
                    let image = bgr_to_rgb(image);
                    Handle::from_pixels(w, h, image)
                }
                _ => {
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
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            _renderer: &Renderer,
            _clipboard: &mut dyn Clipboard,
            _shell: &mut Shell<'_, Message>,
            _viewport: &Rectangle,
        ) -> event::Status {
            let engine = self.0.engine.lock().unwrap();

            match event {
                Event::Keyboard(keyboard_event) => {
                    if let CommandsRecv::Keyboard(status) =
                        engine.recv(Commands::Keyboard(keyboard_event))
                    {
                        status
                    } else {
                        Status::Ignored
                    }
                }
                Event::Mouse(mouse_event) => {
                    if let Some(point) = cursor.position_in(layout.bounds()) {
                        if let CommandsRecv::Mouse(status) =
                            engine.recv(Commands::Mouse(point, mouse_event))
                        {
                            status
                        } else {
                            Status::Ignored
                        }
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

    fn bgr_to_rgb(image: Vec<u8>) -> Vec<u8> {
        image
            .chunks(4)
            .map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
            .flatten()
            .collect()
    }
}
