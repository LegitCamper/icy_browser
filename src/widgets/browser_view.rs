use iced::Element;

use super::{BrowserEngine, State};

// helper function to create browser view
pub fn browser_view<Engine: BrowserEngine>(state: State<Engine>) -> BrowserView<Engine> {
    BrowserView::new(state)
}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Clone)]
pub enum Action {
    None,
}

// Wrapper around BrowserView to keep consistency with other widgets
pub struct BrowserView<Engine: BrowserEngine>(State<Engine>);

impl<Engine: BrowserEngine> BrowserView<Engine> {
    pub fn new(state: State<Engine>) -> Self {
        Self(state)
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {}
        Action::None
    }

    pub fn view(&self) -> Element<Message> {
        browser_view::BrowserView::new(self.0.clone()).into()
    }
}

mod browser_view {
    use super::{BrowserEngine, State};
    use crate::create_image;

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

    pub struct BrowserView<Engine: BrowserEngine>(State<Engine>);

    impl<Engine: BrowserEngine> BrowserView<Engine> {
        pub fn new(state: State<Engine>) -> Self {
            Self(state)
        }
    }

    impl<Message, Renderer, Engine> Widget<Message, Theme, Renderer> for BrowserView<Engine>
    where
        Renderer: iced::advanced::image::Renderer<Handle = iced::advanced::image::Handle>,
        Engine: BrowserEngine,
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
            let mut webengine = self.0.webengine.borrow_mut();

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
                self.0.webengine.borrow().get_cursor()
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
            let mut webengine = self.0.webengine.borrow_mut();

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

    impl<'a, Message, Renderer, Engine: BrowserEngine + 'a> From<BrowserView<Engine>>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: advanced::Renderer + advanced::image::Renderer<Handle = advanced::image::Handle>,
    {
        fn from(widget: BrowserView<Engine>) -> Self {
            Self::new(widget)
        }
    }
}
