use iced::keyboard::{Key, Modifiers};

use super::widgets::Message;

pub struct ShortcutBuilder(Shortcuts);
impl ShortcutBuilder {
    pub fn new() -> Self {
        ShortcutBuilder(Vec::new())
    }

    pub fn add_shortcut(mut self, shortcut_action: Message, shortcut_keys: Vec<KeyType>) -> Self {
        if self.0.iter().filter(|sc| sc.0 == shortcut_action).count() != 0 {
            panic!("Tried to add a duplicated shortcut");
        }

        // Must have 1 char key
        if shortcut_keys
            .iter()
            .map(|item| {
                if let KeyType::Key(_) = item {
                    return true;
                } else if let KeyType::Modifier(_) = item {
                    return false;
                }
                unreachable!()
            })
            .filter(|item| *item == true)
            .count()
            != 1
        {
            panic!("Shortcuts MUST have ONLY one Charecter key")
        }

        // Must have at least one modifier key
        if shortcut_keys
            .iter()
            .map(|item| {
                if let KeyType::Key(_) = item {
                    return false;
                } else if let KeyType::Modifier(_) = item {
                    return true;
                }
                unreachable!()
            })
            .filter(|item| *item == true)
            .count()
            < 1
        {
            panic!("Shortcuts MUST have at least 1 Modifier key")
        }

        self.0.push((shortcut_action, shortcut_keys));
        self
    }

    pub fn build(self) -> Shortcuts {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShortcutModifier {
    Shift,
    Ctrl,
    Alt,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyType {
    Key(iced::keyboard::Key),
    Modifier(ShortcutModifier),
}
/// Configures Widget Keyboard Shortcut
pub type Shortcut = (Message, Vec<KeyType>);

/// Configures Widget Keyboard Shortcuts
pub type Shortcuts = Vec<Shortcut>;

pub fn check_shortcut(shortcut: &Shortcut, key: &Key, modifiers: &Modifiers) -> bool {
    shortcut
        .1
        .iter()
        .map(|s| match s {
            KeyType::Key(s_key) => {
                if let iced::keyboard::Key::Character(s_char) = s_key {
                    if let iced::keyboard::Key::Character(key_char) = key {
                        key_char == s_char
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            KeyType::Modifier(s_mod) => match s_mod {
                ShortcutModifier::Shift => modifiers.shift(),
                ShortcutModifier::Ctrl => modifiers.control(),
                ShortcutModifier::Alt => modifiers.alt(),
            },
        })
        .all(|s| s == true)
}
