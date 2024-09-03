use rdev::{Event, EventType, Key};

/// Key listener for pause or exit key
pub fn global_listener(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => {
            println!("{:?}", key);
            println!("{:?}", event.name);
        }
        EventType::ButtonPress(button) => {
            println!("{:?}", button);
            println!("{:?}", event.name);
        }
        _ => (),
    }
}

pub trait ParseKey {
    fn parse(s: &str) -> Option<Key>;
}

impl ParseKey for Key {
    fn parse(s: &str) -> Option<Key> {
        match s {
            "Alt" => Some(Key::Alt),
            "AltGr" => Some(Key::AltGr),
            "Backspace" => Some(Key::Backspace),
            "CapsLock" => Some(Key::CapsLock),
            "ControlLeft" => Some(Key::ControlLeft),
            "ControlRight" => Some(Key::ControlRight),
            "Delete" => Some(Key::Delete),
            "DownArrow" => Some(Key::DownArrow),
            "End" => Some(Key::End),
            "Escape" => Some(Key::Escape),
            "F1" => Some(Key::F1),
            "F2" => Some(Key::F2),
            "F3" => Some(Key::F3),
            "F4" => Some(Key::F4),
            "F5" => Some(Key::F5),
            "F6" => Some(Key::F6),
            "F7" => Some(Key::F7),
            "F8" => Some(Key::F8),
            "F9" => Some(Key::F9),
            "F10" => Some(Key::F10),
            "F11" => Some(Key::F11),
            "F12" => Some(Key::F12),
            "Home" => Some(Key::Home),
            "LeftArrow" => Some(Key::LeftArrow),
            "MetaLeft" => Some(Key::MetaLeft),
            "MetaRight" => Some(Key::MetaRight),
            "PageDown" => Some(Key::PageDown),
            "PageUp" => Some(Key::PageUp),
            "Return" => Some(Key::Return),
            "RightArrow" => Some(Key::RightArrow),
            "ShiftLeft" => Some(Key::ShiftLeft),
            "ShiftRight" => Some(Key::ShiftRight),
            "Space" => Some(Key::Space),
            "Tab" => Some(Key::Tab),
            "UpArrow" => Some(Key::UpArrow),
            "PrintScreen" => Some(Key::PrintScreen),
            "ScrollLock" => Some(Key::ScrollLock),
            "Pause" => Some(Key::Pause),
            "NumLock" => Some(Key::NumLock),
            "BackQuote" => Some(Key::BackQuote),
            "1" => Some(Key::Num1),
            "2" => Some(Key::Num2),
            "3" => Some(Key::Num3),
            "4" => Some(Key::Num4),
            "5" => Some(Key::Num5),
            "6" => Some(Key::Num6),
            "7" => Some(Key::Num7),
            "8" => Some(Key::Num8),
            "9" => Some(Key::Num9),
            "0" => Some(Key::Num0),
            "Minus" => Some(Key::Minus),
            "Equal" => Some(Key::Equal),
            "Q" => Some(Key::KeyQ),
            "W" => Some(Key::KeyW),
            "E" => Some(Key::KeyE),
            "R" => Some(Key::KeyR),
            "T" => Some(Key::KeyT),
            "Y" => Some(Key::KeyY),
            "U" => Some(Key::KeyU),
            "I" => Some(Key::KeyI),
            "O" => Some(Key::KeyO),
            "P" => Some(Key::KeyP),
            "LeftBracket" => Some(Key::LeftBracket),
            "RightBracket" => Some(Key::RightBracket),
            "A" => Some(Key::KeyA),
            "S" => Some(Key::KeyS),
            "D" => Some(Key::KeyD),
            "F" => Some(Key::KeyF),
            "G" => Some(Key::KeyG),
            "H" => Some(Key::KeyH),
            "J" => Some(Key::KeyJ),
            "K" => Some(Key::KeyK),
            "L" => Some(Key::KeyL),
            ";" => Some(Key::SemiColon),
            "\"" => Some(Key::Quote),
            "\\" => Some(Key::BackSlash),
            "IntlBackslash" => Some(Key::IntlBackslash),
            "Z" => Some(Key::KeyZ),
            "X" => Some(Key::KeyX),
            "C" => Some(Key::KeyC),
            "V" => Some(Key::KeyV),
            "B" => Some(Key::KeyB),
            "N" => Some(Key::KeyN),
            "M" => Some(Key::KeyM),
            "," => Some(Key::Comma),
            "." => Some(Key::Dot),
            "/" => Some(Key::Slash),
            "Insert" => Some(Key::Insert),
            "NumReturn" => Some(Key::KpReturn),
            "NumMinus" => Some(Key::KpMinus),
            "NumPlus" => Some(Key::KpPlus),
            "NumMultiply" => Some(Key::KpMultiply),
            "NumDivide" => Some(Key::KpDivide),
            "Num0" => Some(Key::Kp0),
            "Num1" => Some(Key::Kp1),
            "Num2" => Some(Key::Kp2),
            "Num3" => Some(Key::Kp3),
            "Num4" => Some(Key::Kp4),
            "Num5" => Some(Key::Kp5),
            "Num6" => Some(Key::Kp6),
            "Num7" => Some(Key::Kp7),
            "Num8" => Some(Key::Kp8),
            "Num9" => Some(Key::Kp9),
            "NumDelete" => Some(Key::KpDelete),
            "Function" => Some(Key::Function),
            other => {
                if other.starts_with("Unknown(") {
                    let number: u32 = other
                        .strip_prefix("Unknown(")
                        .unwrap()
                        .strip_suffix(')')?
                        .trim()
                        .parse()
                        .ok()?;
                    Some(Key::Unknown(number))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::keyboard::ParseKey;
    use rdev::Key;

    #[test]
    fn test_parse_key() {
        assert_eq!(Key::parse("F11").unwrap(), Key::F11);
    }
}
