use iced::keyboard::{Event, KeyCode};
use iced::Subscription;
use iced_native::subscription::events_with;
use iced_native::Event as NativeEvent;
use std::collections::{HashMap, HashSet};

const KEY_MAP: [(KeyCode, u8); 16] = [
    (KeyCode::Comma, 0x0),
    (KeyCode::Key7, 0x1),
    (KeyCode::Key8, 0x2),
    (KeyCode::Key9, 0x3),
    (KeyCode::U, 0x4),
    (KeyCode::I, 0x5),
    (KeyCode::O, 0x6),
    (KeyCode::J, 0x7),
    (KeyCode::K, 0x8),
    (KeyCode::L, 0x9),
    (KeyCode::M, 0xA),
    (KeyCode::Period, 0xB),
    (KeyCode::Key0, 0xC),
    (KeyCode::P, 0xD),
    (KeyCode::Semicolon, 0xE),
    (KeyCode::Slash, 0xF),
];

pub struct Keyboard {
    pressed_keys: HashSet<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyboardMessage {
    Press(u8),
    Release(u8),
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn subscription(&self) -> Subscription<KeyboardMessage> {
        events_with(|event, _status| match event {
            NativeEvent::Keyboard(keyboard_event) => match keyboard_event {
                Event::KeyPressed {
                    key_code,
                    modifiers: _,
                } => HashMap::from(KEY_MAP)
                    .get(&key_code)
                    .map(|value| KeyboardMessage::Press(*value)),
                Event::KeyReleased {
                    key_code,
                    modifiers: _,
                } => HashMap::from(KEY_MAP)
                    .get(&key_code)
                    .map(|value| KeyboardMessage::Release(*value)),
                _ => None,
            },
            _ => None,
        })
    }

    pub fn update(&mut self, message: KeyboardMessage) {
        match message {
            KeyboardMessage::Press(value) => {
                self.pressed_keys.insert(value);
            }
            KeyboardMessage::Release(value) => {
                self.pressed_keys.remove(&value);
            }
        }
    }

    pub fn is_pressed(&self, value: u8) -> bool {
        self.pressed_keys.contains(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn initial(value: u8) {
            let keyboard = Keyboard::new();

            assert!(!keyboard.is_pressed(value));
        }

        #[test]
        fn press_initial(value: u8) {
            let mut keyboard = Keyboard::new();

            keyboard.update(KeyboardMessage::Press(value));
            assert!(keyboard.is_pressed(value));
        }

        #[test]
        fn press_twice(value: u8) {
            let mut keyboard = Keyboard::new();

            keyboard.update(KeyboardMessage::Press(value));
            keyboard.update(KeyboardMessage::Press(value));
            assert!(keyboard.is_pressed(value));
        }

        #[test]
        fn press_then_release(value: u8) {
            let mut keyboard = Keyboard::new();

            keyboard.update(KeyboardMessage::Press(value));
            keyboard.update(KeyboardMessage::Release(value));
            assert!(!keyboard.is_pressed(value));
        }
    }
}
