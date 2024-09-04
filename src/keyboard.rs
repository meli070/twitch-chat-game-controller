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
        match s.to_lowercase().as_str() {
            "alt" => Some(Key::Alt),
            "altGr" => Some(Key::AltGr),
            "backspace" => Some(Key::Backspace),
            "capslock" => Some(Key::CapsLock),
            "controlleft" | "control" | "ctrl" => Some(Key::ControlLeft),
            "controlright" => Some(Key::ControlRight),
            "delete" => Some(Key::Delete),
            "downarrow" => Some(Key::DownArrow),
            "end" => Some(Key::End),
            "escape" => Some(Key::Escape),
            "f1" => Some(Key::F1),
            "f2" => Some(Key::F2),
            "f3" => Some(Key::F3),
            "f4" => Some(Key::F4),
            "f5" => Some(Key::F5),
            "f6" => Some(Key::F6),
            "f7" => Some(Key::F7),
            "f8" => Some(Key::F8),
            "f9" => Some(Key::F9),
            "f10" => Some(Key::F10),
            "f11" => Some(Key::F11),
            "f12" => Some(Key::F12),
            "home" => Some(Key::Home),
            "leftarrow" | "<-" | "left" => Some(Key::LeftArrow),
            "metaleft" | "windowsleft" | "windows" | "meta" => Some(Key::MetaLeft),
            "metaright" | "windowsright" => Some(Key::MetaRight),
            "pagedown" => Some(Key::PageDown),
            "pageup" => Some(Key::PageUp),
            "return" => Some(Key::Return),
            "rightarrow" | "->" | "right" => Some(Key::RightArrow),
            "shiftleft" | "shift" => Some(Key::ShiftLeft),
            "shiftright" => Some(Key::ShiftRight),
            "space" | " " => Some(Key::Space),
            "tab" => Some(Key::Tab),
            "uparrow" | "up" => Some(Key::UpArrow),
            "printscreen" | "print" => Some(Key::PrintScreen),
            "scrolllock" | "roll" => Some(Key::ScrollLock),
            "pause" => Some(Key::Pause),
            "numlock" => Some(Key::NumLock),
            "backquote" => Some(Key::BackQuote),
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
            "minus" | "-" => Some(Key::Minus),
            "equal" | "=" => Some(Key::Equal),
            "q" => Some(Key::KeyQ),
            "w" => Some(Key::KeyW),
            "e" => Some(Key::KeyE),
            "r" => Some(Key::KeyR),
            "t" => Some(Key::KeyT),
            "y" => Some(Key::KeyY),
            "u" => Some(Key::KeyU),
            "i" => Some(Key::KeyI),
            "o" => Some(Key::KeyO),
            "p" => Some(Key::KeyP),
            "leftbracket" | "(" => Some(Key::LeftBracket),
            "rightbracket" | ")" => Some(Key::RightBracket),
            "a" => Some(Key::KeyA),
            "s" => Some(Key::KeyS),
            "d" => Some(Key::KeyD),
            "f" => Some(Key::KeyF),
            "g" => Some(Key::KeyG),
            "h" => Some(Key::KeyH),
            "j" => Some(Key::KeyJ),
            "k" => Some(Key::KeyK),
            "l" => Some(Key::KeyL),
            "semicolon" | ";" => Some(Key::SemiColon),
            "quote" | "´" => Some(Key::Quote),
            "backslash" | "\\" => Some(Key::BackSlash),
            "intlbackslash" => Some(Key::IntlBackslash),
            "z" => Some(Key::KeyZ),
            "x" => Some(Key::KeyX),
            "c" => Some(Key::KeyC),
            "v" => Some(Key::KeyV),
            "b" => Some(Key::KeyB),
            "n" => Some(Key::KeyN),
            "m" => Some(Key::KeyM),
            "comma" | "," => Some(Key::Comma),
            "dot" | "." => Some(Key::Dot),
            "slash" | "/" => Some(Key::Slash),
            "insert" => Some(Key::Insert),
            "numreturn" => Some(Key::KpReturn),
            "numminus" => Some(Key::KpMinus),
            "numplus" => Some(Key::KpPlus),
            "nummultiply" => Some(Key::KpMultiply),
            "numdivide" => Some(Key::KpDivide),
            "num0" => Some(Key::Kp0),
            "num1" => Some(Key::Kp1),
            "num2" => Some(Key::Kp2),
            "num3" => Some(Key::Kp3),
            "num4" => Some(Key::Kp4),
            "num5" => Some(Key::Kp5),
            "num6" => Some(Key::Kp6),
            "num7" => Some(Key::Kp7),
            "num8" => Some(Key::Kp8),
            "num9" => Some(Key::Kp9),
            "numdelete" => Some(Key::KpDelete),
            "function" => Some(Key::Function),
            other => {
                if other.starts_with("unknown(") {
                    let number: u32 = other
                        .strip_prefix("unknown(")
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
