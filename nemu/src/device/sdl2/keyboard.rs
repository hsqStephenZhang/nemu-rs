mod scancode {
    use std::str::FromStr;

    use num_derive::FromPrimitive;
    use sdl2::sys::SDL_Scancode;
    use strum_macros::{EnumIter, EnumString, IntoStaticStr};

    #[repr(i32)]
    #[derive(
        Copy, Clone, Eq, PartialEq, Hash, Debug, EnumString, IntoStaticStr, EnumIter, FromPrimitive,
    )]
    pub enum MyScancode {
        A = SDL_Scancode::SDL_SCANCODE_A as i32,
        B = SDL_Scancode::SDL_SCANCODE_B as i32,
        C = SDL_Scancode::SDL_SCANCODE_C as i32,
        D = SDL_Scancode::SDL_SCANCODE_D as i32,
        E = SDL_Scancode::SDL_SCANCODE_E as i32,
        F = SDL_Scancode::SDL_SCANCODE_F as i32,
        G = SDL_Scancode::SDL_SCANCODE_G as i32,
        H = SDL_Scancode::SDL_SCANCODE_H as i32,
        I = SDL_Scancode::SDL_SCANCODE_I as i32,
        J = SDL_Scancode::SDL_SCANCODE_J as i32,
        K = SDL_Scancode::SDL_SCANCODE_K as i32,
        L = SDL_Scancode::SDL_SCANCODE_L as i32,
        M = SDL_Scancode::SDL_SCANCODE_M as i32,
        N = SDL_Scancode::SDL_SCANCODE_N as i32,
        O = SDL_Scancode::SDL_SCANCODE_O as i32,
        P = SDL_Scancode::SDL_SCANCODE_P as i32,
        Q = SDL_Scancode::SDL_SCANCODE_Q as i32,
        R = SDL_Scancode::SDL_SCANCODE_R as i32,
        S = SDL_Scancode::SDL_SCANCODE_S as i32,
        T = SDL_Scancode::SDL_SCANCODE_T as i32,
        U = SDL_Scancode::SDL_SCANCODE_U as i32,
        V = SDL_Scancode::SDL_SCANCODE_V as i32,
        W = SDL_Scancode::SDL_SCANCODE_W as i32,
        X = SDL_Scancode::SDL_SCANCODE_X as i32,
        Y = SDL_Scancode::SDL_SCANCODE_Y as i32,
        Z = SDL_Scancode::SDL_SCANCODE_Z as i32,
        NUM1 = SDL_Scancode::SDL_SCANCODE_1 as i32,
        NUM2 = SDL_Scancode::SDL_SCANCODE_2 as i32,
        NUM3 = SDL_Scancode::SDL_SCANCODE_3 as i32,
        NUM4 = SDL_Scancode::SDL_SCANCODE_4 as i32,
        NUM5 = SDL_Scancode::SDL_SCANCODE_5 as i32,
        NUM6 = SDL_Scancode::SDL_SCANCODE_6 as i32,
        NUM7 = SDL_Scancode::SDL_SCANCODE_7 as i32,
        NUM8 = SDL_Scancode::SDL_SCANCODE_8 as i32,
        NUM9 = SDL_Scancode::SDL_SCANCODE_9 as i32,
        NUM0 = SDL_Scancode::SDL_SCANCODE_0 as i32,
        RETURN = SDL_Scancode::SDL_SCANCODE_RETURN as i32,
        ESCAPE = SDL_Scancode::SDL_SCANCODE_ESCAPE as i32,
        BACKSPACE = SDL_Scancode::SDL_SCANCODE_BACKSPACE as i32,
        TAB = SDL_Scancode::SDL_SCANCODE_TAB as i32,
        SPACE = SDL_Scancode::SDL_SCANCODE_SPACE as i32,
        MINUS = SDL_Scancode::SDL_SCANCODE_MINUS as i32,
        EQUALS = SDL_Scancode::SDL_SCANCODE_EQUALS as i32,
        LEFTBRACKET = SDL_Scancode::SDL_SCANCODE_LEFTBRACKET as i32,
        RIGHTBRACKET = SDL_Scancode::SDL_SCANCODE_RIGHTBRACKET as i32,
        BACKSLASH = SDL_Scancode::SDL_SCANCODE_BACKSLASH as i32,
        NONUSHASH = SDL_Scancode::SDL_SCANCODE_NONUSHASH as i32,
        SEMICOLON = SDL_Scancode::SDL_SCANCODE_SEMICOLON as i32,
        APOSTROPHE = SDL_Scancode::SDL_SCANCODE_APOSTROPHE as i32,
        GRAVE = SDL_Scancode::SDL_SCANCODE_GRAVE as i32,
        COMMA = SDL_Scancode::SDL_SCANCODE_COMMA as i32,
        PERIOD = SDL_Scancode::SDL_SCANCODE_PERIOD as i32,
        SLASH = SDL_Scancode::SDL_SCANCODE_SLASH as i32,
        CAPSLOCK = SDL_Scancode::SDL_SCANCODE_CAPSLOCK as i32,
        F1 = SDL_Scancode::SDL_SCANCODE_F1 as i32,
        F2 = SDL_Scancode::SDL_SCANCODE_F2 as i32,
        F3 = SDL_Scancode::SDL_SCANCODE_F3 as i32,
        F4 = SDL_Scancode::SDL_SCANCODE_F4 as i32,
        F5 = SDL_Scancode::SDL_SCANCODE_F5 as i32,
        F6 = SDL_Scancode::SDL_SCANCODE_F6 as i32,
        F7 = SDL_Scancode::SDL_SCANCODE_F7 as i32,
        F8 = SDL_Scancode::SDL_SCANCODE_F8 as i32,
        F9 = SDL_Scancode::SDL_SCANCODE_F9 as i32,
        F10 = SDL_Scancode::SDL_SCANCODE_F10 as i32,
        F11 = SDL_Scancode::SDL_SCANCODE_F11 as i32,
        F12 = SDL_Scancode::SDL_SCANCODE_F12 as i32,
        PRINTSCREEN = SDL_Scancode::SDL_SCANCODE_PRINTSCREEN as i32,
        SCROLLLOCK = SDL_Scancode::SDL_SCANCODE_SCROLLLOCK as i32,
        PAUSE = SDL_Scancode::SDL_SCANCODE_PAUSE as i32,
        INSERT = SDL_Scancode::SDL_SCANCODE_INSERT as i32,
        HOME = SDL_Scancode::SDL_SCANCODE_HOME as i32,
        PAGEUP = SDL_Scancode::SDL_SCANCODE_PAGEUP as i32,
        DELETE = SDL_Scancode::SDL_SCANCODE_DELETE as i32,
        END = SDL_Scancode::SDL_SCANCODE_END as i32,
        PAGEDOWN = SDL_Scancode::SDL_SCANCODE_PAGEDOWN as i32,
        RIGHT = SDL_Scancode::SDL_SCANCODE_RIGHT as i32,
        LEFT = SDL_Scancode::SDL_SCANCODE_LEFT as i32,
        DOWN = SDL_Scancode::SDL_SCANCODE_DOWN as i32,
        UP = SDL_Scancode::SDL_SCANCODE_UP as i32,
        NUMLOCKCLEAR = SDL_Scancode::SDL_SCANCODE_NUMLOCKCLEAR as i32,
        KPDIVIDE = SDL_Scancode::SDL_SCANCODE_KP_DIVIDE as i32,
        KPMULTIPLY = SDL_Scancode::SDL_SCANCODE_KP_MULTIPLY as i32,
        KPMINUS = SDL_Scancode::SDL_SCANCODE_KP_MINUS as i32,
        KPPLUS = SDL_Scancode::SDL_SCANCODE_KP_PLUS as i32,
        KPENTER = SDL_Scancode::SDL_SCANCODE_KP_ENTER as i32,
        KP1 = SDL_Scancode::SDL_SCANCODE_KP_1 as i32,
        KP2 = SDL_Scancode::SDL_SCANCODE_KP_2 as i32,
        KP3 = SDL_Scancode::SDL_SCANCODE_KP_3 as i32,
        KP4 = SDL_Scancode::SDL_SCANCODE_KP_4 as i32,
        KP5 = SDL_Scancode::SDL_SCANCODE_KP_5 as i32,
        KP6 = SDL_Scancode::SDL_SCANCODE_KP_6 as i32,
        KP7 = SDL_Scancode::SDL_SCANCODE_KP_7 as i32,
        KP8 = SDL_Scancode::SDL_SCANCODE_KP_8 as i32,
        KP9 = SDL_Scancode::SDL_SCANCODE_KP_9 as i32,
        KP0 = SDL_Scancode::SDL_SCANCODE_KP_0 as i32,
        KPPERIOD = SDL_Scancode::SDL_SCANCODE_KP_PERIOD as i32,
        NONUSBACKSLASH = SDL_Scancode::SDL_SCANCODE_NONUSBACKSLASH as i32,
        APPLICATION = SDL_Scancode::SDL_SCANCODE_APPLICATION as i32,
        POWER = SDL_Scancode::SDL_SCANCODE_POWER as i32,
        KPEQUALS = SDL_Scancode::SDL_SCANCODE_KP_EQUALS as i32,
        F13 = SDL_Scancode::SDL_SCANCODE_F13 as i32,
        F14 = SDL_Scancode::SDL_SCANCODE_F14 as i32,
        F15 = SDL_Scancode::SDL_SCANCODE_F15 as i32,
        F16 = SDL_Scancode::SDL_SCANCODE_F16 as i32,
        F17 = SDL_Scancode::SDL_SCANCODE_F17 as i32,
        F18 = SDL_Scancode::SDL_SCANCODE_F18 as i32,
        F19 = SDL_Scancode::SDL_SCANCODE_F19 as i32,
        F20 = SDL_Scancode::SDL_SCANCODE_F20 as i32,
        F21 = SDL_Scancode::SDL_SCANCODE_F21 as i32,
        F22 = SDL_Scancode::SDL_SCANCODE_F22 as i32,
        F23 = SDL_Scancode::SDL_SCANCODE_F23 as i32,
        F24 = SDL_Scancode::SDL_SCANCODE_F24 as i32,
        EXECUTE = SDL_Scancode::SDL_SCANCODE_EXECUTE as i32,
        HELP = SDL_Scancode::SDL_SCANCODE_HELP as i32,
        MENU = SDL_Scancode::SDL_SCANCODE_MENU as i32,
        SELECT = SDL_Scancode::SDL_SCANCODE_SELECT as i32,
        STOP = SDL_Scancode::SDL_SCANCODE_STOP as i32,
        AGAIN = SDL_Scancode::SDL_SCANCODE_AGAIN as i32,
        UNDO = SDL_Scancode::SDL_SCANCODE_UNDO as i32,
        CUT = SDL_Scancode::SDL_SCANCODE_CUT as i32,
        COPY = SDL_Scancode::SDL_SCANCODE_COPY as i32,
        PASTE = SDL_Scancode::SDL_SCANCODE_PASTE as i32,
        FIND = SDL_Scancode::SDL_SCANCODE_FIND as i32,
        MUTE = SDL_Scancode::SDL_SCANCODE_MUTE as i32,
        VOLUMEUP = SDL_Scancode::SDL_SCANCODE_VOLUMEUP as i32,
        VOLUMEDOWN = SDL_Scancode::SDL_SCANCODE_VOLUMEDOWN as i32,
        KPCOMMA = SDL_Scancode::SDL_SCANCODE_KP_COMMA as i32,
        KPEQUALSAS400 = SDL_Scancode::SDL_SCANCODE_KP_EQUALSAS400 as i32,
        INTERNATIONAL1 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL1 as i32,
        INTERNATIONAL2 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL2 as i32,
        INTERNATIONAL3 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL3 as i32,
        INTERNATIONAL4 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL4 as i32,
        INTERNATIONAL5 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL5 as i32,
        INTERNATIONAL6 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL6 as i32,
        INTERNATIONAL7 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL7 as i32,
        INTERNATIONAL8 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL8 as i32,
        INTERNATIONAL9 = SDL_Scancode::SDL_SCANCODE_INTERNATIONAL9 as i32,
        LANG1 = SDL_Scancode::SDL_SCANCODE_LANG1 as i32,
        LANG2 = SDL_Scancode::SDL_SCANCODE_LANG2 as i32,
        LANG3 = SDL_Scancode::SDL_SCANCODE_LANG3 as i32,
        LANG4 = SDL_Scancode::SDL_SCANCODE_LANG4 as i32,
        LANG5 = SDL_Scancode::SDL_SCANCODE_LANG5 as i32,
        LANG6 = SDL_Scancode::SDL_SCANCODE_LANG6 as i32,
        LANG7 = SDL_Scancode::SDL_SCANCODE_LANG7 as i32,
        LANG8 = SDL_Scancode::SDL_SCANCODE_LANG8 as i32,
        LANG9 = SDL_Scancode::SDL_SCANCODE_LANG9 as i32,
        ALTERASE = SDL_Scancode::SDL_SCANCODE_ALTERASE as i32,
        SYSREQ = SDL_Scancode::SDL_SCANCODE_SYSREQ as i32,
        CANCEL = SDL_Scancode::SDL_SCANCODE_CANCEL as i32,
        CLEAR = SDL_Scancode::SDL_SCANCODE_CLEAR as i32,
        PRIOR = SDL_Scancode::SDL_SCANCODE_PRIOR as i32,
        RETURN2 = SDL_Scancode::SDL_SCANCODE_RETURN2 as i32,
        SEPARATOR = SDL_Scancode::SDL_SCANCODE_SEPARATOR as i32,
        OUT = SDL_Scancode::SDL_SCANCODE_OUT as i32,
        OPER = SDL_Scancode::SDL_SCANCODE_OPER as i32,
        CLEARAGAIN = SDL_Scancode::SDL_SCANCODE_CLEARAGAIN as i32,
        CRSEL = SDL_Scancode::SDL_SCANCODE_CRSEL as i32,
        EXSEL = SDL_Scancode::SDL_SCANCODE_EXSEL as i32,
        KP00 = SDL_Scancode::SDL_SCANCODE_KP_00 as i32,
        KP000 = SDL_Scancode::SDL_SCANCODE_KP_000 as i32,
        THOUSANDSSEPARATOR = SDL_Scancode::SDL_SCANCODE_THOUSANDSSEPARATOR as i32,
        DECIMALSEPARATOR = SDL_Scancode::SDL_SCANCODE_DECIMALSEPARATOR as i32,
        CURRENCYUNIT = SDL_Scancode::SDL_SCANCODE_CURRENCYUNIT as i32,
        CURRENCYSUBUNIT = SDL_Scancode::SDL_SCANCODE_CURRENCYSUBUNIT as i32,
        KPLEFTPAREN = SDL_Scancode::SDL_SCANCODE_KP_LEFTPAREN as i32,
        KPRIGHTPAREN = SDL_Scancode::SDL_SCANCODE_KP_RIGHTPAREN as i32,
        KPLEFTBRACE = SDL_Scancode::SDL_SCANCODE_KP_LEFTBRACE as i32,
        KPRIGHTBRACE = SDL_Scancode::SDL_SCANCODE_KP_RIGHTBRACE as i32,
        KPTAB = SDL_Scancode::SDL_SCANCODE_KP_TAB as i32,
        KPBACKSPACE = SDL_Scancode::SDL_SCANCODE_KP_BACKSPACE as i32,
        KPA = SDL_Scancode::SDL_SCANCODE_KP_A as i32,
        KPB = SDL_Scancode::SDL_SCANCODE_KP_B as i32,
        KPC = SDL_Scancode::SDL_SCANCODE_KP_C as i32,
        KPD = SDL_Scancode::SDL_SCANCODE_KP_D as i32,
        KPE = SDL_Scancode::SDL_SCANCODE_KP_E as i32,
        KPF = SDL_Scancode::SDL_SCANCODE_KP_F as i32,
        KPXOR = SDL_Scancode::SDL_SCANCODE_KP_XOR as i32,
        KPPOWER = SDL_Scancode::SDL_SCANCODE_KP_POWER as i32,
        KPPERCENT = SDL_Scancode::SDL_SCANCODE_KP_PERCENT as i32,
        KPLESS = SDL_Scancode::SDL_SCANCODE_KP_LESS as i32,
        KPGREATER = SDL_Scancode::SDL_SCANCODE_KP_GREATER as i32,
        KPAMPERSAND = SDL_Scancode::SDL_SCANCODE_KP_AMPERSAND as i32,
        KPDBLAMPERSAND = SDL_Scancode::SDL_SCANCODE_KP_DBLAMPERSAND as i32,
        KPVERTICALBAR = SDL_Scancode::SDL_SCANCODE_KP_VERTICALBAR as i32,
        KPDBLVERTICALBAR = SDL_Scancode::SDL_SCANCODE_KP_DBLVERTICALBAR as i32,
        KPCOLON = SDL_Scancode::SDL_SCANCODE_KP_COLON as i32,
        KPHASH = SDL_Scancode::SDL_SCANCODE_KP_HASH as i32,
        KPSPACE = SDL_Scancode::SDL_SCANCODE_KP_SPACE as i32,
        KPAT = SDL_Scancode::SDL_SCANCODE_KP_AT as i32,
        KPEXCLAM = SDL_Scancode::SDL_SCANCODE_KP_EXCLAM as i32,
        KPMEMSTORE = SDL_Scancode::SDL_SCANCODE_KP_MEMSTORE as i32,
        KPMEMRECALL = SDL_Scancode::SDL_SCANCODE_KP_MEMRECALL as i32,
        KPMEMCLEAR = SDL_Scancode::SDL_SCANCODE_KP_MEMCLEAR as i32,
        KPMEMADD = SDL_Scancode::SDL_SCANCODE_KP_MEMADD as i32,
        KPMEMSUBTRACT = SDL_Scancode::SDL_SCANCODE_KP_MEMSUBTRACT as i32,
        KPMEMMULTIPLY = SDL_Scancode::SDL_SCANCODE_KP_MEMMULTIPLY as i32,
        KPMEMDIVIDE = SDL_Scancode::SDL_SCANCODE_KP_MEMDIVIDE as i32,
        KPPLUSMINUS = SDL_Scancode::SDL_SCANCODE_KP_PLUSMINUS as i32,
        KPCLEAR = SDL_Scancode::SDL_SCANCODE_KP_CLEAR as i32,
        KPCLEARENTRY = SDL_Scancode::SDL_SCANCODE_KP_CLEARENTRY as i32,
        KPBINARY = SDL_Scancode::SDL_SCANCODE_KP_BINARY as i32,
        KPOCTAL = SDL_Scancode::SDL_SCANCODE_KP_OCTAL as i32,
        KPDECIMAL = SDL_Scancode::SDL_SCANCODE_KP_DECIMAL as i32,
        KPHEXADECIMAL = SDL_Scancode::SDL_SCANCODE_KP_HEXADECIMAL as i32,
        LCTRL = SDL_Scancode::SDL_SCANCODE_LCTRL as i32,
        LSHIFT = SDL_Scancode::SDL_SCANCODE_LSHIFT as i32,
        LALT = SDL_Scancode::SDL_SCANCODE_LALT as i32,
        LGUI = SDL_Scancode::SDL_SCANCODE_LGUI as i32,
        RCTRL = SDL_Scancode::SDL_SCANCODE_RCTRL as i32,
        RSHIFT = SDL_Scancode::SDL_SCANCODE_RSHIFT as i32,
        RALT = SDL_Scancode::SDL_SCANCODE_RALT as i32,
        RGUI = SDL_Scancode::SDL_SCANCODE_RGUI as i32,
        MODE = SDL_Scancode::SDL_SCANCODE_MODE as i32,
        AUDIONEXT = SDL_Scancode::SDL_SCANCODE_AUDIONEXT as i32,
        AUDIOPREV = SDL_Scancode::SDL_SCANCODE_AUDIOPREV as i32,
        AUDIOSTOP = SDL_Scancode::SDL_SCANCODE_AUDIOSTOP as i32,
        AUDIOPLAY = SDL_Scancode::SDL_SCANCODE_AUDIOPLAY as i32,
        AUDIOMUTE = SDL_Scancode::SDL_SCANCODE_AUDIOMUTE as i32,
        MEDIASELECT = SDL_Scancode::SDL_SCANCODE_MEDIASELECT as i32,
        WWW = SDL_Scancode::SDL_SCANCODE_WWW as i32,
        MAIL = SDL_Scancode::SDL_SCANCODE_MAIL as i32,
        CALCULATOR = SDL_Scancode::SDL_SCANCODE_CALCULATOR as i32,
        COMPUTER = SDL_Scancode::SDL_SCANCODE_COMPUTER as i32,
        ACSEARCH = SDL_Scancode::SDL_SCANCODE_AC_SEARCH as i32,
        ACHOME = SDL_Scancode::SDL_SCANCODE_AC_HOME as i32,
        ACBACK = SDL_Scancode::SDL_SCANCODE_AC_BACK as i32,
        ACFORWARD = SDL_Scancode::SDL_SCANCODE_AC_FORWARD as i32,
        ACSTOP = SDL_Scancode::SDL_SCANCODE_AC_STOP as i32,
        ACREFRESH = SDL_Scancode::SDL_SCANCODE_AC_REFRESH as i32,
        ACBOOKMARKS = SDL_Scancode::SDL_SCANCODE_AC_BOOKMARKS as i32,
        BRIGHTNESSDOWN = SDL_Scancode::SDL_SCANCODE_BRIGHTNESSDOWN as i32,
        BRIGHTNESSUP = SDL_Scancode::SDL_SCANCODE_BRIGHTNESSUP as i32,
        DISPLAYSWITCH = SDL_Scancode::SDL_SCANCODE_DISPLAYSWITCH as i32,
        KBDILLUMTOGGLE = SDL_Scancode::SDL_SCANCODE_KBDILLUMTOGGLE as i32,
        KBDILLUMDOWN = SDL_Scancode::SDL_SCANCODE_KBDILLUMDOWN as i32,
        KBDILLUMUP = SDL_Scancode::SDL_SCANCODE_KBDILLUMUP as i32,
        EJECT = SDL_Scancode::SDL_SCANCODE_EJECT as i32,
        SLEEP = SDL_Scancode::SDL_SCANCODE_SLEEP as i32,
        APP1 = SDL_Scancode::SDL_SCANCODE_APP1 as i32,
        APP2 = SDL_Scancode::SDL_SCANCODE_APP2 as i32,
        NUM = SDL_Scancode::SDL_NUM_SCANCODES as i32,
    }

    #[rustfmt::skip]
    #[allow(non_camel_case_types)]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumString, IntoStaticStr, EnumIter, FromPrimitive)]
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

    impl TryFrom<MyScancode> for KeyCode {
        type Error = String;
        fn try_from(value: MyScancode) -> Result<Self, Self::Error> {
            let name: &str = value.into();
            KeyCode::from_str(name).map_err(|_| format!("Invalid key name: {}", name))
        }
    }

    impl From<KeyCode> for MyScancode {
        fn from(value: KeyCode) -> Self {
            let name: &str = value.into();
            MyScancode::from_str(name).unwrap()
        }
    }

    #[test]
    fn test_all_keys() {
        use strum::IntoEnumIterator;

        for key in KeyCode::iter() {
            let name: &str = key.into();
            if name == "_KEY_NONE" {
                continue; // Skip the placeholder key
            }
            // Check if the key can be converted to a Scancode
            let scancode: Result<MyScancode, _> = key.try_into();
            assert!(
                scancode.is_ok(),
                "Key {:?} does not have a valid Scancode",
                key
            );
        }
    }
}

use std::sync::Arc;

use crossbeam_queue::ArrayQueue;
use tracing::{error, info};

pub use self::scancode::*;
use crate::addr_space::IOMap;
use crate::device::AsyncDevice;
use crate::utils::UPSafeCellRaw;

const KEYDOWN_MASK: u32 = 0x8000;
const KEY_QUEUE_LEN: usize = 1024;

#[derive(Debug)]
pub struct KeyboardIOMap;

impl KeyboardIOMap {
    pub fn new_mmio() -> Box<dyn IOMap> {
        Box::new(KeyboardIOMap)
    }
}

impl IOMap for KeyboardIOMap {
    fn read(&self, offset: crate::addr_space::PAddr) -> u64 {
        if offset.as_u64() != 0x0 {
            error!(
                "KeyboardIOMap only supports reading from offset 0x0, actual offset: {}",
                offset.as_u64()
            );
            return 0;
        } else {
            key_dequeue().unwrap_or(0) as u64
        }
    }

    fn write(&mut self, offset: crate::addr_space::PAddr, _: u64) {
        error!(
            "KeyboardIOMap does not support writing, actual offset: {}",
            offset.as_u64(),
        );
    }

    fn len(&self) -> usize {
        1 * std::mem::size_of::<u32>() // 1 u32 for the key event
    }
}

//                    read                write
// keyboard driver <------- KEY_QUEUE <--------- sdl2 event loop(hardware)
lazy_static::lazy_static! {
    static ref KEY_QUEUE: Arc<ArrayQueue<u32>> = Arc::new(ArrayQueue::new(KEY_QUEUE_LEN));
    pub static ref KEY_BOARD_DEVICE: UPSafeCellRaw<KeyboardDevice> = unsafe {
        UPSafeCellRaw::new(KeyboardDevice)
    };
}

// Called from your event loop
pub fn send_key(scancode: MyScancode, is_keydown: bool) {
    let res: Result<KeyCode, _> = scancode.try_into();
    match res {
        Ok(key) => {
            let event_code = key as u32 | if is_keydown { KEYDOWN_MASK } else { 0 };
            KEY_QUEUE.push(event_code).unwrap_or_else(|_| {
                error!("Failed to push key event to queue: {:?}", event_code);
            });
        }
        Err(e) => {
            // Key not found, you can handle this case if needed
            error!("Failed to convert Scancode {:?} to Key: {}", scancode, e);
            return;
        }
    }
}

// Expose this to MMIO/poll handler
pub fn key_dequeue() -> Option<u32> {
    KEY_QUEUE.pop()
}

pub struct KeyboardDevice;

impl AsyncDevice for KeyboardDevice {
    fn name(&self) -> &'static str {
        "keyboard"
    }

    fn period(&self) -> Option<u64> {
        Some(1000)
    }

    fn callback(&self) -> Option<Box<dyn FnMut(u64, u64) + 'static>> {
        Some(Box::new(move |_, _| {
            if let Err(e) = super::SDL_DEVICE.get_mut().poll_events() {
                error!("Failed to handle SDL events: {}", e);
            }
        }))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_crossbeam_queue() {
        send_key(MyScancode::A, true);
        send_key(MyScancode::B, true);

        assert!(key_dequeue().is_some());
        assert!(key_dequeue().is_some());
        assert!(
            key_dequeue().is_none(),
            "Queue should be empty after dequeuing all keys"
        );
    }
}
