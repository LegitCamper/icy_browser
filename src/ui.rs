use iced::{
    theme::Theme,
    widget::{row, text, Button, Row},
    Element, Renderer,
};

pub fn nav_bar<'a, Message: std::clone::Clone + 'a>(
    url: &str,
) -> Element<'a, Message, Theme, Renderer> {
    let bar: Row<Message> = row!(
        Button::new(text("<")),
        Button::new(text(">")),
        Button::new(text("R")),
        text(url),
    );
    Element::new(bar)
}

// pub struct BrowserView {}
// impl Component<Message, Renderer> for BrowserView {}
