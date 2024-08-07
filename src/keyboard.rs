use strum::{EnumIter, IntoEnumIterator};
use uinput::{
    event::{
        keyboard::{Key, KeyPad},
        Keyboard,
    },
    Device, Event, Result,
};

#[derive(Eq, Hash, PartialEq, Debug, EnumIter)]
pub enum CalcKey {
    CursorDown,
    CursorRight,
    CursorUp,
    CursorLeft,
    Hand,
    Shift,
    Diamond,
    _2nd,
    _3,
    _2,
    _1,
    F8,
    W,
    S,
    Z,
    _6,
    _5,
    _4,
    F3,
    E,
    D,
    X,
    _9,
    _8,
    _7,
    F7,
    R,
    F,
    C,
    STO,
    Comma,
    RightParenthesis,
    LeftParenthesis,
    F2,
    T,
    G,
    V,
    Space,
    TAN,
    COS,
    SIN,
    F6,
    Y,
    H,
    B,
    Division,
    P,
    ENTER2,
    LN,
    F1,
    U,
    J,
    N,
    Exponentiation,
    Multiplication,
    APPS,
    CLEAR,
    F5,
    I,
    K,
    M,
    Equals,
    ESC,
    MODE,
    Addition,
    O,
    L,
    Theta,
    BckSpc,
    Negative,
    Period,
    _0,
    F4,
    Q,
    A,
    ENTER1,
    Subtraction,
    ON,
}

pub struct VirtualKeyboard {
    pub handle: Device,
}
impl VirtualKeyboard {
    pub fn new() -> Result<VirtualKeyboard> {
        let mut virtual_kbd = uinput::default()?.name("i68apollo")?;

        for calc_key in CalcKey::iter() {
            virtual_kbd = virtual_kbd.event(Event::Keyboard(map_key_to_key(&calc_key)))?;
        }

        let handle = virtual_kbd.create()?;

        Ok(VirtualKeyboard { handle })
    }

    pub fn press_key(&mut self, key: &CalcKey) {
        let key_event = map_key_to_key(key);

        self.handle.press(&key_event).expect("Can't press key!");
    }

    pub fn release_key(&mut self, key: &CalcKey) {
        let key_event = map_key_to_key(key);

        self.handle.release(&key_event).expect("Can't release key!");
    }

    pub fn sync(&mut self) {
        self.handle.synchronize().expect("Can't sync!");
    }
}

fn map_key_to_key(key: &CalcKey) -> Keyboard {
    match key {
        CalcKey::CursorDown => Keyboard::Key(Key::Down),
        CalcKey::CursorRight => Keyboard::Key(Key::Right),
        CalcKey::CursorUp => Keyboard::Key(Key::Up),
        CalcKey::CursorLeft => Keyboard::Key(Key::Left),
        CalcKey::Hand => Keyboard::Key(Key::LeftMeta),
        CalcKey::Shift => Keyboard::Key(Key::LeftShift),
        CalcKey::Diamond => Keyboard::Key(Key::LeftControl),
        CalcKey::_2nd => Keyboard::Key(Key::LeftAlt),
        CalcKey::_3 => Keyboard::Key(Key::_3),
        CalcKey::_2 => Keyboard::Key(Key::_2),
        CalcKey::_1 => Keyboard::Key(Key::_1),
        CalcKey::F8 => Keyboard::Key(Key::F8),
        CalcKey::W => Keyboard::Key(Key::W),
        CalcKey::S => Keyboard::Key(Key::S),
        CalcKey::Z => Keyboard::Key(Key::Z),
        CalcKey::_6 => Keyboard::Key(Key::_6),
        CalcKey::_5 => Keyboard::Key(Key::_5),
        CalcKey::_4 => Keyboard::Key(Key::_4),
        CalcKey::F3 => Keyboard::Key(Key::F3),
        CalcKey::E => Keyboard::Key(Key::E),
        CalcKey::D => Keyboard::Key(Key::D),
        CalcKey::X => Keyboard::Key(Key::X),
        CalcKey::_9 => Keyboard::Key(Key::_9),
        CalcKey::_8 => Keyboard::Key(Key::_8),
        CalcKey::_7 => Keyboard::Key(Key::_7),
        CalcKey::F7 => Keyboard::Key(Key::F7),
        CalcKey::R => Keyboard::Key(Key::R),
        CalcKey::F => Keyboard::Key(Key::F),
        CalcKey::C => Keyboard::Key(Key::C),
        CalcKey::STO => Keyboard::Key(Key::F24),
        CalcKey::Comma => Keyboard::Key(Key::Comma),
        CalcKey::RightParenthesis => Keyboard::Key(Key::RightBrace),
        CalcKey::LeftParenthesis => Keyboard::Key(Key::LeftBrace),
        CalcKey::F2 => Keyboard::Key(Key::F2),
        CalcKey::T => Keyboard::Key(Key::T),
        CalcKey::G => Keyboard::Key(Key::G),
        CalcKey::V => Keyboard::Key(Key::V),
        CalcKey::Space => Keyboard::Key(Key::Space),
        CalcKey::TAN => Keyboard::Key(Key::F23),
        CalcKey::COS => Keyboard::Key(Key::F22),
        CalcKey::SIN => Keyboard::Key(Key::F21),
        CalcKey::F6 => Keyboard::Key(Key::F6),
        CalcKey::Y => Keyboard::Key(Key::Y),
        CalcKey::H => Keyboard::Key(Key::H),
        CalcKey::B => Keyboard::Key(Key::B),
        CalcKey::Division => Keyboard::Key(Key::Slash),
        CalcKey::P => Keyboard::Key(Key::P),
        CalcKey::ENTER2 => Keyboard::Key(Key::LineFeed),
        CalcKey::LN => Keyboard::Key(Key::F20),
        CalcKey::F1 => Keyboard::Key(Key::F1),
        CalcKey::U => Keyboard::Key(Key::U),
        CalcKey::J => Keyboard::Key(Key::J),
        CalcKey::N => Keyboard::Key(Key::N),
        CalcKey::Exponentiation => Keyboard::Key(Key::F19),
        CalcKey::Multiplication => Keyboard::KeyPad(KeyPad::Asterisk),
        CalcKey::APPS => Keyboard::Key(Key::F18),
        CalcKey::CLEAR => Keyboard::Key(Key::F17),
        CalcKey::F5 => Keyboard::Key(Key::F5),
        CalcKey::I => Keyboard::Key(Key::I),
        CalcKey::K => Keyboard::Key(Key::K),
        CalcKey::M => Keyboard::Key(Key::M),
        CalcKey::Equals => Keyboard::Key(Key::Equal),
        CalcKey::ESC => Keyboard::Key(Key::Esc),
        CalcKey::MODE => Keyboard::Key(Key::F16),
        CalcKey::Addition => Keyboard::KeyPad(KeyPad::Plus),
        CalcKey::O => Keyboard::Key(Key::O),
        CalcKey::L => Keyboard::Key(Key::L),
        CalcKey::Theta => Keyboard::Key(Key::F15),
        CalcKey::BckSpc => Keyboard::Key(Key::BackSpace),
        CalcKey::Negative => Keyboard::KeyPad(KeyPad::Minus),
        CalcKey::Period => Keyboard::Key(Key::Dot),
        CalcKey::_0 => Keyboard::Key(Key::_0),
        CalcKey::F4 => Keyboard::Key(Key::F4),
        CalcKey::Q => Keyboard::Key(Key::Q),
        CalcKey::A => Keyboard::Key(Key::A),
        CalcKey::ENTER1 => Keyboard::KeyPad(KeyPad::Enter),
        CalcKey::Subtraction => Keyboard::Key(Key::Minus),
        CalcKey::ON => Keyboard::Key(Key::SysRq),
    }
}
