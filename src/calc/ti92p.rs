use uinput::event::{
    keyboard::{Key, KeyPad},
    Keyboard,
};

pub const KEY_MATRIX_LEN: usize = 10;

pub const KEY_TO_KEY_MAP: [((u8, u8), Keyboard); 78] = [
    ((0, 7), Keyboard::Key(Key::Down)),           // Down
    ((0, 6), Keyboard::Key(Key::Right)),          // Right
    ((0, 5), Keyboard::Key(Key::Up)),             // Up
    ((0, 4), Keyboard::Key(Key::Left)),           // Left
    ((0, 3), Keyboard::Key(Key::LeftMeta)),       // Hand
    ((0, 2), Keyboard::Key(Key::LeftShift)),      // Shift
    ((0, 1), Keyboard::Key(Key::LeftControl)),    // Diamnd
    ((0, 0), Keyboard::Key(Key::LeftAlt)),        // 2nd
    ((1, 7), Keyboard::Key(Key::_3)),             // 3
    ((1, 6), Keyboard::Key(Key::_2)),             // 2
    ((1, 5), Keyboard::Key(Key::_1)),             // 1
    ((1, 4), Keyboard::Key(Key::F8)),             // F8
    ((1, 3), Keyboard::Key(Key::W)),              // W
    ((1, 2), Keyboard::Key(Key::S)),              // S
    ((1, 1), Keyboard::Key(Key::Z)),              // Z
    ((2, 7), Keyboard::Key(Key::_6)),             // 6
    ((2, 6), Keyboard::Key(Key::_5)),             // 5
    ((2, 5), Keyboard::Key(Key::_4)),             // 4
    ((2, 4), Keyboard::Key(Key::F3)),             // F3
    ((2, 3), Keyboard::Key(Key::E)),              // E
    ((2, 2), Keyboard::Key(Key::D)),              // D
    ((2, 1), Keyboard::Key(Key::X)),              // X
    ((3, 7), Keyboard::Key(Key::_9)),             // 9
    ((3, 6), Keyboard::Key(Key::_8)),             // 8
    ((3, 5), Keyboard::Key(Key::_7)),             // 7
    ((3, 4), Keyboard::Key(Key::F7)),             // F7
    ((3, 3), Keyboard::Key(Key::R)),              // R
    ((3, 2), Keyboard::Key(Key::F)),              // F
    ((3, 1), Keyboard::Key(Key::C)),              // C
    ((3, 0), Keyboard::Key(Key::F24)),            // STO
    ((4, 7), Keyboard::Key(Key::Comma)),          // ,
    ((4, 6), Keyboard::Key(Key::RightBrace)),     // )
    ((4, 5), Keyboard::Key(Key::LeftBrace)),      // (
    ((4, 4), Keyboard::Key(Key::F2)),             // F2
    ((4, 3), Keyboard::Key(Key::T)),              // T
    ((4, 2), Keyboard::Key(Key::G)),              // G
    ((4, 1), Keyboard::Key(Key::V)),              // V
    ((4, 0), Keyboard::Key(Key::Space)),          // Space
    ((5, 7), Keyboard::Key(Key::F23)),            // TAN
    ((5, 6), Keyboard::Key(Key::F22)),            // COS
    ((5, 5), Keyboard::Key(Key::F21)),            // SIN
    ((5, 4), Keyboard::Key(Key::F6)),             // F6
    ((5, 3), Keyboard::Key(Key::Y)),              // Y
    ((5, 2), Keyboard::Key(Key::H)),              // H
    ((5, 1), Keyboard::Key(Key::B)),              // B
    ((5, 0), Keyboard::Key(Key::Slash)),          // /
    ((6, 7), Keyboard::Key(Key::P)),              // P
    ((6, 6), Keyboard::Key(Key::LineFeed)),       // ENTER2
    ((6, 5), Keyboard::Key(Key::F20)),            // LN
    ((6, 4), Keyboard::Key(Key::F1)),             // F1
    ((6, 3), Keyboard::Key(Key::U)),              // U
    ((6, 2), Keyboard::Key(Key::J)),              // J
    ((6, 1), Keyboard::Key(Key::N)),              // N
    ((6, 0), Keyboard::Key(Key::F19)),            // ^
    ((7, 7), Keyboard::KeyPad(KeyPad::Asterisk)), // *
    ((7, 6), Keyboard::Key(Key::F18)),            // APPS
    ((7, 5), Keyboard::Key(Key::F17)),            // CLEAR
    ((7, 4), Keyboard::Key(Key::F5)),             // F5
    ((7, 3), Keyboard::Key(Key::I)),              // I
    ((7, 2), Keyboard::Key(Key::K)),              // K
    ((7, 1), Keyboard::Key(Key::M)),              // M
    ((7, 0), Keyboard::Key(Key::Equal)),          // =
    ((8, 6), Keyboard::Key(Key::Esc)),            // ESC
    ((8, 5), Keyboard::Key(Key::F16)),            // MODE
    ((8, 4), Keyboard::KeyPad(KeyPad::Plus)),     // +
    ((8, 3), Keyboard::Key(Key::O)),              // O
    ((8, 2), Keyboard::Key(Key::L)),              // L
    ((8, 1), Keyboard::Key(Key::F15)),            // θ
    ((8, 0), Keyboard::Key(Key::BackSpace)),      // BckSpc
    ((9, 7), Keyboard::KeyPad(KeyPad::Minus)),    // (-)
    ((9, 6), Keyboard::Key(Key::Dot)),            // .
    ((9, 5), Keyboard::Key(Key::_0)),             // 0
    ((9, 4), Keyboard::Key(Key::F4)),             // F4
    ((9, 3), Keyboard::Key(Key::Q)),              // Q
    ((9, 2), Keyboard::Key(Key::A)),              // A
    ((9, 1), Keyboard::KeyPad(KeyPad::Enter)),    // ENTER1
    ((9, 0), Keyboard::Key(Key::Minus)),          // -
    ((1, 0), Keyboard::Key(Key::SysRq)), // ON, snuck into an empty spot in the existing key matrix
];
