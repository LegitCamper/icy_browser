#![feature(ascii_char)]

use iced::{
    executor,
    widget::{
        column, container,
        image::{Handle, Image},
        text, Container,
    },
    Application, Command, Settings, Subscription, Theme,
};
use std::time::Duration;
use ul_next::{
    config::Config,
    platform::{self, LogLevel, Logger},
    renderer::Renderer,
    surface::Surface,
    view::{View, ViewConfig},
};

pub mod ui;
use ui::nav_bar;
pub mod browser_engine;

struct MyLogger;

impl Logger for MyLogger {
    fn log_message(&mut self, log_level: LogLevel, message: String) {
        println!("{:?}: {}", log_level, message);
    }
}

struct BrowserView {
    renderer: Renderer,
    view: View,
    surface: Surface,
    surface_height: u32,
    surface_width: u32,
    image: Option<Vec<u8>>,
}

impl BrowserView {
    pub fn new() -> Self {
        let config = Config::start().build().unwrap();
        platform::enable_platform_fontloader();
        // TODO: this should change to ~/.rust-browser
        platform::enable_platform_filesystem(".").unwrap();
        platform::set_logger(MyLogger);
        // TODO: this should change to ~/.rust-browser
        platform::enable_default_logger("./log.txt").unwrap();
        let renderer = Renderer::create(config).unwrap();
        let view_config = ViewConfig::start()
            .initial_device_scale(2.0)
            .font_family_standard("Arial")
            .is_accelerated(false)
            .build()
            .unwrap();

        let view = renderer
            .create_view(1600, 1600, &view_config, None)
            .unwrap();

        view.load_url("https://google.com").unwrap();

        let surface = view.surface().unwrap();

        let width = surface.width();
        let height = surface.height();
        let bytes_per_pixel = surface.row_bytes() / width;
        // RGBA
        assert!(bytes_per_pixel == 4);

        Self {
            renderer,
            view,
            surface,
            surface_height: height,
            surface_width: width,
            image: None,
        }
    }

    pub fn update(&mut self) {
        self.renderer.update();

        self.renderer.render();
        // Get the raw pixels of the surface
        if let Some(pixels_data) = self.surface.lock_pixels() {
            let mut vec = Vec::new();
            vec.extend_from_slice(&pixels_data);
            self.image = Some(vec);
        }
    }
}

struct Browser {
    view: BrowserView,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Update,
}

impl Application for Browser {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                view: BrowserView::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Test Browser")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Update => {
                self.view.update();
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let update = iced::time::every(Duration::from_millis(100)).map(move |_| Message::Update);

        update
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let bar = match self.view.view.url() {
            Ok(url) => nav_bar(&url),
            Err(_) => nav_bar("HomePage"),
        };
        let content: Container<'_, Message> = match self.view.image.clone() {
            Some(image) => {
                let img_handle =
                    Handle::from_pixels(self.view.surface_width, self.view.surface_height, image);
                container(Image::new(img_handle))
            }
            None => container(text("loading")),
        };

        let ui = column!(bar, content);

        container(ui)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    Browser::run(Settings::default())
}
