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
use iced::{theme::Theme, Element, Event, Length, Point, Rectangle, Size};

use crate::ImageInfo;

pub fn browser_view<Message>(
    bounds: Size<u32>,
    image: &ImageInfo,
    send_bounds: Box<dyn Fn(Size<u32>) -> Message>,
    keyboard_event: Box<dyn Fn(iced::keyboard::Event) -> Message>,
    mouse_event: Box<dyn Fn(Point, iced::mouse::Event) -> Message>,
) -> BrowserView<Message> {
    BrowserView::new(bounds, image, send_bounds, keyboard_event, mouse_event)
}

pub struct BrowserView<Message> {
    bounds: Size<u32>,
    image: Image<Handle>,
    send_bounds: Box<dyn Fn(Size<u32>) -> Message>,
    keyboard_event: Box<dyn Fn(iced::keyboard::Event) -> Message>,
    mouse_event: Box<dyn Fn(Point, iced::mouse::Event) -> Message>,
}

impl<Message> BrowserView<Message> {
    pub fn new(
        bounds: Size<u32>,
        image: &ImageInfo,
        send_bounds: Box<dyn Fn(Size<u32>) -> Message>,
        keyboard_event: Box<dyn Fn(iced::keyboard::Event) -> Message>,
        mouse_event: Box<dyn Fn(Point, iced::mouse::Event) -> Message>,
    ) -> Self {
        Self {
            bounds,
            image: image.as_image(),
            send_bounds,
            keyboard_event,
            mouse_event,
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for BrowserView<Message>
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
        // Send updates back if bounds change
        // convert to u32 because Image takes u32
        let size = Size::new(layout.bounds().width as u32, layout.bounds().height as u32);
        if self.bounds != size {
            shell.publish((self.send_bounds)(size));
        }

        match event {
            Event::Keyboard(event) => {
                shell.publish((self.keyboard_event)(event));
                Status::Captured
            }
            Event::Mouse(event) => {
                if let Some(point) = cursor.position_in(layout.bounds()) {
                    shell.publish((self.mouse_event)(point, event));
                    Status::Captured
                } else {
                    Status::Ignored
                }
            }
            _ => Status::Ignored,
        }
    }
}

impl<'a, Message: 'a, Renderer> From<BrowserView<Message>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer + advanced::image::Renderer<Handle = advanced::image::Handle>,
{
    fn from(widget: BrowserView<Message>) -> Self {
        Self::new(widget)
    }
}
