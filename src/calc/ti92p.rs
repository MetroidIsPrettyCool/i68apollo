use std::time::Duration;

use crate::{cable::Cable, keyboard::CalcKey};

use super::CalcHandle;

pub const KEY_MATRIX_LEN: usize = 10;

pub const KEY_TO_KEY_MAP: [((usize, u8), CalcKey); 78] = [
    ((0, 7), CalcKey::CursorDown),
    ((0, 6), CalcKey::CursorRight),
    ((0, 5), CalcKey::CursorUp),
    ((0, 4), CalcKey::CursorLeft),
    ((0, 3), CalcKey::Hand),
    ((0, 2), CalcKey::Shift),
    ((0, 1), CalcKey::Diamond),
    ((0, 0), CalcKey::_2nd),
    ((1, 7), CalcKey::_3),
    ((1, 6), CalcKey::_2),
    ((1, 5), CalcKey::_1),
    ((1, 4), CalcKey::F8),
    ((1, 3), CalcKey::W),
    ((1, 2), CalcKey::S),
    ((1, 1), CalcKey::Z),
    ((2, 7), CalcKey::_6),
    ((2, 6), CalcKey::_5),
    ((2, 5), CalcKey::_4),
    ((2, 4), CalcKey::F3),
    ((2, 3), CalcKey::E),
    ((2, 2), CalcKey::D),
    ((2, 1), CalcKey::X),
    ((3, 7), CalcKey::_9),
    ((3, 6), CalcKey::_8),
    ((3, 5), CalcKey::_7),
    ((3, 4), CalcKey::F7),
    ((3, 3), CalcKey::R),
    ((3, 2), CalcKey::F),
    ((3, 1), CalcKey::C),
    ((3, 0), CalcKey::STO),
    ((4, 7), CalcKey::Comma),
    ((4, 6), CalcKey::RightParenthesis),
    ((4, 5), CalcKey::LeftParenthesis),
    ((4, 4), CalcKey::F2),
    ((4, 3), CalcKey::T),
    ((4, 2), CalcKey::G),
    ((4, 1), CalcKey::V),
    ((4, 0), CalcKey::Space),
    ((5, 7), CalcKey::TAN),
    ((5, 6), CalcKey::COS),
    ((5, 5), CalcKey::SIN),
    ((5, 4), CalcKey::F6),
    ((5, 3), CalcKey::Y),
    ((5, 2), CalcKey::H),
    ((5, 1), CalcKey::B),
    ((5, 0), CalcKey::Division),
    ((6, 7), CalcKey::P),
    ((6, 6), CalcKey::ENTER2),
    ((6, 5), CalcKey::LN),
    ((6, 4), CalcKey::F1),
    ((6, 3), CalcKey::U),
    ((6, 2), CalcKey::J),
    ((6, 1), CalcKey::N),
    ((6, 0), CalcKey::Exponentiation),
    ((7, 7), CalcKey::Multiplication),
    ((7, 6), CalcKey::APPS),
    ((7, 5), CalcKey::CLEAR),
    ((7, 4), CalcKey::F5),
    ((7, 3), CalcKey::I),
    ((7, 2), CalcKey::K),
    ((7, 1), CalcKey::M),
    ((7, 0), CalcKey::Equals),
    ((8, 6), CalcKey::ESC),
    ((8, 5), CalcKey::MODE),
    ((8, 4), CalcKey::Addition),
    ((8, 3), CalcKey::O),
    ((8, 2), CalcKey::L),
    ((8, 1), CalcKey::Theta),
    ((8, 0), CalcKey::BckSpc),
    ((9, 7), CalcKey::Negative),
    ((9, 6), CalcKey::Period),
    ((9, 5), CalcKey::_0),
    ((9, 4), CalcKey::F4),
    ((9, 3), CalcKey::Q),
    ((9, 2), CalcKey::A),
    ((9, 1), CalcKey::ENTER1),
    ((9, 0), CalcKey::Subtraction),
    ((1, 0), CalcKey::ON),
];

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct TI92Plus {
    key_matrix: [u8; KEY_MATRIX_LEN],
    prev_key_matrix: [u8; KEY_MATRIX_LEN],
}
impl CalcHandle for TI92Plus {
    fn get_keys(&mut self, cable: &mut Cable) -> Vec<(crate::keyboard::CalcKey, bool)> {
        self.prev_key_matrix.clone_from_slice(&self.key_matrix);
        self.key_matrix
            .copy_from_slice(&cable.read_bytes(KEY_MATRIX_LEN, Duration::from_secs(0)));

        let mut keys = Vec::new();

        for key_to_key_pair in KEY_TO_KEY_MAP {
            let ((row, col), key) = key_to_key_pair;
            if self.key_matrix[row] & (1 << col) != self.prev_key_matrix[row] & (1 << col) {
                if self.key_matrix[row] & (1 << col) == 0 {
                    keys.push((key, false));
                } else {
                    keys.push((key, true));
                }
            }
        }

        keys
    }
}
impl TI92Plus {
    pub fn new() -> TI92Plus {
        TI92Plus {
            key_matrix: [0; KEY_MATRIX_LEN],
            prev_key_matrix: [0; KEY_MATRIX_LEN],
        }
    }
}
