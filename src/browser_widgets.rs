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
    keyboard::{self, key::Named},
    theme::Theme,
    widget::{
        component,
        image::{Handle, Image},
        row, text, Button, Component, Row,
    },
    Border, Color, Element, Event, Length, Rectangle, Shadow, Size,
};
use std::sync::{Arc, Mutex};

// Configures the Browser Widget
#[derive(Debug, Clone)]
pub struct Config {
    start_page: String,
    enable_nav_bar: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_page: String::from("https://google.com"),
            enable_nav_bar: true,
        }
    }
}

// Simple type alias for BrowserState
pub type State = Arc<Mutex<BrowserState>>;

// Holds the State of the Browser Widget
pub struct BrowserState {
    config: Config,
    #[cfg(feature = "webkit")]
    webengine: Ultralight,
}

impl BrowserState {
    pub fn new() -> Arc<Mutex<Self>> {
        #[cfg(feature = "webkit")]
        // size does not matter since it is resized to fit window
        let mut webengine = Ultralight::new(1600, 1600);

        let config = Config::default();
        webengine.new_tab(&config.start_page);

        Arc::new(Mutex::new(Self { config, webengine }))
    }

    pub fn do_work(&self) {
        self.webengine.update()
    }
}

// Simple customizable navigation bar widget
fn nav_bar<'a, Message: std::clone::Clone + 'a>(
    url: &str,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let bar: Row<Message> = row!(
        Button::new(text("<")),
        Button::new(text(">")),
        Button::new(text("R")),
        text(url),
    );
    Element::new(bar)
}

pub struct Browser {
    state: Arc<Mutex<BrowserState>>,
}

impl Browser {
    pub fn new(state: Arc<Mutex<BrowserState>>) -> Self {
        Self { state }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Browser
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
        let mut state = self.state.lock().unwrap();

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
            Some(image) => Handle::from_pixels(w, h, image),
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
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            Event::Keyboard(keyboard_event) => self
                .state
                .lock()
                .unwrap()
                .webengine
                .handle_keyboard_event(keyboard_event),
            Event::Mouse(mouse_event) => {
                if let Some(cursor_pos) = cursor.position() {
                    self.state
                        .lock()
                        .unwrap()
                        .webengine
                        .handle_mouse_event(mouse_event, cursor_pos)
                } else {
                    event::Status::Ignored
                }
            }
            // ignore all unhandled events
            _ => event::Status::Ignored,
        }
    }
}

impl<'a, Message, Renderer> From<Browser> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer
        + iced::advanced::image::Renderer<Handle = iced::advanced::image::Handle>,
{
    fn from(widget: Browser) -> Self {
        Self::new(widget)
    }
}
