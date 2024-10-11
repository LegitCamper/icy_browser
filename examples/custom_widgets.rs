// Custom view example with rainbow border

use iced::widget::container;
use iced::{time, Border};
use iced::{Color, Element, Length, Settings, Subscription, Task, Theme};
use std::time::{Duration, Instant};

use icy_browser::{get_fonts, widgets, IcyBrowser, Ultralight};

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
    IcyBrowser(widgets::Message),
    Update,
    Tick,
}

#[derive(Debug, Clone)]
struct CustomWidgetState {
    border_colors: Vec<Color>,
    start_time: Instant,
}

struct Browser {
    icy_browser: IcyBrowser<Ultralight>,
    custom_widget_state: CustomWidgetState,
}

impl Default for Browser {
    fn default() -> Self {
        Self {
            icy_browser: IcyBrowser::new().with_tab_bar().with_nav_bar().build(),
            custom_widget_state: CustomWidgetState {
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
        }
    }
}

impl Browser {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IcyBrowser(msg) => self.icy_browser.update(msg).map(Message::IcyBrowser),
            Message::Update => self.icy_browser.force_update().map(Message::IcyBrowser),
            Message::Tick => Task::none(), // Tick
        }
    }

    fn view(&self) -> Element<Message> {
        let elapsed = self.custom_widget_state.start_time.elapsed().as_secs_f32();
        let color_index = (elapsed * 2.0) as usize % self.custom_widget_state.border_colors.len();
        let color = self.custom_widget_state.border_colors[color_index];

        container(self.icy_browser.view().map(Message::IcyBrowser))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(20)
            .style(move |_theme| container::Style {
                border: Border::default().color(color).width(20),
                ..Default::default()
            })
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            time::every(Duration::from_millis(10)).map(move |_| Message::Update),
            time::every(Duration::from_millis(16)).map(|_| Message::Tick),
        ])
    }
}
