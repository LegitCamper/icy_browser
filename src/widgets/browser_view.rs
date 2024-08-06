use crate::create_image;

use super::{BrowserEngine, State};

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
        let mut webengine = self.0.webengine.lock().unwrap();

        let (current_size, allowed_size) = (webengine.size(), layout.bounds().size());
        if current_size.0 != allowed_size.width as u32
            || current_size.1 != allowed_size.height as u32
        {
            webengine.resize(allowed_size.width as u32, allowed_size.height as u32);
        }

        let image_data = webengine.pixel_buffer().unwrap();
        let image = create_image(image_data, current_size.0, current_size.1, true);

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
            self.0.webengine.lock().unwrap().get_cursor()
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
        let mut webengine = self.0.webengine.lock().unwrap();
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
