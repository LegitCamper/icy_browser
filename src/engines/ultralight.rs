use iced::event::Status;
use iced::keyboard::{self};
use iced::mouse::{self, ScrollDelta};
use iced::Point;
use smol_str::SmolStr;
use std::sync::{Arc, RwLock};
use ul_next::event::{self, KeyEventCreationInfo};
use ul_next::{
    config::Config,
    event::{MouseEvent, ScrollEvent},
    key_code::VirtualKeyCode,
    platform::{self, LogLevel, Logger},
    renderer::Renderer,
    view::{View, ViewConfig},
    window::Cursor,
    Surface,
};

struct UlLogger;
impl Logger for UlLogger {
    fn log_message(&mut self, log_level: LogLevel, message: String) {
        println!("{:?}: {}", log_level, message);
    }
}

pub struct Tab {
    surface: Surface,
    view: View,
    title: Arc<RwLock<String>>,
    url: Arc<RwLock<String>>,
    cursor: Arc<RwLock<mouse::Interaction>>,
}

pub struct Ultralight {
    renderer: Renderer,
    view_config: ViewConfig,
    width: u32,
    height: u32,
    current_tab: Option<u32>,
    tabs: Vec<Tab>,
    last_view: Option<Vec<u8>>,
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
            current_tab: None,
            tabs: Vec::new(),
            last_view: None,
        }
    }

    fn get_tab(&self) -> Option<&Tab> {
        if let Some(index) = self.current_tab.as_ref() {
            self.tabs.get(*index as usize)
        } else {
            None
        }
    }

    fn get_tab_mut(&mut self) -> Option<&mut Tab> {
        if let Some(index) = self.current_tab.as_ref() {
            self.tabs.get_mut(*index as usize)
        } else {
            None
        }
    }
}

impl super::BrowserEngine for Ultralight {
    fn new() -> Self {
        Ultralight::new(800, 800)
    }

    fn do_work(&self) {
        self.renderer.update()
    }

    fn need_render(&self) -> bool {
        self.get_tab().unwrap().view.needs_paint()
    }

    fn render(&mut self) {
        self.renderer.render();
    }

    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn resize(&mut self, width: u32, height: u32) {
        (self.width, self.height) = (width, height);
        self.tabs.iter().for_each(|tab| {
            tab.view.resize(width, height);
            tab.surface.resize(width, height);
        })
    }

    fn pixel_buffer(&mut self) -> Option<Vec<u8>> {
        // Get the raw pixels of the surface
        if self.need_render() {
            self.render();

            let size = self.size();
            let mut vec = Vec::new();
            if let Some(pixels_data) = self.get_tab_mut()?.surface.lock_pixels() {
                vec.extend_from_slice(&pixels_data);
            } else {
                let image = vec![255; size.0 as usize * size.1 as usize];
                vec.extend_from_slice(&image)
            }
            self.last_view = Some(vec);
            return Some(self.last_view.clone()?);
        }

        self.last_view.clone()
    }

    fn get_cursor(&self) -> mouse::Interaction {
        *self.get_tab().unwrap().cursor.read().unwrap()
    }

    fn get_title(&self) -> Option<String> {
        self.get_tab()?.view.title().ok()
    }

    fn get_url(&self) -> Option<String> {
        Some(self.get_tab()?.url.read().ok()?.clone())
    }

    fn goto_url(&self, url: &str) {
        self.get_tab().unwrap().view.load_url(url).unwrap();
    }

    fn has_loaded(&self) -> bool {
        !self.get_tab().unwrap().view.is_loading()
    }

    fn new_tab(&mut self, url: &str) {
        if self
            .tabs
            .iter()
            .filter(|tab| *tab.url.read().unwrap() == *url)
            .count()
            == 0
        {
            let view = self
                .renderer
                .create_view(self.width, self.height, &self.view_config, None)
                .unwrap();

            let surface = view.surface().unwrap();
            view.load_url(url).unwrap();

            // RGBA
            debug_assert!(surface.row_bytes() / self.width == 4);

            // set callbacks
            let site_url = Arc::new(RwLock::new(url.to_string()));
            let cb_url = site_url.clone();
            view.set_change_url_callback(move |_view, url| {
                *cb_url.write().unwrap() = url;
            });

            let title = Arc::new(RwLock::new(view.title().unwrap()));
            let cb_title = title.clone();
            view.set_change_title_callback(move |_view, title| {
                *cb_title.write().unwrap() = title;
            });

            let cursor = Arc::new(RwLock::new(mouse::Interaction::Idle));
            let cb_cursor = cursor.clone();
            view.set_change_cursor_callback(move |_view, cursor_update| {
                *cb_cursor.write().unwrap() = match cursor_update {
                    Cursor::None => mouse::Interaction::Idle,
                    Cursor::Pointer => mouse::Interaction::Pointer,
                    Cursor::Hand => mouse::Interaction::Pointer,
                    Cursor::Grab => mouse::Interaction::Grab,
                    Cursor::VerticalText => mouse::Interaction::Text,
                    Cursor::IBeam => mouse::Interaction::Text,
                    Cursor::Cross => mouse::Interaction::Crosshair,
                    Cursor::Wait => mouse::Interaction::Working,
                    Cursor::Grabbing => mouse::Interaction::Grab,
                    Cursor::NorthSouthResize => mouse::Interaction::ResizingVertically,
                    Cursor::EastWestResize => mouse::Interaction::ResizingHorizontally,
                    Cursor::NotAllowed => mouse::Interaction::NotAllowed,
                    Cursor::ZoomIn => mouse::Interaction::ZoomIn,
                    Cursor::ZoomOut => mouse::Interaction::ZoomIn,
                    _ => mouse::Interaction::Pointer,
                };
            });

            let tab = Tab {
                title,
                view,
                surface,
                url: site_url,
                cursor,
            };

            self.tabs.push(tab);
            self.current_tab = Some(self.tabs.len() as u32 - 1);
        }
    }

    fn goto_tab(&mut self, url: &str) -> Option<()> {
        for (index, tab) in self.tabs.iter().enumerate() {
            if *tab.url.read().unwrap() == url {
                self.current_tab = Some(index as u32);
                return Some(());
            }
        }
        None
    }

    fn refresh(&self) {
        self.get_tab().unwrap().view.reload();
    }

    fn go_forward(&self) {
        self.get_tab().unwrap().view.go_forward();
    }

    fn go_back(&self) {
        self.get_tab().unwrap().view.go_back();
    }

    fn focus(&self) {
        self.get_tab().unwrap().view.focus();
    }

    fn unfocus(&self) {
        self.get_tab().unwrap().view.unfocus();
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
        self.get_tab().unwrap().view.fire_scroll_event(scroll_event);
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
                self.get_tab().unwrap().view.fire_key_event(key_event);

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
                self.get_tab().unwrap().view.fire_mouse_event(
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
                self.get_tab().unwrap().view.fire_mouse_event(
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
                self.get_tab().unwrap().view.fire_mouse_event(
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
                self.get_tab().unwrap().view.fire_mouse_event(
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
                self.get_tab().unwrap().view.fire_mouse_event(
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
                    _ => return None,
                },
                #[cfg(windows)]
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
                    _ => return None,
                },
                #[cfg(unix)]
                keyboard::Key::Character(key) => match key.as_str() {
                    "1" => (VirtualKeyCode::Key1, 2),
                    "2" => (VirtualKeyCode::Key2, 3),
                    "3" => (VirtualKeyCode::Key3, 4),
                    "4" => (VirtualKeyCode::Key4, 5),
                    "5" => (VirtualKeyCode::Key5, 6),
                    "6" => (VirtualKeyCode::Key6, 7),
                    "7" => (VirtualKeyCode::Key7, 8),
                    "8" => (VirtualKeyCode::Key8, 9),
                    "9" => (VirtualKeyCode::Key9, 10),
                    "0" => (VirtualKeyCode::Key0, 11),
                    "a" => (VirtualKeyCode::A, 30),
                    "b" => (VirtualKeyCode::B, 48),
                    "c" => (VirtualKeyCode::C, 46),
                    "d" => (VirtualKeyCode::D, 32),
                    "e" => (VirtualKeyCode::E, 18),
                    "f" => (VirtualKeyCode::F, 33),
                    "g" => (VirtualKeyCode::G, 34),
                    "h" => (VirtualKeyCode::H, 35),
                    "i" => (VirtualKeyCode::I, 23),
                    "j" => (VirtualKeyCode::J, 36),
                    "k" => (VirtualKeyCode::K, 37),
                    "l" => (VirtualKeyCode::L, 38),
                    "m" => (VirtualKeyCode::M, 50),
                    "n" => (VirtualKeyCode::N, 49),
                    "o" => (VirtualKeyCode::O, 24),
                    "p" => (VirtualKeyCode::P, 25),
                    "q" => (VirtualKeyCode::Q, 16),
                    "r" => (VirtualKeyCode::R, 19),
                    "s" => (VirtualKeyCode::S, 31),
                    "t" => (VirtualKeyCode::T, 20),
                    "u" => (VirtualKeyCode::U, 22),
                    "v" => (VirtualKeyCode::V, 47),
                    "w" => (VirtualKeyCode::W, 17),
                    "x" => (VirtualKeyCode::X, 47),
                    "y" => (VirtualKeyCode::Y, 21),
                    "z" => (VirtualKeyCode::Z, 44),
                    "," => (VirtualKeyCode::OemComma, 51),
                    "." => (VirtualKeyCode::OemPeriod, 52),
                    ";" => (VirtualKeyCode::OemPeriod, 39),
                    _ => return None,
                },
                keyboard::Key::Unidentified => return None,
            }
        } else {
            return None;
        }
    };

    let text: &str = match text {
        Some(text) => &text.to_string(),
        None => "",
    };

    let creation_info = KeyEventCreationInfo {
        ty,
        modifiers,
        virtual_key_code: virtual_key,
        native_key_code: native_key,
        text,
        unmodified_text: text,
        is_keypad: false,
        is_auto_repeat: false,
        #[cfg(windows)]
        is_system_key: true,
        #[cfg(not(windows))]
        is_system_key: false,
    };

    Some(event::KeyEvent::new(creation_info).ok()?)
}
