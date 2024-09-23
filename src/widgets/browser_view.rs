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

use super::Message;
use crate::ImageInfo;

pub fn browser_view(bounds: Size<u32>, image: &ImageInfo, can_type: bool) -> BrowserView {
    BrowserView::new(bounds, image, can_type)
}

pub struct BrowserView {
    bounds: Size<u32>,
    image: Image<Handle>,
    can_interact: bool, // wheather or not to allow typing - useful when overlay enabled
}

impl BrowserView {
    pub fn new(bounds: Size<u32>, image: &ImageInfo, can_type: bool) -> Self {
        Self {
            bounds,
            image: image.as_image(),
            can_interact: can_type,
        }
    }
}

impl<Renderer> Widget<Message, Theme, Renderer> for BrowserView
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
        <Image<Handle> as Widget<Message, Theme, Renderer>>::draw(
            &self.image,
            tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        if self.can_interact {
            // Send updates back if bounds change
            // convert to u32 because Image takes u32
            let size = Size::new(layout.bounds().width as u32, layout.bounds().height as u32);
            if self.bounds != size {
                shell.publish(Message::UpdateViewSize(size));
            }

            match event {
                Event::Keyboard(event) => {
                    shell.publish(Message::SendKeyboardEvent(Some(event)));
                }
                Event::Mouse(event) => {
                    if let Some(point) = cursor.position_in(layout.bounds()) {
                        shell.publish(Message::SendMouseEvent(point, Some(event)));
                    }
                }
                _ => (),
            }
        }
        Status::Ignored
    }
}

impl<'a, Message: 'a, Renderer> From<BrowserView> for Element<'a, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer + advanced::image::Renderer<Handle = advanced::image::Handle>,
    BrowserView: Widget<Message, Theme, Renderer>,
{
    fn from(widget: BrowserView) -> Self {
        Self::new(widget)
    }
}
