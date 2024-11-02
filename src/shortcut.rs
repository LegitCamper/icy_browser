use iced::keyboard::{Key, Modifiers};

/// Defines the allowed modifier keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShortcutModifier {
    Shift,
    Ctrl,
    Alt,
}

/// Configures Widget Keyboard Shortcut
pub struct Shortcut<'a, Message> {
    pub action: Message,
    r#mod: ShortcutModifier,
    key: &'a str,
}

impl<'a, Message> Shortcut<'a, Message> {
    pub fn new(action: Message, r#mod: ShortcutModifier, key: &'a str) -> Self {
        assert!(!key.is_empty());
        Shortcut { action, r#mod, key }
    }

    pub fn is_pressed(&self, key: &Key, modifiers: &Modifiers) -> bool {
        match self.r#mod {
            ShortcutModifier::Shift => {
                if !modifiers.shift() {
                    return false;
                }
            }
            ShortcutModifier::Ctrl => {
                if !modifiers.control() {
                    return false;
                }
            }
            ShortcutModifier::Alt => {
                if !modifiers.alt() {
                    return false;
                }
            }
        }

        if let Key::Character(key) = key {
            if self.key != key.as_str() {
                return false;
            }
        }

        true
    }
}
