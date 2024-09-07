use std::sync::{
    atomic::{AtomicBool, Ordering},
    LazyLock,
};

use log::{debug, info, warn};
use rdev::{Event, EventType, Key};
use tokio_util::sync::CancellationToken;
use yaml_rust::Yaml;

use crate::exit_on_error::ExitOnError;

static EXIT_SIGNAL: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
static EXIT_REQUEST: AtomicBool = AtomicBool::new(false);
pub static PAUSE: AtomicBool = AtomicBool::new(false);

pub fn get_exit_cancellation_token() -> CancellationToken {
    EXIT_SIGNAL.clone()
}

/// Create event listener for pause or exit key
pub fn create_global_listener(config: &Yaml) -> impl FnMut(Event) + 'static {
    let exit_key = Key::parse(
        config["control_keys"]["exit"]
            .as_str()
            .exit_on_error("Exit key not specified"),
    )
    .exit_on_error("Could not parse exit key!");
    let pause_key = Key::parse(
        config["control_keys"]["pause"]
            .as_str()
            .exit_on_error("Pause key not specified"),
    )
    .exit_on_error("Could not parse pause key!");
    debug!("Global keys parsed.");
    move |event| {
        if let EventType::KeyPress(key) = &event.event_type {
            if key == &exit_key {
                info!("Requesting exit...");
                let previous = EXIT_REQUEST.swap(true, Ordering::Relaxed);
                EXIT_SIGNAL.cancel();
                if previous {
                    warn!("Exit already requested, forcing exit...");
                    std::process::exit(10);
                }
            } else if key == &pause_key {
                PAUSE.fetch_xor(true, Ordering::Relaxed);
            }
        }
    }
}

pub trait ParseKey {
    fn parse(s: &str) -> Option<Key>;
}

impl ParseKey for Key {
    fn parse(s: &str) -> Option<Key> {
        match s.to_lowercase().as_str() {
            "alt" => Some(Key::Alt),
            "altgr" => Some(Key::AltGr),
            "backspace" => Some(Key::Backspace),
            "capslock" => Some(Key::CapsLock),
            "controlleft" | "control" | "ctrl" => Some(Key::ControlLeft),
            "controlright" => Some(Key::ControlRight),
            "delete" | "del" => Some(Key::Delete),
            "downarrow" | "down" => Some(Key::DownArrow),
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
