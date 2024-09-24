// Custom view example with rainbow border

use iced::widget::{column, container};
use iced::{time, Border};
use iced::{Color, Element, Length, Settings, Subscription, Task, Theme};
use std::time::{Duration, Instant};

use icy_browser::{
    browser_view, get_fonts, hoverable, nav_bar, tab_bar, widgets, BrowserEngine, BrowserWidget,
    Ultralight,
};

fn main() -> iced::Result {
    let settings = Settings {
        fonts: get_fonts(),
        ..Default::default()
    };

    iced::application("Keyboard Driven Browser", Browser::update, Browser::view)
        .subscription(Browser::subscription)
        .settings(settings)
        .theme(|_| Theme::Dark)
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowserWidget(widgets::Message), // Passes messagees to Browser widgets
    Update,
    Tick,
}

#[derive(Debug, Clone)]
struct CustomWidgetState {
    border_colors: Vec<Color>,
    start_time: Instant,
}

struct Browser {
    widgets: BrowserWidget<Ultralight, CustomWidgetState>,
}

impl Default for Browser {
    fn default() -> Self {
        Self {
            widgets: BrowserWidget::new()
                .with_custom_view(
                    custom_view,
                    CustomWidgetState {
                        border_colors: vec![
                            Color::from_rgb(1.0, 0.0, 0.0),   // Red
                            Color::from_rgb(1.0, 0.5, 0.0),   // Orange
                            Color::from_rgb(1.0, 1.0, 0.0),   // Yellow
                            Color::from_rgb(0.0, 1.0, 0.0),   // Green
                            Color::from_rgb(0.0, 0.0, 1.0),   // Blue
                            Color::from_rgb(0.29, 0.0, 0.51), // Indigo
                            Color::from_rgb(0.56, 0.0, 1.0),  // Violet
                        ],
                        start_time: Instant::now(),
                    },
                )
                .build(),
        }
    }
}

impl Browser {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BrowserWidget(msg) => self.widgets.update(msg).map(Message::BrowserWidget),
            Message::Update => self.widgets.force_update().map(Message::BrowserWidget),
            Message::Tick => Task::none(), // Tick
        }
    }

    fn view(&self) -> Element<Message> {
        self.widgets.view().map(Message::BrowserWidget)
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            time::every(Duration::from_millis(10)).map(move |_| Message::Update),
            time::every(Duration::from_millis(16)).map(|_| Message::Tick),
        ])
    }
}

// fn custom_update<'a, Engine: BrowserEngine, CustomViewState>(
//     browser_widget: &'a BrowserWidget<Engine, CustomWidgetState>,
//     custom_view_state: &'a mut CustomViewState,
// ) -> Element<'a, icy_browser::Message> {

// }

fn custom_view<Engine: BrowserEngine>(
    browser_widget: &BrowserWidget<Engine, CustomWidgetState>,
    widget_state: CustomWidgetState,
) -> Element<icy_browser::Message> {
    let elapsed = widget_state.start_time.elapsed().as_secs_f32();
    let color_index = (elapsed * 2.0) as usize % widget_state.border_colors.len();
    let color = widget_state.border_colors[color_index];

    container(column![
        tab_bar(browser_widget.engine().get_tabs()),
        hoverable(nav_bar(&browser_widget.nav_bar_state))
            .on_focus_change(icy_browser::Message::UpdateUrl),
        browser_view(
            browser_widget.view_size,
            browser_widget.engine().get_tabs().get_current().get_view(),
            !browser_widget.show_overlay,
        ),
    ])
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .padding(20)
    .style(move |_theme| container::Style {
        border: Border::default().color(color).width(20),
        ..Default::default()
    })
    .into()
}
