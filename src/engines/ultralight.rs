use iced::keyboard::{self};
use iced::mouse::{self, ScrollDelta};
use iced::{Point, Size};
use smol_str::SmolStr;
use std::sync::{Arc, RwLock};
use ul_next::{
    config::Config,
    event::{self, KeyEventCreationInfo, MouseEvent, ScrollEvent},
    key_code::VirtualKeyCode,
    platform::{self, LogLevel, Logger},
    renderer::Renderer,
    view::{View, ViewConfig},
    window::Cursor,
    Surface,
};
use url::Url;

#[cfg(not(debug_assertions))]
use env_home::env_home_dir;

use super::{BrowserEngine, PixelFormat, Tab, TabInfo, Tabs};

struct UlLogger;
impl Logger for UlLogger {
    fn log_message(&mut self, log_level: LogLevel, message: String) {
        println!("{:?}: {}", log_level, message);
    }
}

pub struct UltalightTabInfo {
    surface: Surface,
    view: View,
    cursor: Arc<RwLock<mouse::Interaction>>,
}

impl TabInfo for UltalightTabInfo {
    fn title(&self) -> String {
        self.view
            .title()
            .expect("Failed to get title from ultralight")
    }

    fn url(&self) -> String {
        self.view.url().expect("Failed to get url from ultralight")
    }
}

pub struct Ultralight {
    renderer: Renderer,
    view_config: ViewConfig,
    tabs: Tabs<UltalightTabInfo>,
}

impl Default for Ultralight {
    fn default() -> Self {
        Self::new()
    }
}

impl Ultralight {
    pub fn new() -> Self {
        let config = Config::start().build().unwrap();
        platform::enable_platform_fontloader();

        #[cfg(not(debug_assertions))]
        let mut home_dir = env_home_dir().unwrap();
        #[cfg(not(debug_assertions))]
        home_dir.push(".icy_browser");
        #[cfg(not(debug_assertions))]
        platform::enable_platform_filesystem(home_dir.as_path()).unwrap();

        #[cfg(debug_assertions)]
        platform::enable_platform_filesystem(".").unwrap();

        platform::set_logger(UlLogger);

        #[cfg(not(debug_assertions))]
        home_dir.push("logs.txt");
        #[cfg(not(debug_assertions))]
        platform::enable_default_logger(home_dir.as_path()).unwrap();

        #[cfg(debug_assertions)]
        platform::enable_default_logger("./logs.txt").unwrap();

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
            tabs: Tabs::new(),
        }
    }
}

impl BrowserEngine for Ultralight {
    type Info = UltalightTabInfo;

    fn new() -> Self {
        Ultralight::new()
    }

    fn do_work(&self) {
        self.renderer.update()
    }

    fn force_need_render(&self) {
        self.get_tabs()
            .get_current()
            .info
            .view
            .set_needs_paint(true)
    }

    fn need_render(&self) -> bool {
        self.get_tabs().get_current().info.view.needs_paint()
    }

    fn render(&mut self) {
        self.renderer.render();
    }

    fn size(&self) -> (u32, u32) {
        let view = &self.tabs.get_current().info.view;
        (view.width(), view.height())
    }

    fn resize(&mut self, size: Size<u32>) {
        self.tabs.tabs.iter().for_each(|tab| {
            tab.info.view.resize(size.width, size.height);
            tab.info.surface.resize(size.width, size.height);
        })
    }

    fn pixel_buffer(&mut self) -> (PixelFormat, Vec<u8>) {
        self.render();

        let size = self.size();
        let mut vec = Vec::new();
        match self.tabs.get_current_mut().info.surface.lock_pixels() {
            Some(pixel_data) => vec.extend_from_slice(&pixel_data),
            None => {
                let image = vec![255; size.0 as usize * size.1 as usize];
                vec.extend_from_slice(&image)
            }
        };

        (PixelFormat::Bgra, vec)
    }

    fn get_cursor(&self) -> mouse::Interaction {
        *self.tabs.get_current().info.cursor.read().unwrap()
    }

    fn goto_url(&self, url: &Url) {
        self.tabs
            .get_current()
            .info
            .view
            .load_url(url.as_ref())
            .unwrap();
    }

    fn has_loaded(&self) -> bool {
        !self.tabs.get_current().info.view.is_loading()
    }

    fn get_tabs(&self) -> &Tabs<UltalightTabInfo> {
        &self.tabs
    }

    fn get_tabs_mut(&mut self) -> &mut Tabs<UltalightTabInfo> {
        &mut self.tabs
    }

    fn new_tab(&mut self, url: Url, size: Size<u32>) -> Tab<UltalightTabInfo> {
        let view = self
            .renderer
            .create_view(size.width, size.height, &self.view_config, None)
            .unwrap();

        let surface = view.surface().unwrap();
        view.load_url(url.as_ref()).unwrap();

        // RGBA
        debug_assert!(surface.row_bytes() / size.width == 4);

        let cursor = Arc::new(RwLock::new(mouse::Interaction::Idle));
        let cb_cursor = cursor.clone();
        view.set_change_cursor_callback(move |_view, cursor_update| {
            *cb_cursor.write().unwrap() = match cursor_update {
                Cursor::None => mouse::Interaction::Idle,
                Cursor::Pointer => mouse::Interaction::Idle,
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

        let info = UltalightTabInfo {
            surface,
            view,
            cursor,
        };

        Tab::new(info)
    }

    fn refresh(&self) {
        self.tabs.get_current().info.view.reload();
    }

    fn go_forward(&self) {
        self.tabs.get_current().info.view.go_forward();
    }

    fn go_back(&self) {
        self.tabs.get_current().info.view.go_back();
    }

    fn focus(&self) {
        self.tabs.get_current().info.view.focus();
    }

    fn unfocus(&self) {
        self.tabs.get_current().info.view.unfocus();
    }

    fn scroll(&self, delta: ScrollDelta) {
        let scroll_event = match delta {
            ScrollDelta::Lines { x, y } => ScrollEvent::new(
                ul_next::event::ScrollEventType::ScrollByPixel,
                x as i32 * 100,
                y as i32 * 100,
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
            .get_current()
            .info
            .view
            .fire_scroll_event(scroll_event);
    }

    fn handle_keyboard_event(&self, event: keyboard::Event) {
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

        if let Some(key_event) = key_event {
            self.tabs.get_current().info.view.fire_key_event(key_event);
        }
    }

    fn handle_mouse_event(&mut self, point: Point, event: mouse::Event) {
        match event {
            mouse::Event::ButtonPressed(mouse::Button::Other(_)) => (),
            mouse::Event::ButtonReleased(mouse::Button::Other(_)) => (),
            mouse::Event::ButtonPressed(mouse::Button::Middle) => (),
            mouse::Event::ButtonReleased(mouse::Button::Middle) => (),
            mouse::Event::ButtonPressed(mouse::Button::Forward) => (),
            mouse::Event::ButtonReleased(mouse::Button::Forward) => (),
            mouse::Event::ButtonPressed(mouse::Button::Back) => (),
            mouse::Event::ButtonReleased(mouse::Button::Back) => (),
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                self.tabs.get_current().info.view.fire_mouse_event(
                    MouseEvent::new(
                        ul_next::event::MouseEventType::MouseDown,
                        point.x as i32,
                        point.y as i32,
                        ul_next::event::MouseButton::Left,
                    )
                    .unwrap(),
                );
            }
            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                self.tabs.get_current().info.view.fire_mouse_event(
                    MouseEvent::new(
                        ul_next::event::MouseEventType::MouseUp,
                        point.x as i32,
                        point.y as i32,
                        ul_next::event::MouseButton::Left,
                    )
                    .unwrap(),
                );
            }
            mouse::Event::ButtonPressed(mouse::Button::Right) => {
                self.tabs.get_current().info.view.fire_mouse_event(
                    MouseEvent::new(
                        ul_next::event::MouseEventType::MouseDown,
                        point.x as i32,
                        point.y as i32,
                        ul_next::event::MouseButton::Right,
                    )
                    .unwrap(),
                );
            }
            mouse::Event::ButtonReleased(mouse::Button::Right) => {
                self.tabs.get_current().info.view.fire_mouse_event(
                    MouseEvent::new(
                        ul_next::event::MouseEventType::MouseUp,
                        point.x as i32,
                        point.y as i32,
                        ul_next::event::MouseButton::Right,
                    )
                    .unwrap(),
                );
            }
            mouse::Event::CursorMoved { position: _ } => {
                self.tabs.get_current().info.view.fire_mouse_event(
                    MouseEvent::new(
                        ul_next::event::MouseEventType::MouseMoved,
                        point.x as i32,
                        point.y as i32,
                        ul_next::event::MouseButton::None,
                    )
                    .unwrap(),
                );
            }
            mouse::Event::WheelScrolled { delta } => self.scroll(delta),
            mouse::Event::CursorLeft => {
                self.unfocus();
            }
            mouse::Event::CursorEntered => {
                self.focus();
            }
        }
    }
}

#[derive(PartialEq, Eq)]
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
    let mut ty = match press {
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
                #[cfg(windows)]
                keyboard::Key::Named(key) => match key {
                    keyboard::key::Named::Control => (VirtualKeyCode::Control, 17),
                    keyboard::key::Named::Shift => (VirtualKeyCode::Shift, 16),
                    keyboard::key::Named::Enter => (VirtualKeyCode::Return, 13),
                    keyboard::key::Named::Tab => (VirtualKeyCode::Tab, 9),
                    keyboard::key::Named::Space => (VirtualKeyCode::Space, 32),
                    keyboard::key::Named::ArrowDown => (VirtualKeyCode::Down, 40),
                    keyboard::key::Named::ArrowLeft => (VirtualKeyCode::Right, 37),
                    keyboard::key::Named::ArrowRight => (VirtualKeyCode::Up, 39),
                    keyboard::key::Named::ArrowUp => (VirtualKeyCode::Left, 33),
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
                #[cfg(unix)]
                keyboard::Key::Named(key) => match key {
                    keyboard::key::Named::Control => (VirtualKeyCode::Control, 29),
                    keyboard::key::Named::Shift => (VirtualKeyCode::Shift, 42),
                    keyboard::key::Named::Enter => (VirtualKeyCode::Return, 28),
                    keyboard::key::Named::Tab => (VirtualKeyCode::Tab, 15),
                    keyboard::key::Named::Space => (VirtualKeyCode::Space, 57),
                    keyboard::key::Named::ArrowDown => (VirtualKeyCode::Down, 108),
                    keyboard::key::Named::ArrowLeft => (VirtualKeyCode::Right, 106),
                    keyboard::key::Named::ArrowRight => (VirtualKeyCode::Up, 103),
                    keyboard::key::Named::ArrowUp => (VirtualKeyCode::Left, 105),
                    keyboard::key::Named::End => (VirtualKeyCode::End, 107),
                    keyboard::key::Named::Home => (VirtualKeyCode::Home, 102),
                    keyboard::key::Named::Backspace => (VirtualKeyCode::Back, 14),
                    keyboard::key::Named::Delete => (VirtualKeyCode::Delete, 11),
                    keyboard::key::Named::Insert => (VirtualKeyCode::Insert, 110),
                    keyboard::key::Named::Escape => (VirtualKeyCode::Escape, 1),
                    keyboard::key::Named::F1 => (VirtualKeyCode::F1, 59),
                    keyboard::key::Named::F2 => (VirtualKeyCode::F2, 60),
                    keyboard::key::Named::F3 => (VirtualKeyCode::F3, 61),
                    keyboard::key::Named::F4 => (VirtualKeyCode::F4, 62),
                    keyboard::key::Named::F5 => (VirtualKeyCode::F5, 63),
                    keyboard::key::Named::F6 => (VirtualKeyCode::F6, 64),
                    keyboard::key::Named::F7 => (VirtualKeyCode::F7, 65),
                    keyboard::key::Named::F8 => (VirtualKeyCode::F8, 66),
                    keyboard::key::Named::F9 => (VirtualKeyCode::F9, 67),
                    keyboard::key::Named::F10 => (VirtualKeyCode::F10, 68),
                    keyboard::key::Named::F11 => (VirtualKeyCode::F11, 69),
                    keyboard::key::Named::F12 => (VirtualKeyCode::F12, 70),
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

    // solution to char events being ignored:
    // Note that only KeyEventType::Char events actually generate text in input fields.
    // if text has char rewrite event type to char
    if !text.is_empty() {
        ty = event::KeyEventType::Char
    }

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

    event::KeyEvent::new(creation_info).ok()
}
