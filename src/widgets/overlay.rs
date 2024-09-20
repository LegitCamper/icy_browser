use iced::widget::{center, container, mouse_area, opaque, stack, text};
use iced::{Color, Element};

pub fn overlay<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    click_on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = text("Overlay content here").center();

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
