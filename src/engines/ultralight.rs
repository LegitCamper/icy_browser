use iced::event::Status;
use iced::keyboard::{self};
use iced::mouse::{self, ScrollDelta};
use iced::Point;
use smol_str::SmolStr;
use std::collections::HashMap;
use ul_next::event::{self, KeyEventCreationInfo};
use ul_next::{
    config::Config,
    event::{MouseEvent, ScrollEvent},
    key_code::VirtualKeyCode,
    platform::{self, LogLevel, Logger},
    renderer::Renderer,
    view::{View, ViewConfig},
    Surface,
};

use super::BrowserEngine;

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
}

pub struct Ultralight {
    renderer: Renderer,
    view_config: ViewConfig,
    width: u32,
    height: u32,
    // mouse_loc: Option<Point>,
    current_tab: Option<String>,
    tabs: HashMap<String, Tab>,
}

impl Ultralight {
    pub fn new(width: u32, height: u32) -> Self
    where
        Self: Sized,
    {
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
            // mouse_loc: None,
            current_tab: None,
            tabs: HashMap::new(),
        }
    }

    fn get_tab(&mut self) -> Option<&Tab> {
        self.tabs.get(&self.current_tab.clone()?)
    }
}

impl BrowserEngine for Ultralight {
    fn new(width: u32, height: u32) -> Self {
        Self::new(width, height)
    }

    fn do_work(&self) {
        self.renderer.update()
    }

    fn render(&self) {
        self.renderer.render()
    }

    fn needs_render(&self) -> bool {
        self.tabs
            .get(&self.current_tab.clone().unwrap())
            .unwrap()
            .view
            .needs_paint()
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

    fn focus(&self) {
        self.tabs
            .get(&self.current_tab.clone().unwrap())
            .unwrap()
            .view
            .focus();
    }

    fn unfocus(&self) {
        self.tabs
            .get(&self.current_tab.clone().unwrap())
            .unwrap()
            .view
            .unfocus();
    }

    fn scroll(&self, delta: ScrollDelta) -> Status {
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
        Status::Captured
    }

    fn handle_keyboard_event(&self, event: keyboard::Event) -> Status {
        let key_event = match event {
            keyboard::Event::KeyPressed {
                key,
                location,
                modifiers,
                text,
            } => iced_key_to_ultralight_key(
                KeyPress::Press,
                Some(key),
                Some(location),
                modifiers,
                text,
            ),
            keyboard::Event::KeyReleased {
                key,
                location,
                modifiers,
            } => iced_key_to_ultralight_key(
                KeyPress::Unpress,
                Some(key),
                Some(location),
                modifiers,
                None,
            ),
            keyboard::Event::ModifiersChanged(modifiers) => {
                iced_key_to_ultralight_key(KeyPress::Press, None, None, modifiers, None)
            }
        };

        match key_event {
            Some(key_event) => {
                self.tabs
                    .get(&self.current_tab.clone().unwrap())
                    .unwrap()
                    .view
                    .fire_key_event(key_event);

                Status::Captured
            }
            None => Status::Ignored,
        }
    }

    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event) -> Status {
        match event {
            mouse::Event::ButtonPressed(mouse::Button::Other(_)) => Status::Ignored,
            mouse::Event::ButtonReleased(mouse::Button::Other(_)) => Status::Ignored,
            mouse::Event::ButtonPressed(mouse::Button::Middle) => Status::Ignored,
            mouse::Event::ButtonReleased(mouse::Button::Middle) => Status::Ignored,
            mouse::Event::ButtonPressed(mouse::Button::Forward) => Status::Ignored,
            mouse::Event::ButtonReleased(mouse::Button::Forward) => Status::Ignored,
            mouse::Event::ButtonPressed(mouse::Button::Back) => Status::Ignored,
            mouse::Event::ButtonReleased(mouse::Button::Back) => Status::Ignored,
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                self.tabs
                    .get(&self.current_tab.to_owned().unwrap())
                    .unwrap()
                    .view
                    .fire_mouse_event(
                        MouseEvent::new(
                            ul_next::event::MouseEventType::MouseDown,
                            point.x as i32,
                            point.y as i32,
                            ul_next::event::MouseButton::Left,
                        )
                        .unwrap(),
                    );
                Status::Captured
            }
            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                self.tabs
                    .get(&self.current_tab.to_owned().unwrap())
                    .unwrap()
                    .view
                    .fire_mouse_event(
                        MouseEvent::new(
                            ul_next::event::MouseEventType::MouseUp,
                            point.x as i32,
                            point.y as i32,
                            ul_next::event::MouseButton::Left,
                        )
                        .unwrap(),
                    );
                Status::Captured
            }
            mouse::Event::ButtonPressed(mouse::Button::Right) => {
                self.tabs
                    .get(&self.current_tab.to_owned().unwrap())
                    .unwrap()
                    .view
                    .fire_mouse_event(
                        MouseEvent::new(
                            ul_next::event::MouseEventType::MouseDown,
                            point.x as i32,
                            point.y as i32,
                            ul_next::event::MouseButton::Right,
                        )
                        .unwrap(),
                    );
                Status::Captured
            }
            mouse::Event::ButtonReleased(mouse::Button::Right) => {
                self.tabs
                    .get(&self.current_tab.to_owned().unwrap())
                    .unwrap()
                    .view
                    .fire_mouse_event(
                        MouseEvent::new(
                            ul_next::event::MouseEventType::MouseUp,
                            point.x as i32,
                            point.y as i32,
                            ul_next::event::MouseButton::Right,
                        )
                        .unwrap(),
                    );
                Status::Captured
            }
            mouse::Event::CursorMoved { position: _ } => {
                self.tabs
                    .get(&self.current_tab.to_owned().unwrap())
                    .unwrap()
                    .view
                    .fire_mouse_event(
                        MouseEvent::new(
                            ul_next::event::MouseEventType::MouseMoved,
                            point.x as i32,
                            point.y as i32,
                            ul_next::event::MouseButton::None,
                        )
                        .unwrap(),
                    );
                Status::Captured
            }
            mouse::Event::WheelScrolled { delta } => self.scroll(delta),
            mouse::Event::CursorLeft => {
                self.unfocus();
                Status::Captured
            }
            mouse::Event::CursorEntered => {
                self.focus();
                Status::Captured
            }
        }
    }
}

enum KeyPress {
    Press,
    Unpress,
}

fn iced_key_to_ultralight_key(
    press: KeyPress,
    key: Option<keyboard::Key>,
    _location: Option<keyboard::Location>,
    modifiers: keyboard::Modifiers,
    text: Option<SmolStr>,
) -> Option<event::KeyEvent> {
    let ty = match press {
        KeyPress::Press => event::KeyEventType::KeyDown,
        KeyPress::Unpress => event::KeyEventType::KeyUp,
    };
    let modifiers = event::KeyEventModifiers {
        alt: modifiers.alt(),
        ctrl: modifiers.control(),
        meta: modifiers.logo(),
        shift: modifiers.shift(),
    };

    let (virtual_key, native_key) = {
        if let Some(key) = key {
            match key {
                keyboard::Key::Named(key) => match key {
                    keyboard::key::Named::Control => (VirtualKeyCode::Control, 17),
                    keyboard::key::Named::Shift => (VirtualKeyCode::Shift, 16),
                    keyboard::key::Named::Enter => (VirtualKeyCode::Return, 13),
                    keyboard::key::Named::Tab => (VirtualKeyCode::Tab, 9),
                    keyboard::key::Named::Space => (VirtualKeyCode::Space, 32),
                    keyboard::key::Named::ArrowDown => (VirtualKeyCode::Down, 40),
                    keyboard::key::Named::ArrowLeft => (VirtualKeyCode::Right, 37),
                    keyboard::key::Named::ArrowRight => (VirtualKeyCode::Up, 39),
                    keyboard::key::Named::ArrowUp => (VirtualKeyCode::Up, 33),
                    keyboard::key::Named::End => (VirtualKeyCode::End, 35),
                    keyboard::key::Named::Home => (VirtualKeyCode::Home, 36),
                    keyboard::key::Named::Backspace => (VirtualKeyCode::Back, 8),
                    keyboard::key::Named::Clear => (VirtualKeyCode::Clear, 12),
                    keyboard::key::Named::Delete => (VirtualKeyCode::Delete, 46),
                    keyboard::key::Named::Insert => (VirtualKeyCode::Insert, 45),
                    keyboard::key::Named::Escape => (VirtualKeyCode::Escape, 27),
                    keyboard::key::Named::F1 => (VirtualKeyCode::F1, 112),
                    keyboard::key::Named::F2 => (VirtualKeyCode::F2, 113),
                    keyboard::key::Named::F3 => (VirtualKeyCode::F3, 114),
                    keyboard::key::Named::F4 => (VirtualKeyCode::F4, 115),
                    keyboard::key::Named::F5 => (VirtualKeyCode::F5, 116),
                    keyboard::key::Named::F6 => (VirtualKeyCode::F6, 117),
                    keyboard::key::Named::F7 => (VirtualKeyCode::F7, 118),
                    keyboard::key::Named::F8 => (VirtualKeyCode::F8, 119),
                    keyboard::key::Named::F9 => (VirtualKeyCode::F9, 120),
                    keyboard::key::Named::F10 => (VirtualKeyCode::F10, 121),
                    keyboard::key::Named::F11 => (VirtualKeyCode::F11, 122),
                    keyboard::key::Named::F12 => (VirtualKeyCode::F12, 123),
                    _ => (VirtualKeyCode::Unknown, 31),
                },
                keyboard::Key::Character(key) => match key.as_str() {
                    "a" => (VirtualKeyCode::A, 65),
                    "b" => (VirtualKeyCode::B, 66),
                    "c" => (VirtualKeyCode::C, 67),
                    "d" => (VirtualKeyCode::D, 68),
                    "e" => (VirtualKeyCode::E, 69),
                    "f" => (VirtualKeyCode::F, 70),
                    "g" => (VirtualKeyCode::G, 71),
                    "h" => (VirtualKeyCode::H, 72),
                    "i" => (VirtualKeyCode::I, 73),
                    "j" => (VirtualKeyCode::J, 74),
                    "k" => (VirtualKeyCode::K, 75),
                    "l" => (VirtualKeyCode::L, 76),
                    "m" => (VirtualKeyCode::M, 77),
                    "n" => (VirtualKeyCode::N, 78),
                    "o" => (VirtualKeyCode::O, 79),
                    "p" => (VirtualKeyCode::P, 80),
                    "q" => (VirtualKeyCode::Q, 81),
                    "r" => (VirtualKeyCode::R, 82),
                    "s" => (VirtualKeyCode::S, 83),
                    "t" => (VirtualKeyCode::T, 84),
                    "u" => (VirtualKeyCode::U, 85),
                    "v" => (VirtualKeyCode::V, 86),
                    "w" => (VirtualKeyCode::W, 87),
                    "x" => (VirtualKeyCode::X, 88),
                    "y" => (VirtualKeyCode::Y, 89),
                    "z" => (VirtualKeyCode::Z, 90),
                    "," => (VirtualKeyCode::OemComma, 188),
                    "." => (VirtualKeyCode::OemPeriod, 190),
                    ";" => (VirtualKeyCode::OemPeriod, 186),
                    _ => (VirtualKeyCode::C, 31),
                },
                keyboard::Key::Unidentified => (VirtualKeyCode::Unknown, 31),
            }
        } else {
            (VirtualKeyCode::Unknown, 31)
        }
    };

    let text: &str = match text {
        Some(text) => &text.to_string(),
        None => "",
    };

    println!("text {}", text);

    let creation_info = KeyEventCreationInfo {
        ty,
        modifiers,
        virtual_key_code: virtual_key,
        native_key_code: native_key,
        text,
        unmodified_text: text,
        is_keypad: false,
        is_auto_repeat: false,
        is_system_key: false,
    };

    Some(event::KeyEvent::new(creation_info).ok()?)
}
