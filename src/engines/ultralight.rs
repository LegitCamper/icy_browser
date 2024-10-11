use clipboard_rs::{Clipboard, ClipboardContext};
use iced::keyboard::{self};
use iced::mouse::{self, ScrollDelta};
use iced::{Point, Size};
use smol_str::SmolStr;
use std::sync::{Arc, RwLock};
use ul_next::{
    config::Config,
    event::{self, KeyEventCreationInfo, MouseEvent, ScrollEvent},
    key_code::VirtualKeyCode,
    platform,
    renderer::Renderer,
    view::{View, ViewConfig},
    window::Cursor,
    Surface,
};
use url::Url;

use super::{BrowserEngine, PixelFormat, Tab, TabInfo, Tabs};

struct UlClipboard;
impl platform::Clipboard for UlClipboard {
    fn clear(&mut self) {}

    fn read_plain_text(&mut self) -> Option<String> {
        let ctx = clipboard_rs::ClipboardContext::new().ok()?;
        Some(ctx.get_text().unwrap_or("".to_string()))
    }

    fn write_plain_text(&mut self, text: &str) {
        let ctx = ClipboardContext::new().expect("Failed to open clipboard");
        ctx.set_text(text.into())
            .expect("Failed to set contents of clipboard");
    }
}

pub struct UltalightTabInfo {
    surface: Surface,
    view: View,
    cursor: Arc<RwLock<mouse::Interaction>>,
}

impl TabInfo for UltalightTabInfo {
    fn title(&self) -> String {
        self.view.title().unwrap_or("Title Error".to_string())
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
        platform::enable_platform_filesystem(".").unwrap();
        platform::set_clipboard(UlClipboard);

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
                modified_key,
                physical_key: _,
            } => iced_key_to_ultralight_key(
                KeyPress::Press,
                Some(modified_key),
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
                None,
                Some(key),
                Some(location),
                modifiers,
                None,
            ),
            keyboard::Event::ModifiersChanged(modifiers) => {
                iced_key_to_ultralight_key(KeyPress::Press, None, None, None, modifiers, None)
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

#[derive(Debug, PartialEq, Eq)]
enum KeyPress {
    Press,
    Unpress,
}

fn iced_key_to_ultralight_key(
    press: KeyPress,
    modified_key: Option<keyboard::Key>,
    key: Option<keyboard::Key>, // This one is modified by ctrl and results in wrong key
    _location: Option<keyboard::Location>,
    modifiers: keyboard::Modifiers,
    text: Option<SmolStr>,
) -> Option<event::KeyEvent> {
    let (text, virtual_key, native_key) = {
        if let Some(key) = key {
            let text = match key {
                keyboard::Key::Named(key) => {
                    if key == keyboard::key::Named::Space {
                        String::from(" ")
                    } else {
                        String::from("")
                    }
                }
                keyboard::Key::Character(_) => match text {
                    Some(text) => text.to_string(),
                    None => String::from(""),
                },
                keyboard::Key::Unidentified => return None,
            };
            let (virtual_key, native_key) = match key {
                keyboard::Key::Named(key) => match key {
                    keyboard::key::Named::Control => (
                        VirtualKeyCode::Control,
                        #[cfg(windows)]
                        17,
                        #[cfg(unix)]
                        29,
                    ),
                    keyboard::key::Named::Shift => (
                        VirtualKeyCode::Shift,
                        #[cfg(windows)]
                        16,
                        #[cfg(unix)]
                        42,
                    ),
                    keyboard::key::Named::Enter => (
                        VirtualKeyCode::Return,
                        #[cfg(windows)]
                        13,
                        #[cfg(unix)]
                        28,
                    ),
                    keyboard::key::Named::Tab => (
                        VirtualKeyCode::Tab,
                        #[cfg(windows)]
                        9,
                        #[cfg(unix)]
                        15,
                    ),
                    keyboard::key::Named::Space => (
                        VirtualKeyCode::Space,
                        #[cfg(windows)]
                        32,
                        #[cfg(unix)]
                        57,
                    ),
                    keyboard::key::Named::ArrowDown => (
                        VirtualKeyCode::Down,
                        #[cfg(windows)]
                        40,
                        #[cfg(unix)]
                        108,
                    ),
                    keyboard::key::Named::ArrowLeft => (
                        VirtualKeyCode::Right,
                        #[cfg(windows)]
                        37,
                        #[cfg(unix)]
                        106,
                    ),
                    keyboard::key::Named::ArrowRight => (
                        VirtualKeyCode::Up,
                        #[cfg(windows)]
                        39,
                        #[cfg(unix)]
                        103,
                    ),
                    keyboard::key::Named::ArrowUp => (
                        VirtualKeyCode::Left,
                        #[cfg(windows)]
                        33,
                        #[cfg(unix)]
                        105,
                    ),
                    keyboard::key::Named::End => (
                        VirtualKeyCode::End,
                        #[cfg(windows)]
                        35,
                        #[cfg(unix)]
                        107,
                    ),
                    keyboard::key::Named::Home => (
                        VirtualKeyCode::Home,
                        #[cfg(windows)]
                        36,
                        #[cfg(unix)]
                        102,
                    ),
                    keyboard::key::Named::Backspace => (
                        VirtualKeyCode::Back,
                        #[cfg(windows)]
                        8,
                        #[cfg(unix)]
                        14,
                    ),
                    keyboard::key::Named::Delete => (
                        VirtualKeyCode::Delete,
                        #[cfg(windows)]
                        46,
                        #[cfg(unix)]
                        11,
                    ),
                    keyboard::key::Named::Insert => (
                        VirtualKeyCode::Insert,
                        #[cfg(windows)]
                        45,
                        #[cfg(unix)]
                        110,
                    ),
                    keyboard::key::Named::Escape => (
                        VirtualKeyCode::Escape,
                        #[cfg(windows)]
                        27,
                        #[cfg(unix)]
                        1,
                    ),
                    keyboard::key::Named::F1 => (
                        VirtualKeyCode::F1,
                        #[cfg(windows)]
                        112,
                        #[cfg(unix)]
                        59,
                    ),
                    keyboard::key::Named::F2 => (
                        VirtualKeyCode::F2,
                        #[cfg(windows)]
                        113,
                        #[cfg(unix)]
                        60,
                    ),
                    keyboard::key::Named::F3 => (
                        VirtualKeyCode::F3,
                        #[cfg(windows)]
                        114,
                        #[cfg(unix)]
                        61,
                    ),
                    keyboard::key::Named::F4 => (
                        VirtualKeyCode::F4,
                        #[cfg(windows)]
                        115,
                        #[cfg(unix)]
                        62,
                    ),
                    keyboard::key::Named::F5 => (
                        VirtualKeyCode::F5,
                        #[cfg(windows)]
                        116,
                        #[cfg(unix)]
                        63,
                    ),
                    keyboard::key::Named::F6 => (
                        VirtualKeyCode::F6,
                        #[cfg(windows)]
                        117,
                        #[cfg(unix)]
                        64,
                    ),
                    keyboard::key::Named::F7 => (
                        VirtualKeyCode::F7,
                        #[cfg(windows)]
                        118,
                        #[cfg(unix)]
                        65,
                    ),
                    keyboard::key::Named::F8 => (
                        VirtualKeyCode::F8,
                        #[cfg(windows)]
                        119,
                        #[cfg(unix)]
                        66,
                    ),
                    keyboard::key::Named::F9 => (
                        VirtualKeyCode::F9,
                        #[cfg(windows)]
                        120,
                        #[cfg(unix)]
                        67,
                    ),
                    keyboard::key::Named::F10 => (
                        VirtualKeyCode::F10,
                        #[cfg(windows)]
                        121,
                        #[cfg(unix)]
                        68,
                    ),
                    keyboard::key::Named::F11 => (
                        VirtualKeyCode::F11,
                        #[cfg(windows)]
                        122,
                        #[cfg(unix)]
                        69,
                    ),
                    keyboard::key::Named::F12 => (
                        VirtualKeyCode::F12,
                        #[cfg(windows)]
                        123,
                        #[cfg(unix)]
                        70,
                    ),
                    _ => return None,
                },
                keyboard::Key::Character(key) => match key.as_str() {
                    "a" => (
                        VirtualKeyCode::A,
                        #[cfg(windows)]
                        65,
                        #[cfg(unix)]
                        30,
                    ),
                    "b" => (
                        VirtualKeyCode::B,
                        #[cfg(windows)]
                        66,
                        #[cfg(unix)]
                        48,
                    ),
                    "c" => (
                        VirtualKeyCode::C,
                        #[cfg(windows)]
                        67,
                        #[cfg(unix)]
                        46,
                    ),
                    "d" => (
                        VirtualKeyCode::D,
                        #[cfg(windows)]
                        68,
                        #[cfg(unix)]
                        32,
                    ),
                    "e" => (
                        VirtualKeyCode::E,
                        #[cfg(windows)]
                        69,
                        #[cfg(unix)]
                        18,
                    ),
                    "f" => (
                        VirtualKeyCode::F,
                        #[cfg(windows)]
                        70,
                        #[cfg(unix)]
                        33,
                    ),
                    "g" => (
                        VirtualKeyCode::G,
                        #[cfg(windows)]
                        71,
                        #[cfg(unix)]
                        34,
                    ),
                    "h" => (
                        VirtualKeyCode::H,
                        #[cfg(windows)]
                        72,
                        #[cfg(unix)]
                        35,
                    ),
                    "i" => (
                        VirtualKeyCode::I,
                        #[cfg(windows)]
                        73,
                        #[cfg(unix)]
                        23,
                    ),
                    "j" => (
                        VirtualKeyCode::J,
                        #[cfg(windows)]
                        74,
                        #[cfg(unix)]
                        36,
                    ),
                    "k" => (
                        VirtualKeyCode::K,
                        #[cfg(windows)]
                        75,
                        #[cfg(unix)]
                        37,
                    ),
                    "l" => (
                        VirtualKeyCode::L,
                        #[cfg(windows)]
                        76,
                        #[cfg(unix)]
                        38,
                    ),
                    "m" => (
                        VirtualKeyCode::M,
                        #[cfg(windows)]
                        77,
                        #[cfg(unix)]
                        50,
                    ),
                    "n" => (
                        VirtualKeyCode::N,
                        #[cfg(windows)]
                        78,
                        #[cfg(unix)]
                        49,
                    ),
                    "o" => (
                        VirtualKeyCode::O,
                        #[cfg(windows)]
                        79,
                        #[cfg(unix)]
                        24,
                    ),
                    "p" => (
                        VirtualKeyCode::P,
                        #[cfg(windows)]
                        80,
                        #[cfg(unix)]
                        25,
                    ),
                    "q" => (
                        VirtualKeyCode::Q,
                        #[cfg(windows)]
                        81,
                        #[cfg(unix)]
                        16,
                    ),
                    "r" => (
                        VirtualKeyCode::R,
                        #[cfg(windows)]
                        82,
                        #[cfg(unix)]
                        19,
                    ),
                    "s" => (
                        VirtualKeyCode::S,
                        #[cfg(windows)]
                        83,
                        #[cfg(unix)]
                        31,
                    ),
                    "t" => (
                        VirtualKeyCode::T,
                        #[cfg(windows)]
                        84,
                        #[cfg(unix)]
                        20,
                    ),
                    "u" => (
                        VirtualKeyCode::U,
                        #[cfg(windows)]
                        85,
                        #[cfg(unix)]
                        22,
                    ),
                    "v" => (
                        VirtualKeyCode::V,
                        #[cfg(windows)]
                        86,
                        #[cfg(unix)]
                        47,
                    ),
                    "w" => (
                        VirtualKeyCode::W,
                        #[cfg(windows)]
                        87,
                        #[cfg(unix)]
                        17,
                    ),
                    "x" => (
                        VirtualKeyCode::X,
                        #[cfg(windows)]
                        88,
                        #[cfg(unix)]
                        47,
                    ),
                    "y" => (
                        VirtualKeyCode::Y,
                        #[cfg(windows)]
                        89,
                        #[cfg(unix)]
                        21,
                    ),
                    "z" => (
                        VirtualKeyCode::Z,
                        #[cfg(windows)]
                        90,
                        #[cfg(unix)]
                        44,
                    ),
                    "0" => (
                        VirtualKeyCode::Key0,
                        #[cfg(windows)]
                        48,
                        #[cfg(unix)]
                        11,
                    ),
                    "1" => (
                        VirtualKeyCode::Key1,
                        #[cfg(windows)]
                        49,
                        #[cfg(unix)]
                        2,
                    ),
                    "2" => (
                        VirtualKeyCode::Key2,
                        #[cfg(windows)]
                        50,
                        #[cfg(unix)]
                        3,
                    ),
                    "3" => (
                        VirtualKeyCode::Key3,
                        #[cfg(windows)]
                        51,
                        #[cfg(unix)]
                        4,
                    ),
                    "4" => (
                        VirtualKeyCode::Key4,
                        #[cfg(windows)]
                        52,
                        #[cfg(unix)]
                        5,
                    ),
                    "5" => (
                        VirtualKeyCode::Key5,
                        #[cfg(windows)]
                        53,
                        #[cfg(unix)]
                        6,
                    ),
                    "6" => (
                        VirtualKeyCode::Key6,
                        #[cfg(windows)]
                        54,
                        #[cfg(unix)]
                        7,
                    ),
                    "7" => (
                        VirtualKeyCode::Key7,
                        #[cfg(windows)]
                        55,
                        #[cfg(unix)]
                        8,
                    ),
                    "8" => (
                        VirtualKeyCode::Key8,
                        #[cfg(windows)]
                        56,
                        #[cfg(unix)]
                        9,
                    ),
                    "9" => (
                        VirtualKeyCode::Key9,
                        #[cfg(windows)]
                        57,
                        #[cfg(unix)]
                        10,
                    ),
                    "," => (
                        VirtualKeyCode::OemComma,
                        #[cfg(windows)]
                        188,
                        #[cfg(unix)]
                        51,
                    ),
                    "." => (
                        VirtualKeyCode::OemPeriod,
                        #[cfg(windows)]
                        190,
                        #[cfg(unix)]
                        52,
                    ),
                    ";" => (
                        VirtualKeyCode::OemPeriod,
                        #[cfg(windows)]
                        186,
                        #[cfg(unix)]
                        39,
                    ),
                    "-" => (
                        VirtualKeyCode::OemMinus,
                        #[cfg(windows)]
                        189,
                        #[cfg(unix)]
                        12,
                    ),
                    "_" => (
                        VirtualKeyCode::OemMinus,
                        #[cfg(windows)]
                        189,
                        #[cfg(unix)]
                        74,
                    ),
                    "+" => (
                        VirtualKeyCode::OemPlus,
                        #[cfg(windows)]
                        187,
                        #[cfg(unix)]
                        78,
                    ),
                    "=" => (
                        VirtualKeyCode::OemPlus,
                        #[cfg(windows)]
                        187,
                        #[cfg(unix)]
                        78,
                    ),
                    "\\" => (
                        VirtualKeyCode::Oem5,
                        #[cfg(windows)]
                        220,
                        #[cfg(unix)]
                        43,
                    ),
                    "|" => (
                        VirtualKeyCode::Oem5,
                        #[cfg(windows)]
                        220,
                        #[cfg(unix)]
                        43,
                    ),
                    "`" => (
                        VirtualKeyCode::Oem3,
                        #[cfg(windows)]
                        192,
                        #[cfg(unix)]
                        41,
                    ),
                    "?" => (
                        VirtualKeyCode::Oem2,
                        #[cfg(windows)]
                        191,
                        #[cfg(unix)]
                        53,
                    ),
                    "/" => (
                        VirtualKeyCode::Oem2,
                        #[cfg(windows)]
                        191,
                        #[cfg(unix)]
                        53,
                    ),
                    ">" => (
                        VirtualKeyCode::Oem102,
                        #[cfg(windows)]
                        226,
                        #[cfg(unix)]
                        52,
                    ),
                    "<" => (
                        VirtualKeyCode::Oem102,
                        #[cfg(windows)]
                        226,
                        #[cfg(unix)]
                        52,
                    ),
                    "[" => (
                        VirtualKeyCode::Oem4,
                        #[cfg(windows)]
                        219,
                        #[cfg(unix)]
                        26,
                    ),
                    "]" => (
                        VirtualKeyCode::Oem6,
                        #[cfg(windows)]
                        221,
                        #[cfg(unix)]
                        27,
                    ),
                    _ => return None,
                },
                keyboard::Key::Unidentified => return None,
            };
            (text, virtual_key, native_key)
        } else {
            return None;
        }
    };

    let modifiers = event::KeyEventModifiers {
        alt: modifiers.alt(),
        ctrl: modifiers.control(),
        meta: modifiers.logo(),
        shift: modifiers.shift(),
    };

    let ty = if modifiers.ctrl {
        event::KeyEventType::RawKeyDown
    } else if !text.is_empty() && text.is_ascii() && press == KeyPress::Press {
        event::KeyEventType::Char
    } else {
        match press {
            KeyPress::Press => event::KeyEventType::RawKeyDown,
            KeyPress::Unpress => event::KeyEventType::KeyUp,
        }
    };

    let creation_info = KeyEventCreationInfo {
        ty,
        modifiers,
        virtual_key_code: virtual_key,
        native_key_code: native_key,
        text: text.as_str(),
        unmodified_text: if let Some(keyboard::Key::Character(char)) = modified_key {
            &char.to_string()
        } else {
            text.as_str()
        },
        is_keypad: false,
        is_auto_repeat: false,
        is_system_key: false,
    };

    event::KeyEvent::new(creation_info).ok()
}
