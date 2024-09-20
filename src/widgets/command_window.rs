use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{center, container, mouse_area, opaque, stack};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme, Vector};

pub fn command_window<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    click_on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = Canvas::new(CommandWindow)
        .width(Length::Fill)
        .height(Length::Fill);

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

struct CommandWindow;

impl<Message> Program<Message> for CommandWindow {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb(0.0, 0.2, 0.4));

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

        vec![frame.into_geometry()]
    }
}
