use crate::driver::keyboard::KeyCode;

pub fn putchar(c: u8) {
    unsafe {
        core::ptr::write_volatile(crate::config::SERIAL_PORT, c);
    }
}

pub fn get_time() -> u64 {
    unsafe {
        let t1 = core::ptr::read_volatile(crate::config::RTC_PORT_HIGH);
        let t2 = core::ptr::read_volatile(crate::config::RTC_PORT_LOW);
        ((t1 as u64) << 32) | (t2 as u64)
    }
}

mod keyboard {
    #[rustfmt::skip]
    #[allow(non_camel_case_types)]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub enum KeyCode {
        ESCAPE = 1, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
        GRAVE, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, NUM0,
        MINUS, EQUALS, BACKSPACE,
        TAB, Q, W, E, R, T, Y, U, I, O, P, LEFTBRACKET, RIGHTBRACKET, BACKSLASH,
        CAPSLOCK, A, S, D, F, G, H, J, K, L, SEMICOLON, APOSTROPHE, RETURN,
        LSHIFT, Z, X, C, V, B, N, M, COMMA, PERIOD, SLASH, RSHIFT,
        LCTRL, APPLICATION, LALT, SPACE, RALT, RCTRL,
        UP, DOWN, LEFT, RIGHT, INSERT, DELETE, HOME, END, PAGEUP, PAGEDOWN,
    }

    use core::convert::TryFrom;

    impl TryFrom<u32> for KeyCode {
        type Error = ();

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            match value {
                1 => Ok(KeyCode::ESCAPE),
                2 => Ok(KeyCode::F1),
                3 => Ok(KeyCode::F2),
                4 => Ok(KeyCode::F3),
                5 => Ok(KeyCode::F4),
                6 => Ok(KeyCode::F5),
                7 => Ok(KeyCode::F6),
                8 => Ok(KeyCode::F7),
                9 => Ok(KeyCode::F8),
                10 => Ok(KeyCode::F9),
                11 => Ok(KeyCode::F10),
                12 => Ok(KeyCode::F11),
                13 => Ok(KeyCode::F12),
                14 => Ok(KeyCode::GRAVE),
                15 => Ok(KeyCode::NUM1),
                16 => Ok(KeyCode::NUM2),
                17 => Ok(KeyCode::NUM3),
                18 => Ok(KeyCode::NUM4),
                19 => Ok(KeyCode::NUM5),
                20 => Ok(KeyCode::NUM6),
                21 => Ok(KeyCode::NUM7),
                22 => Ok(KeyCode::NUM8),
                23 => Ok(KeyCode::NUM9),
                24 => Ok(KeyCode::NUM0),
                25 => Ok(KeyCode::MINUS),
                26 => Ok(KeyCode::EQUALS),
                27 => Ok(KeyCode::BACKSPACE),
                28 => Ok(KeyCode::TAB),
                29 => Ok(KeyCode::Q),
                30 => Ok(KeyCode::W),
                31 => Ok(KeyCode::E),
                32 => Ok(KeyCode::R),
                33 => Ok(KeyCode::T),
                34 => Ok(KeyCode::Y),
                35 => Ok(KeyCode::U),
                36 => Ok(KeyCode::I),
                37 => Ok(KeyCode::O),
                38 => Ok(KeyCode::P),
                39 => Ok(KeyCode::LEFTBRACKET),
                40 => Ok(KeyCode::RIGHTBRACKET),
                41 => Ok(KeyCode::BACKSLASH),
                42 => Ok(KeyCode::CAPSLOCK),
                43 => Ok(KeyCode::A),
                44 => Ok(KeyCode::S),
                45 => Ok(KeyCode::D),
                46 => Ok(KeyCode::F),
                47 => Ok(KeyCode::G),
                48 => Ok(KeyCode::H),
                49 => Ok(KeyCode::J),
                50 => Ok(KeyCode::K),
                51 => Ok(KeyCode::L),
                52 => Ok(KeyCode::SEMICOLON),
                53 => Ok(KeyCode::APOSTROPHE),
                54 => Ok(KeyCode::RETURN),
                55 => Ok(KeyCode::LSHIFT),
                56 => Ok(KeyCode::Z),
                57 => Ok(KeyCode::X),
                58 => Ok(KeyCode::C),
                59 => Ok(KeyCode::V),
                60 => Ok(KeyCode::B),
                61 => Ok(KeyCode::N),
                62 => Ok(KeyCode::M),
                63 => Ok(KeyCode::COMMA),
                64 => Ok(KeyCode::PERIOD),
                65 => Ok(KeyCode::SLASH),
                66 => Ok(KeyCode::RSHIFT),
                67 => Ok(KeyCode::LCTRL),
                68 => Ok(KeyCode::APPLICATION),
                69 => Ok(KeyCode::LALT),
                70 => Ok(KeyCode::SPACE),
                71 => Ok(KeyCode::RALT),
                72 => Ok(KeyCode::RCTRL),
                73 => Ok(KeyCode::UP),
                74 => Ok(KeyCode::DOWN),
                75 => Ok(KeyCode::LEFT),
                76 => Ok(KeyCode::RIGHT),
                77 => Ok(KeyCode::INSERT),
                78 => Ok(KeyCode::DELETE),
                79 => Ok(KeyCode::HOME),
                80 => Ok(KeyCode::END),
                81 => Ok(KeyCode::PAGEUP),
                82 => Ok(KeyCode::PAGEDOWN),
                _ => Err(()),
            }
        }
    }

    // 也可以实现 From<KeyCode> for u32 用于反向转换
    impl From<KeyCode> for u32 {
        fn from(key: KeyCode) -> Self {
            key as u32
        }
    }

    pub const KEYDOWN_MASK: u32 = 0x8000;
}

pub fn get_key() -> Option<(KeyCode, bool)> {
    let key = unsafe { core::ptr::read_volatile(crate::config::KBD_PORT) };

    let is_down = key & keyboard::KEYDOWN_MASK != 0;
    let key_code = key & !keyboard::KEYDOWN_MASK;

    KeyCode::try_from(key_code).ok().map(|code| (code, is_down))
}
