use iced::{keyboard, mouse, Element, Point};

// helper function to create browser view
pub fn browser_view() -> BrowserView {
    BrowserView::new()
}

#[derive(Debug, Clone)]
pub enum Message {
    WidgetKeyboardEvent(keyboard::Event),
    WidgetMouseEvent(Point, mouse::Event),
    // GetBounds,
}

pub enum Action {
    // UpdateImage(Rectangle),
    SendKeyboardEvent(keyboard::Event),
    SendMouseEvent(Point, mouse::Event),
    None,
}

// Wrapper around BrowserView to keep consistency with other widgets
pub struct BrowserView;

impl BrowserView {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::WidgetKeyboardEvent(event) => Action::SendKeyboardEvent(event),
            Message::WidgetMouseEvent(point, event) => Action::SendMouseEvent(point, event),
            // Message::GetBounds => Action::UpdateImage(),
        };

        Action::None
    }

    pub fn view(&self) -> Element<Message> {
        browser_view::BrowserView::new(
            Box::new(Message::WidgetKeyboardEvent),
            Box::new(Message::WidgetMouseEvent),
        )
        .into()
    }
}

mod browser_view {
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

    use super::Message;
    use crate::create_empty_view;

    pub struct BrowserView<Message> {
        image: Image<Handle>,
        keyboard_event: Box<dyn Fn(iced::keyboard::Event) -> Message>,
        mouse_event: Box<dyn Fn(Point, iced::mouse::Event) -> Message>,
    }

    impl BrowserView<Message> {
        pub fn new(
            keyboard_event: Box<dyn Fn(iced::keyboard::Event) -> Message>,
            mouse_event: Box<dyn Fn(Point, iced::mouse::Event) -> Message>,
        ) -> Self {
            Self {
                image: create_empty_view(800, 800),
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
}
