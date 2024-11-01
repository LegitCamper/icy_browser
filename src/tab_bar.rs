use iced::widget::{container, row, tooltip, Button};
use iced::{self, Element, Length};
use iced_aw::core::icons::bootstrap::{icon_to_text, Bootstrap};
use iced_aw::{TabBar as TB, TabLabel};
use iced_webview::ViewId;

/// Creates Tab bar widget for basic webview
pub fn tab_bar_basic<Message: 'static + Clone>(
    tabs: Vec<String>,
    active_tab: u32,
    on_tab_select: Box<dyn Fn(u32) -> Message>,
    on_close_tab: Box<dyn Fn(u32) -> Message>,
    on_create_tab: Message,
) -> Element<'static, Message> {
    let active_tab = (
        active_tab,
        tabs.get(active_tab as usize)
            .expect("That active tab index does not exist")
            .clone(),
    );
    row![
        tabs.iter()
            .fold(
                TB::new(move |(index, _)| (on_tab_select)(index)),
                |tab_bar, title| {
                    let id = tab_bar.size();
                    let title = if title.is_empty() {
                        String::from("New Tab")
                    } else {
                        title.clone().to_string()
                    };
                    tab_bar.push((id as u32, title.clone()), TabLabel::Text(title))
                },
            )
            .set_active_tab(&active_tab)
            .on_close(move |(id, _)| (on_close_tab)(id))
            .tab_width(Length::Shrink)
            .spacing(5.0)
            .padding(5.0),
        container(tooltip(
            Button::new(icon_to_text(Bootstrap::Plus))
                .on_press(on_create_tab)
                .padding(5.0),
            "New Tab",
            tooltip::Position::Bottom,
        ))
        .height(Length::Shrink),
    ]
    .into()
}

/// Creates Tab bar widget for advanced webview
pub fn tab_bar_advanced<Message: 'static + Clone>(
    tabs: Vec<(ViewId, String)>,
    active_tab: ViewId,
    on_tab_select: Box<dyn Fn(ViewId) -> Message>,
    on_close_tab: Box<dyn Fn(ViewId) -> Message>,
    on_create_tab: Message,
) -> Element<'static, Message> {
    let active_tab = tabs
        .iter()
        .find(|(id, _)| *id == active_tab)
        .expect("Failed to find that tab id in the given tabs");
    row![
        tabs.iter()
            .fold(
                TB::new(move |(id, _)| (on_tab_select)(id)),
                |tab_bar, (_, title)| {
                    let id = tab_bar.size();
                    let title = if title.is_empty() {
                        String::from("New Tab")
                    } else {
                        title.clone().to_string()
                    };
                    tab_bar.push((id, title.clone()), TabLabel::Text(title))
                },
            )
            .set_active_tab(active_tab)
            .on_close(move |(id, _)| (on_close_tab)(id))
            .tab_width(Length::Shrink)
            .spacing(5.0)
            .padding(5.0),
        container(tooltip(
            Button::new(icon_to_text(Bootstrap::Plus))
                .on_press(on_create_tab)
                .padding(5.0),
            "New Tab",
            tooltip::Position::Bottom,
        ))
        .height(Length::Fill),
    ]
    .into()
}
