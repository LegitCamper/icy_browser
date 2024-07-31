#[allow(unused)]
pub trait BrowserEngine {
    fn new(width: u32, height: u32) -> Self;

    fn do_work(&self);
    fn render(&self);
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, width: u32, height: u32);
    fn pixel_buffer(&mut self) -> Option<Vec<u8>>;

    fn get_url(&self) -> Option<String>;
    fn goto_url(&self, url: &str);
    fn has_loaded(&self) -> bool;
    fn new_tab(&mut self, url: &str);
    fn goto_tab(&mut self, url: &str) -> Option<()>;

    fn refresh(&self);
    fn go_forward(&self);
    fn go_back(&self);

    fn scroll(&self, delta: iced::mouse::ScrollDelta) -> iced::event::Status;
    fn handle_keyboard_event(&self, event: iced::keyboard::Event) -> iced::event::Status;
    fn handle_mouse_event(&mut self, event: iced::mouse::Event) -> iced::event::Status;
}

#[cfg(feature = "webkit")]
#[allow(dead_code)]
pub mod ultralight {
    use std::collections::HashMap;

    use iced::mouse::ScrollDelta;
    use ul_next::{
        config::Config,
        event::{MouseEvent, ScrollEvent},
        platform::{self, LogLevel, Logger},
        renderer::Renderer,
        view::{View, ViewConfig},
        Surface,
    };

    struct UlLogger;
    impl Logger for UlLogger {
        fn log_message(&mut self, log_level: LogLevel, message: String) {
            println!("{:?}: {}", log_level, message);
        }
    }

    pub struct Tab {
        url: String,
        view: View,
        surface: Surface,
        image: Option<Vec<u8>>,
    }

    pub struct Ultralight {
        renderer: Renderer,
        view_config: ViewConfig,
        width: u32,
        height: u32,
        mouse_loc: Option<iced::Point>,
        current_tab: Option<String>,
        tabs: HashMap<String, Tab>,
    }

    impl Ultralight {
        pub fn new(width: u32, height: u32) -> Self {
            let config = Config::start().build().unwrap();
            platform::enable_platform_fontloader();
            // TODO: this should change to ~/.rust-browser
            platform::enable_platform_filesystem(".").unwrap();
            platform::set_logger(UlLogger);
            // TODO: this should change to ~/.rust-browser
            platform::enable_default_logger("./log.txt").unwrap();
            let renderer = Renderer::create(config).unwrap();
            let view_config = ViewConfig::start()
                .initial_device_scale(1.0)
                .font_family_standard("Arial")
                .is_accelerated(false)
                .build()
                .unwrap();

            Self {
                renderer,
                view_config,
                width,
                height,
                mouse_loc: None,
                current_tab: None,
                tabs: HashMap::new(),
            }
        }

        fn get_tab(&mut self) -> Option<&Tab> {
            self.tabs.get(&self.current_tab.clone()?)
        }
    }

    impl super::BrowserEngine for Ultralight {
        fn new(width: u32, height: u32) -> Self {
            Self::new(width, height)
        }

        fn do_work(&self) {
            self.renderer.update()
        }

        fn render(&self) {
            self.renderer.render()
        }

        fn size(&self) -> (u32, u32) {
            (self.width, self.height)
        }

        fn resize(&mut self, width: u32, height: u32) {
            (self.width, self.height) = (width, height);
            self.tabs.iter().for_each(|tab| {
                tab.1.view.resize(width, height);
                tab.1.surface.resize(width, height);
            })
        }

        fn pixel_buffer(&mut self) -> Option<Vec<u8>> {
            // Get the raw pixels of the surface
            if let Some(pixels_data) = self
                .tabs
                .get_mut(&self.current_tab.clone()?)?
                .surface
                .lock_pixels()
            {
                let mut vec = Vec::new();
                vec.extend_from_slice(&pixels_data);
                Some(vec)
            } else {
                None
            }
        }

        fn get_url(&self) -> Option<String> {
            Some(self.current_tab.clone()?)
        }

        fn goto_url(&self, url: &str) {
            self.tabs
                .get(&self.current_tab.clone().unwrap())
                .unwrap()
                .view
                .load_url(url)
                .unwrap();
        }

        fn has_loaded(&self) -> bool {
            !self
                .tabs
                .get(&self.current_tab.clone().unwrap())
                .unwrap()
                .view
                .is_loading()
        }

        fn new_tab(&mut self, url: &str) {
            if !self.tabs.contains_key(url) {
                let view = self
                    .renderer
                    .create_view(self.width, self.height, &self.view_config, None)
                    .unwrap();

                let surface = view.surface().unwrap();
                view.load_url(url).unwrap();

                // RGBA
                debug_assert!(surface.row_bytes() / self.width == 4);

                let tab = Tab {
                    url: url.to_owned(),
                    view,
                    surface,
                    image: None,
                };

                self.tabs.entry(tab.url.clone()).or_insert(tab);
                self.current_tab = Some(url.to_owned());
            }
        }

        fn goto_tab(&mut self, url: &str) -> Option<()> {
            if self.tabs.contains_key(url) {
                self.current_tab = Some(url.to_string());
                return Some(());
            } else {
                return None;
            }
        }

        fn refresh(&self) {
            self.tabs
                .get(&self.current_tab.clone().unwrap())
                .unwrap()
                .view
                .reload();
        }

        fn go_forward(&self) {
            self.tabs
                .get(&self.current_tab.clone().unwrap())
                .unwrap()
                .view
                .go_forward();
        }

        fn go_back(&self) {
            self.tabs
                .get(&self.current_tab.clone().unwrap())
                .unwrap()
                .view
                .go_back();
        }

        fn scroll(&self, delta: ScrollDelta) -> iced::event::Status {
            let scroll_event = match delta {
                ScrollDelta::Lines { x, y } => ScrollEvent::new(
                    ul_next::event::ScrollEventType::ScrollByPage,
                    x as i32,
                    y as i32,
                )
                .unwrap(),
                ScrollDelta::Pixels { x, y } => ScrollEvent::new(
                    ul_next::event::ScrollEventType::ScrollByPixel,
                    x as i32,
                    y as i32,
                )
                .unwrap(),
            };
            self.tabs
                .get(&self.current_tab.clone().unwrap())
                .unwrap()
                .view
                .fire_scroll_event(scroll_event);
            iced::event::Status::Captured
        }

        fn handle_keyboard_event(&self, event: iced::keyboard::Event) -> iced::event::Status {
            match event {
                iced::keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                } => iced::event::Status::Ignored,
                iced::keyboard::Event::KeyReleased {
                    key,
                    location,
                    modifiers,
                } => iced::event::Status::Ignored,
                iced::keyboard::Event::ModifiersChanged(_) => iced::event::Status::Ignored,
            }
        }

        fn handle_mouse_event(&mut self, event: iced::mouse::Event) -> iced::event::Status {
            match event {
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Other(_)) => {
                    iced::event::Status::Ignored
                }
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Other(_)) => {
                    iced::event::Status::Ignored
                }
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Middle) => {
                    iced::event::Status::Ignored
                }
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Middle) => {
                    iced::event::Status::Ignored
                }
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Forward) => {
                    iced::event::Status::Ignored
                }
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Forward) => {
                    iced::event::Status::Ignored
                }
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Back) => {
                    iced::event::Status::Ignored
                }
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Back) => {
                    iced::event::Status::Ignored
                }
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                    if let Some(mouse_loc) = self.mouse_loc {
                        self.tabs
                            .get(&self.current_tab.to_owned().unwrap())
                            .unwrap()
                            .view
                            .fire_mouse_event(
                                MouseEvent::new(
                                    ul_next::event::MouseEventType::MouseDown,
                                    mouse_loc.x as i32,
                                    mouse_loc.y as i32,
                                    ul_next::event::MouseButton::Left,
                                )
                                .unwrap(),
                            );
                        iced::event::Status::Captured
                    } else {
                        iced::event::Status::Ignored
                    }
                }
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
                    if let Some(mouse_loc) = self.mouse_loc {
                        self.tabs
                            .get(&self.current_tab.to_owned().unwrap())
                            .unwrap()
                            .view
                            .fire_mouse_event(
                                MouseEvent::new(
                                    ul_next::event::MouseEventType::MouseUp,
                                    mouse_loc.x as i32,
                                    mouse_loc.y as i32,
                                    ul_next::event::MouseButton::Left,
                                )
                                .unwrap(),
                            );
                        iced::event::Status::Captured
                    } else {
                        iced::event::Status::Ignored
                    }
                }
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right) => {
                    if let Some(mouse_loc) = self.mouse_loc {
                        self.tabs
                            .get(&self.current_tab.to_owned().unwrap())
                            .unwrap()
                            .view
                            .fire_mouse_event(
                                MouseEvent::new(
                                    ul_next::event::MouseEventType::MouseDown,
                                    mouse_loc.x as i32,
                                    mouse_loc.y as i32,
                                    ul_next::event::MouseButton::Right,
                                )
                                .unwrap(),
                            );
                        iced::event::Status::Captured
                    } else {
                        iced::event::Status::Ignored
                    }
                }
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Right) => {
                    if let Some(mouse_loc) = self.mouse_loc {
                        self.tabs
                            .get(&self.current_tab.to_owned().unwrap())
                            .unwrap()
                            .view
                            .fire_mouse_event(
                                MouseEvent::new(
                                    ul_next::event::MouseEventType::MouseUp,
                                    mouse_loc.x as i32,
                                    mouse_loc.y as i32,
                                    ul_next::event::MouseButton::Right,
                                )
                                .unwrap(),
                            );
                        iced::event::Status::Captured
                    } else {
                        iced::event::Status::Ignored
                    }
                }
                iced::mouse::Event::CursorMoved { position } => {
                    self.mouse_loc = Some(position);
                    self.tabs
                        .get(&self.current_tab.to_owned().unwrap())
                        .unwrap()
                        .view
                        .fire_mouse_event(
                            MouseEvent::new(
                                ul_next::event::MouseEventType::MouseMoved,
                                position.x as i32,
                                position.y as i32,
                                ul_next::event::MouseButton::None,
                            )
                            .unwrap(),
                        );
                    iced::event::Status::Captured
                }
                iced::mouse::Event::WheelScrolled { delta } => self.scroll(delta),
                // cursur left browser view
                iced::mouse::Event::CursorLeft => iced::event::Status::Ignored,
                // cursur entered browser view
                iced::mouse::Event::CursorEntered => iced::event::Status::Ignored,
            }
        }
    }
}
