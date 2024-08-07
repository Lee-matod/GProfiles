pub enum KeyboardKey {
    Backspace,
    Tab,
    Enter,
    Escape,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Insert,
    Delete,
    Numrow0,
    Numrow1,
    Numrow2,
    Numrow3,
    Numrow4,
    Numrow5,
    Numrow6,
    Numrow7,
    Numrow8,
    Numrow9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LSuper,
    RSuper,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumLock,
    ScrollLock,
    CapsLock,
    LShift,
    RShift,
    LControl,
    RControl,
    LAlt,
    RAlt,

    BrowserBack,
    BrowserForward,
    BrowserRefresh,

    VolumeMute,
    VolumeDown,
    VolumeUp,

    MediaNextTrack,
    MediaPrevTrack,
    MediaStop,
    MediaPlayPause,

    Backquote,
    Slash,
    Backslash,
    Comma,
    Period,
    Minus,
    Quote,
    Semicolon,
    LBracket,
    RBracket,
    Equal,
}

impl PartialEq for KeyboardKey {
    fn eq(&self, other: &Self) -> bool {
        u64::from(self) == u64::from(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl From<u64> for KeyboardKey {
    fn from(value: u64) -> KeyboardKey {
        match value {
            0x08 => KeyboardKey::Backspace,
            0x09 => KeyboardKey::Tab,
            0x0D => KeyboardKey::Enter,
            0x1B => KeyboardKey::Escape,
            0x20 => KeyboardKey::Space,
            0x21 => KeyboardKey::PageUp,
            0x22 => KeyboardKey::PageDown,
            0x23 => KeyboardKey::End,
            0x24 => KeyboardKey::Home,
            0x25 => KeyboardKey::Left,
            0x26 => KeyboardKey::Up,
            0x27 => KeyboardKey::Right,
            0x28 => KeyboardKey::Down,
            0x2D => KeyboardKey::Insert,
            0x2E => KeyboardKey::Delete,
            0x30 => KeyboardKey::Numrow0,
            0x31 => KeyboardKey::Numrow1,
            0x32 => KeyboardKey::Numrow2,
            0x33 => KeyboardKey::Numrow3,
            0x34 => KeyboardKey::Numrow4,
            0x35 => KeyboardKey::Numrow5,
            0x36 => KeyboardKey::Numrow6,
            0x37 => KeyboardKey::Numrow7,
            0x38 => KeyboardKey::Numrow8,
            0x39 => KeyboardKey::Numrow9,
            0x41 => KeyboardKey::A,
            0x42 => KeyboardKey::B,
            0x43 => KeyboardKey::C,
            0x44 => KeyboardKey::D,
            0x45 => KeyboardKey::E,
            0x46 => KeyboardKey::F,
            0x47 => KeyboardKey::G,
            0x48 => KeyboardKey::H,
            0x49 => KeyboardKey::I,
            0x4A => KeyboardKey::J,
            0x4B => KeyboardKey::K,
            0x4C => KeyboardKey::L,
            0x4D => KeyboardKey::M,
            0x4E => KeyboardKey::N,
            0x4F => KeyboardKey::O,
            0x50 => KeyboardKey::P,
            0x51 => KeyboardKey::Q,
            0x52 => KeyboardKey::R,
            0x53 => KeyboardKey::S,
            0x54 => KeyboardKey::T,
            0x55 => KeyboardKey::U,
            0x56 => KeyboardKey::V,
            0x57 => KeyboardKey::W,
            0x58 => KeyboardKey::X,
            0x59 => KeyboardKey::Y,
            0x5A => KeyboardKey::Z,
            0x5B => KeyboardKey::LSuper,
            0x5C => KeyboardKey::RSuper,
            0x60 => KeyboardKey::Numpad0,
            0x61 => KeyboardKey::Numpad1,
            0x62 => KeyboardKey::Numpad2,
            0x63 => KeyboardKey::Numpad3,
            0x64 => KeyboardKey::Numpad4,
            0x65 => KeyboardKey::Numpad5,
            0x66 => KeyboardKey::Numpad6,
            0x67 => KeyboardKey::Numpad7,
            0x68 => KeyboardKey::Numpad8,
            0x69 => KeyboardKey::Numpad9,
            0x70 => KeyboardKey::F1,
            0x71 => KeyboardKey::F2,
            0x72 => KeyboardKey::F3,
            0x73 => KeyboardKey::F4,
            0x74 => KeyboardKey::F5,
            0x75 => KeyboardKey::F6,
            0x76 => KeyboardKey::F7,
            0x77 => KeyboardKey::F8,
            0x78 => KeyboardKey::F9,
            0x79 => KeyboardKey::F10,
            0x7A => KeyboardKey::F11,
            0x7B => KeyboardKey::F12,
            0x7C => KeyboardKey::F13,
            0x7D => KeyboardKey::F14,
            0x7E => KeyboardKey::F15,
            0x7F => KeyboardKey::F16,
            0x80 => KeyboardKey::F17,
            0x81 => KeyboardKey::F18,
            0x82 => KeyboardKey::F19,
            0x83 => KeyboardKey::F20,
            0x84 => KeyboardKey::F21,
            0x85 => KeyboardKey::F22,
            0x86 => KeyboardKey::F23,
            0x87 => KeyboardKey::F24,
            0x90 => KeyboardKey::NumLock,
            0x91 => KeyboardKey::ScrollLock,
            0x14 => KeyboardKey::CapsLock,
            0xA0 => KeyboardKey::LShift,
            0xA1 => KeyboardKey::RShift,
            0xA2 => KeyboardKey::LControl,
            0xA3 => KeyboardKey::RControl,
            0xA4 => KeyboardKey::LAlt,
            0xA5 => KeyboardKey::RAlt,
            0xA6 => KeyboardKey::BrowserBack,
            0xA7 => KeyboardKey::BrowserForward,
            0xA8 => KeyboardKey::BrowserRefresh,
            0xAD => KeyboardKey::VolumeMute,
            0xAE => KeyboardKey::VolumeDown,
            0xAF => KeyboardKey::VolumeUp,
            0xB0 => KeyboardKey::MediaNextTrack,
            0xB1 => KeyboardKey::MediaPrevTrack,
            0xB2 => KeyboardKey::MediaStop,
            0xB3 => KeyboardKey::MediaPlayPause,
            0xC0 => KeyboardKey::Backquote,
            0xBF => KeyboardKey::Slash,
            0xDC => KeyboardKey::Backslash,
            0xBC => KeyboardKey::Comma,
            0xBE => KeyboardKey::Period,
            0xBD => KeyboardKey::Minus,
            0xDE => KeyboardKey::Quote,
            0xBA => KeyboardKey::Semicolon,
            0xDB => KeyboardKey::LBracket,
            0xDD => KeyboardKey::RBracket,
            0xBB => KeyboardKey::Equal,
            _ => KeyboardKey::Escape,
        }
    }
}

impl From<&KeyboardKey> for u64 {
    fn from(value: &KeyboardKey) -> u64 {
        match value {
            KeyboardKey::Backspace => 0x08,
            KeyboardKey::Tab => 0x09,
            KeyboardKey::Enter => 0x0D,
            KeyboardKey::Escape => 0x1B,
            KeyboardKey::Space => 0x20,
            KeyboardKey::PageUp => 0x21,
            KeyboardKey::PageDown => 0x22,
            KeyboardKey::End => 0x23,
            KeyboardKey::Home => 0x24,
            KeyboardKey::Left => 0x25,
            KeyboardKey::Up => 0x26,
            KeyboardKey::Right => 0x27,
            KeyboardKey::Down => 0x28,
            KeyboardKey::Insert => 0x2D,
            KeyboardKey::Delete => 0x2E,
            KeyboardKey::Numrow0 => 0x30,
            KeyboardKey::Numrow1 => 0x31,
            KeyboardKey::Numrow2 => 0x32,
            KeyboardKey::Numrow3 => 0x33,
            KeyboardKey::Numrow4 => 0x34,
            KeyboardKey::Numrow5 => 0x35,
            KeyboardKey::Numrow6 => 0x36,
            KeyboardKey::Numrow7 => 0x37,
            KeyboardKey::Numrow8 => 0x38,
            KeyboardKey::Numrow9 => 0x39,
            KeyboardKey::A => 0x41,
            KeyboardKey::B => 0x42,
            KeyboardKey::C => 0x43,
            KeyboardKey::D => 0x44,
            KeyboardKey::E => 0x45,
            KeyboardKey::F => 0x46,
            KeyboardKey::G => 0x47,
            KeyboardKey::H => 0x48,
            KeyboardKey::I => 0x49,
            KeyboardKey::J => 0x4A,
            KeyboardKey::K => 0x4B,
            KeyboardKey::L => 0x4C,
            KeyboardKey::M => 0x4D,
            KeyboardKey::N => 0x4E,
            KeyboardKey::O => 0x4F,
            KeyboardKey::P => 0x50,
            KeyboardKey::Q => 0x51,
            KeyboardKey::R => 0x52,
            KeyboardKey::S => 0x53,
            KeyboardKey::T => 0x54,
            KeyboardKey::U => 0x55,
            KeyboardKey::V => 0x56,
            KeyboardKey::W => 0x57,
            KeyboardKey::X => 0x58,
            KeyboardKey::Y => 0x59,
            KeyboardKey::Z => 0x5A,
            KeyboardKey::LSuper => 0x5B,
            KeyboardKey::RSuper => 0x5C,
            KeyboardKey::Numpad0 => 0x60,
            KeyboardKey::Numpad1 => 0x61,
            KeyboardKey::Numpad2 => 0x62,
            KeyboardKey::Numpad3 => 0x63,
            KeyboardKey::Numpad4 => 0x64,
            KeyboardKey::Numpad5 => 0x65,
            KeyboardKey::Numpad6 => 0x66,
            KeyboardKey::Numpad7 => 0x67,
            KeyboardKey::Numpad8 => 0x68,
            KeyboardKey::Numpad9 => 0x69,
            KeyboardKey::F1 => 0x70,
            KeyboardKey::F2 => 0x71,
            KeyboardKey::F3 => 0x72,
            KeyboardKey::F4 => 0x73,
            KeyboardKey::F5 => 0x74,
            KeyboardKey::F6 => 0x75,
            KeyboardKey::F7 => 0x76,
            KeyboardKey::F8 => 0x77,
            KeyboardKey::F9 => 0x78,
            KeyboardKey::F10 => 0x79,
            KeyboardKey::F11 => 0x7A,
            KeyboardKey::F12 => 0x7B,
            KeyboardKey::F13 => 0x7C,
            KeyboardKey::F14 => 0x7D,
            KeyboardKey::F15 => 0x7E,
            KeyboardKey::F16 => 0x7F,
            KeyboardKey::F17 => 0x80,
            KeyboardKey::F18 => 0x81,
            KeyboardKey::F19 => 0x82,
            KeyboardKey::F20 => 0x83,
            KeyboardKey::F21 => 0x84,
            KeyboardKey::F22 => 0x85,
            KeyboardKey::F23 => 0x86,
            KeyboardKey::F24 => 0x87,
            KeyboardKey::NumLock => 0x90,
            KeyboardKey::ScrollLock => 0x91,
            KeyboardKey::CapsLock => 0x14,
            KeyboardKey::LShift => 0xA0,
            KeyboardKey::RShift => 0xA1,
            KeyboardKey::LControl => 0xA2,
            KeyboardKey::RControl => 0xA3,
            KeyboardKey::LAlt => 0xA4,
            KeyboardKey::RAlt => 0xA5,
            KeyboardKey::BrowserBack => 0xA6,
            KeyboardKey::BrowserForward => 0xA7,
            KeyboardKey::BrowserRefresh => 0xA8,
            KeyboardKey::VolumeMute => 0xAD,
            KeyboardKey::VolumeDown => 0xAE,
            KeyboardKey::VolumeUp => 0xAF,
            KeyboardKey::MediaNextTrack => 0xB0,
            KeyboardKey::MediaPrevTrack => 0xB1,
            KeyboardKey::MediaStop => 0xB2,
            KeyboardKey::MediaPlayPause => 0xB3,
            KeyboardKey::Backquote => 0xC0,
            KeyboardKey::Slash => 0xBF,
            KeyboardKey::Backslash => 0xDC,
            KeyboardKey::Comma => 0xBC,
            KeyboardKey::Period => 0xBE,
            KeyboardKey::Minus => 0xBD,
            KeyboardKey::Quote => 0xDE,
            KeyboardKey::Semicolon => 0xBA,
            KeyboardKey::LBracket => 0xDB,
            KeyboardKey::RBracket => 0xDD,
            KeyboardKey::Equal => 0xBB,
        }
    }
}

impl std::fmt::Display for KeyboardKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KeyboardKey::Backspace => "Backspace",
                KeyboardKey::Tab => "Tab",
                KeyboardKey::Enter => "Enter",
                KeyboardKey::Escape => "NONE",
                KeyboardKey::Space => "Space",
                KeyboardKey::PageUp => "PageUp",
                KeyboardKey::PageDown => "PageDown",
                KeyboardKey::End => "End",
                KeyboardKey::Home => "Home",
                KeyboardKey::Left => "Left",
                KeyboardKey::Up => "Up",
                KeyboardKey::Right => "Right",
                KeyboardKey::Down => "Down",
                KeyboardKey::Insert => "Insert",
                KeyboardKey::Delete => "Delete",
                KeyboardKey::Numrow0 => "0",
                KeyboardKey::Numrow1 => "1",
                KeyboardKey::Numrow2 => "2",
                KeyboardKey::Numrow3 => "3",
                KeyboardKey::Numrow4 => "4",
                KeyboardKey::Numrow5 => "5",
                KeyboardKey::Numrow6 => "6",
                KeyboardKey::Numrow7 => "7",
                KeyboardKey::Numrow8 => "8",
                KeyboardKey::Numrow9 => "9",
                KeyboardKey::A => "a",
                KeyboardKey::B => "b",
                KeyboardKey::C => "c",
                KeyboardKey::D => "d",
                KeyboardKey::E => "e",
                KeyboardKey::F => "f",
                KeyboardKey::G => "g",
                KeyboardKey::H => "h",
                KeyboardKey::I => "i",
                KeyboardKey::J => "j",
                KeyboardKey::K => "k",
                KeyboardKey::L => "l",
                KeyboardKey::M => "m",
                KeyboardKey::N => "n",
                KeyboardKey::O => "o",
                KeyboardKey::P => "p",
                KeyboardKey::Q => "q",
                KeyboardKey::R => "r",
                KeyboardKey::S => "s",
                KeyboardKey::T => "t",
                KeyboardKey::U => "u",
                KeyboardKey::V => "v",
                KeyboardKey::W => "w",
                KeyboardKey::X => "x",
                KeyboardKey::Y => "y",
                KeyboardKey::Z => "z",
                KeyboardKey::LSuper => "LeftWindows",
                KeyboardKey::RSuper => "RightWindows",
                KeyboardKey::Numpad0 => "NumPad0",
                KeyboardKey::Numpad1 => "NumPad1",
                KeyboardKey::Numpad2 => "NumPad2",
                KeyboardKey::Numpad3 => "NumPad3",
                KeyboardKey::Numpad4 => "NumPad4",
                KeyboardKey::Numpad5 => "NumPad5",
                KeyboardKey::Numpad6 => "NumPad6",
                KeyboardKey::Numpad7 => "NumPad7",
                KeyboardKey::Numpad8 => "NumPad8",
                KeyboardKey::Numpad9 => "NumPad9",
                KeyboardKey::F1 => "F1",
                KeyboardKey::F2 => "F2",
                KeyboardKey::F3 => "F3",
                KeyboardKey::F4 => "F4",
                KeyboardKey::F5 => "F5",
                KeyboardKey::F6 => "F6",
                KeyboardKey::F7 => "F7",
                KeyboardKey::F8 => "F8",
                KeyboardKey::F9 => "F9",
                KeyboardKey::F10 => "F10",
                KeyboardKey::F11 => "F11",
                KeyboardKey::F12 => "F12",
                KeyboardKey::F13 => "F13",
                KeyboardKey::F14 => "F14",
                KeyboardKey::F15 => "F15",
                KeyboardKey::F16 => "F16",
                KeyboardKey::F17 => "F17",
                KeyboardKey::F18 => "F18",
                KeyboardKey::F19 => "F19",
                KeyboardKey::F20 => "F20",
                KeyboardKey::F21 => "F21",
                KeyboardKey::F22 => "F22",
                KeyboardKey::F23 => "F23",
                KeyboardKey::F24 => "F24",
                KeyboardKey::NumLock => "NumLock",
                KeyboardKey::ScrollLock => "ScrollLock",
                KeyboardKey::CapsLock => "CapsLock",
                KeyboardKey::LShift => "LeftShift",
                KeyboardKey::RShift => "RightShift",
                KeyboardKey::LControl => "LeftControl",
                KeyboardKey::RControl => "RightControl",
                KeyboardKey::LAlt => "LeftAlt",
                KeyboardKey::RAlt => "RightAlt",
                KeyboardKey::BrowserBack => "Back",
                KeyboardKey::BrowserForward => "Forward",
                KeyboardKey::BrowserRefresh => "Refresh",
                KeyboardKey::VolumeMute => "VolumeMute",
                KeyboardKey::VolumeDown => "VolumeDown",
                KeyboardKey::VolumeUp => "VolumeUp",
                KeyboardKey::MediaNextTrack => "MediaNext",
                KeyboardKey::MediaPrevTrack => "MediaPrevious",
                KeyboardKey::MediaStop => "MediaStop",
                KeyboardKey::MediaPlayPause => "MediaPlay",
                KeyboardKey::Backquote => "`",
                KeyboardKey::Slash => "/",
                KeyboardKey::Backslash => "\\",
                KeyboardKey::Comma => ",",
                KeyboardKey::Period => ".",
                KeyboardKey::Minus => "-",
                KeyboardKey::Quote => "'",
                KeyboardKey::Semicolon => ";",
                KeyboardKey::LBracket => "[",
                KeyboardKey::RBracket => "]",
                KeyboardKey::Equal => "=",
            }
        )
    }
}

impl std::fmt::Debug for KeyboardKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KeyboardKey::Backspace => "Backspace",
                KeyboardKey::Tab => "Tab",
                KeyboardKey::Enter => "Enter",
                KeyboardKey::Escape => "NONE",
                KeyboardKey::Space => "Space",
                KeyboardKey::PageUp => "PageUp",
                KeyboardKey::PageDown => "PageDown",
                KeyboardKey::End => "End",
                KeyboardKey::Home => "Home",
                KeyboardKey::Left => "Left",
                KeyboardKey::Up => "Up",
                KeyboardKey::Right => "Right",
                KeyboardKey::Down => "Down",
                KeyboardKey::Insert => "Insert",
                KeyboardKey::Delete => "Delete",
                KeyboardKey::Numrow0 => "0",
                KeyboardKey::Numrow1 => "1",
                KeyboardKey::Numrow2 => "2",
                KeyboardKey::Numrow3 => "3",
                KeyboardKey::Numrow4 => "4",
                KeyboardKey::Numrow5 => "5",
                KeyboardKey::Numrow6 => "6",
                KeyboardKey::Numrow7 => "7",
                KeyboardKey::Numrow8 => "8",
                KeyboardKey::Numrow9 => "9",
                KeyboardKey::A => "a",
                KeyboardKey::B => "b",
                KeyboardKey::C => "c",
                KeyboardKey::D => "d",
                KeyboardKey::E => "e",
                KeyboardKey::F => "f",
                KeyboardKey::G => "g",
                KeyboardKey::H => "h",
                KeyboardKey::I => "i",
                KeyboardKey::J => "j",
                KeyboardKey::K => "k",
                KeyboardKey::L => "l",
                KeyboardKey::M => "m",
                KeyboardKey::N => "n",
                KeyboardKey::O => "o",
                KeyboardKey::P => "p",
                KeyboardKey::Q => "q",
                KeyboardKey::R => "r",
                KeyboardKey::S => "s",
                KeyboardKey::T => "t",
                KeyboardKey::U => "u",
                KeyboardKey::V => "v",
                KeyboardKey::W => "w",
                KeyboardKey::X => "x",
                KeyboardKey::Y => "y",
                KeyboardKey::Z => "z",
                KeyboardKey::LSuper => "LeftWindows",
                KeyboardKey::RSuper => "RightWindows",
                KeyboardKey::Numpad0 => "NumPad0",
                KeyboardKey::Numpad1 => "NumPad1",
                KeyboardKey::Numpad2 => "NumPad2",
                KeyboardKey::Numpad3 => "NumPad3",
                KeyboardKey::Numpad4 => "NumPad4",
                KeyboardKey::Numpad5 => "NumPad5",
                KeyboardKey::Numpad6 => "NumPad6",
                KeyboardKey::Numpad7 => "NumPad7",
                KeyboardKey::Numpad8 => "NumPad8",
                KeyboardKey::Numpad9 => "NumPad9",
                KeyboardKey::F1 => "F1",
                KeyboardKey::F2 => "F2",
                KeyboardKey::F3 => "F3",
                KeyboardKey::F4 => "F4",
                KeyboardKey::F5 => "F5",
                KeyboardKey::F6 => "F6",
                KeyboardKey::F7 => "F7",
                KeyboardKey::F8 => "F8",
                KeyboardKey::F9 => "F9",
                KeyboardKey::F10 => "F10",
                KeyboardKey::F11 => "F11",
                KeyboardKey::F12 => "F12",
                KeyboardKey::F13 => "F13",
                KeyboardKey::F14 => "F14",
                KeyboardKey::F15 => "F15",
                KeyboardKey::F16 => "F16",
                KeyboardKey::F17 => "F17",
                KeyboardKey::F18 => "F18",
                KeyboardKey::F19 => "F19",
                KeyboardKey::F20 => "F20",
                KeyboardKey::F21 => "F21",
                KeyboardKey::F22 => "F22",
                KeyboardKey::F23 => "F23",
                KeyboardKey::F24 => "F24",
                KeyboardKey::NumLock => "NumLock",
                KeyboardKey::ScrollLock => "ScrollLock",
                KeyboardKey::CapsLock => "CapsLock",
                KeyboardKey::LShift => "LeftShift",
                KeyboardKey::RShift => "RightShift",
                KeyboardKey::LControl => "LeftControl",
                KeyboardKey::RControl => "RightControl",
                KeyboardKey::LAlt => "LeftAlt",
                KeyboardKey::RAlt => "RightAlt",
                KeyboardKey::BrowserBack => "Back",
                KeyboardKey::BrowserForward => "Forward",
                KeyboardKey::BrowserRefresh => "Refresh",
                KeyboardKey::VolumeMute => "VolumeMute",
                KeyboardKey::VolumeDown => "VolumeDown",
                KeyboardKey::VolumeUp => "VolumeUp",
                KeyboardKey::MediaNextTrack => "MediaNext",
                KeyboardKey::MediaPrevTrack => "MediaPrevious",
                KeyboardKey::MediaStop => "MediaStop",
                KeyboardKey::MediaPlayPause => "MediaPlay",
                KeyboardKey::Backquote => "Backquote",
                KeyboardKey::Slash => "Slash",
                KeyboardKey::Backslash => "Backslash",
                KeyboardKey::Comma => "Comma",
                KeyboardKey::Period => "Period",
                KeyboardKey::Minus => "Minus",
                KeyboardKey::Quote => "Quote",
                KeyboardKey::Semicolon => "Semicolon",
                KeyboardKey::LBracket => "LeftBracket",
                KeyboardKey::RBracket => "RightBracket",
                KeyboardKey::Equal => "Equals",
            }
        )
    }
}
