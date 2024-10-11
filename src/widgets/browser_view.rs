use iced::{Element, Size};
use iced_event_wrapper::wrapper;

use super::Message;
use crate::ImageInfo;

pub fn browser_view(image: &ImageInfo) -> Element<Message> {
    wrapper(image.as_image())
        .always_ignore_events()
        .on_keyboard_event(|event| Message::SendKeyboardEvent(Some(event)))
        .on_mouse_event(|event, point| Message::SendMouseEvent(point, Some(event)))
        .on_bounds_change(|bounds: Size| {
            Message::UpdateViewSize(Size::new(bounds.width as u32, bounds.height as u32))
        })
        .into()
}
