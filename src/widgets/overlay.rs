use iced::widget::{center, container, mouse_area, opaque, stack, text};
use iced::{Color, Element};

pub fn overlay<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    click_on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = text("fsdfsdf").center();
    // let content = command_window::CommandWindow::new();

    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(click_on_blur)
        )
    ]
    .into()
}

pub mod command_window {
    use iced::advanced::{layout, renderer::Style, widget::Tree, Layout, Widget};
    use iced::widget::canvas::{Cache, Path, Stroke};
    use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};

    #[derive(Clone)]
    enum Message {}

    pub struct CommandWindow {
        cache: Cache,
    }

    impl CommandWindow {
        pub fn new() -> Self {
            Self {
                cache: Cache::new(),
            }
        }
    }

    impl Widget<Message, Theme, Renderer> for CommandWindow {
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
            layout::Node::new(Size::new(
                limits.max().width / 60.,
                limits.max().height / 60.,
            ))
        }

        fn draw(
            &self,
            _tree: &Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &Style,
            layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            self.cache.draw(renderer, layout.bounds().size(), |frame| {
                frame.fill_rectangle(
                    Point::ORIGIN,
                    layout.bounds().size(),
                    Color::from_rgb(0.0, 0.2, 0.4),
                );

                frame.fill(
                    &Path::circle(frame.center(), frame.width().min(frame.height()) / 4.0),
                    Color::from_rgb(0.6, 0.8, 1.0),
                );

                frame.stroke(
                    &Path::line(
                        frame.center() + Vector::new(-250.0, 100.0),
                        frame.center() + Vector::new(250.0, -100.0),
                    ),
                    Stroke {
                        style: Color::WHITE.into(),
                        width: 50.0,
                        ..Default::default()
                    },
                );
            });
        }
    }

    impl<Message, Renderer: iced::advanced::Renderer> From<CommandWindow>
        for Element<'_, Message, Theme, Renderer>
    where
        CommandWindow: Widget<Message, Theme, Renderer>,
    {
        fn from(widget: CommandWindow) -> Self {
            Self::new(widget)
        }
    }
}
