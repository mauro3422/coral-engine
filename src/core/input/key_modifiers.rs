// Key Modifiers and Key Combo - Coral Engine
// Low-level input key handling

use std::fmt;
use winit::keyboard::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub logo: bool,
}

impl KeyModifiers {
    pub fn none() -> Self {
        Self::default()
    }
    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            ..Default::default()
        }
    }
    pub fn shift() -> Self {
        Self {
            shift: true,
            ..Default::default()
        }
    }
    pub fn alt() -> Self {
        Self {
            alt: true,
            ..Default::default()
        }
    }
    pub fn logo() -> Self {
        Self {
            logo: true,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.ctrl && !self.shift && !self.alt && !self.logo
    }

    pub fn has_any(&self) -> bool {
        self.ctrl || self.shift || self.alt || self.logo
    }

    pub fn to_u8(&self) -> u8 {
        let mut bits = 0u8;
        if self.ctrl {
            bits |= 1;
        }
        if self.shift {
            bits |= 2;
        }
        if self.alt {
            bits |= 4;
        }
        if self.logo {
            bits |= 8;
        }
        bits
    }

    pub fn from_u8(bits: u8) -> Self {
        Self {
            ctrl: bits & 1 != 0,
            shift: bits & 2 != 0,
            alt: bits & 4 != 0,
            logo: bits & 8 != 0,
        }
    }
}

impl fmt::Display for KeyModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.logo {
            parts.push("Logo");
        }

        if parts.is_empty() {
            write!(f, "None")
        } else {
            write!(f, "{}", parts.join("+"))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub modifiers: KeyModifiers,
    pub key: KeyCode,
}

impl KeyCombo {
    pub fn from_key(key: KeyCode) -> Self {
        Self {
            modifiers: KeyModifiers::none(),
            key,
        }
    }

    pub fn with_ctrl(key: KeyCode) -> Self {
        Self {
            modifiers: KeyModifiers::ctrl(),
            key,
        }
    }

    pub fn with_shift(key: KeyCode) -> Self {
        Self {
            modifiers: KeyModifiers::shift(),
            key,
        }
    }

    pub fn with_alt(key: KeyCode) -> Self {
        Self {
            modifiers: KeyModifiers::alt(),
            key,
        }
    }

    pub fn with_modifiers(modifiers: KeyModifiers, key: KeyCode) -> Self {
        Self { modifiers, key }
    }
}

impl fmt::Display for KeyCombo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.modifiers.ctrl {
            parts.push("Ctrl");
        }
        if self.modifiers.shift {
            parts.push("Shift");
        }
        if self.modifiers.alt {
            parts.push("Alt");
        }
        if self.modifiers.logo {
            parts.push("Logo");
        }

        let key_name = key_code_to_string(self.key);
        parts.push(key_name);

        write!(f, "{}", parts.join("+"))
    }
}

fn key_code_to_string(key: KeyCode) -> &'static str {
    match key {
        KeyCode::KeyW => "W",
        KeyCode::KeyA => "A",
        KeyCode::KeyS => "S",
        KeyCode::KeyD => "D",
        KeyCode::KeyQ => "Q",
        KeyCode::KeyE => "E",
        KeyCode::KeyR => "R",
        KeyCode::KeyG => "G",
        KeyCode::KeyF => "F",
        KeyCode::KeyZ => "Z",
        KeyCode::KeyX => "X",
        KeyCode::KeyC => "C",
        KeyCode::KeyV => "V",
        KeyCode::KeyY => "Y",
        KeyCode::KeyU => "U",
        KeyCode::KeyI => "I",
        KeyCode::KeyO => "O",
        KeyCode::KeyP => "P",
        KeyCode::KeyH => "H",
        KeyCode::KeyJ => "J",
        KeyCode::KeyK => "K",
        KeyCode::KeyL => "L",
        KeyCode::KeyN => "N",
        KeyCode::KeyM => "M",
        KeyCode::KeyB => "B",
        KeyCode::KeyT => "T",

        KeyCode::Digit1 => "1",
        KeyCode::Digit2 => "2",
        KeyCode::Digit3 => "3",

        KeyCode::Space => "Space",
        KeyCode::Tab => "Tab",
        KeyCode::Enter => "Enter",
        KeyCode::Escape => "Esc",
        KeyCode::Backspace => "Backspace",
        KeyCode::Delete => "Del",

        KeyCode::ShiftLeft => "ShiftL",
        KeyCode::ShiftRight => "ShiftR",
        KeyCode::ControlLeft => "CtrlL",
        KeyCode::ControlRight => "CtrlR",
        KeyCode::AltLeft => "AltL",
        KeyCode::AltRight => "AltR",

        KeyCode::Numpad1 => "Num1",
        KeyCode::Numpad2 => "Num2",
        KeyCode::Numpad3 => "Num3",
        KeyCode::Numpad4 => "Num4",
        KeyCode::Numpad5 => "Num5",
        KeyCode::Numpad6 => "Num6",
        KeyCode::Numpad7 => "Num7",
        KeyCode::Numpad8 => "Num8",
        KeyCode::Numpad9 => "Num9",

        KeyCode::ArrowUp => "Up",
        KeyCode::ArrowDown => "Down",
        KeyCode::ArrowLeft => "Left",
        KeyCode::ArrowRight => "Right",

        _ => "?",
    }
}
