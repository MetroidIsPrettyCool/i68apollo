use std::time::Duration;

use crate::{cable::Cable, keyboard::CalcKey};

use super::CalcHandle;

pub const KEY_MATRIX_LEN: usize = 7;

pub const KEY_TO_KEY_MAP: [((usize, u8), CalcKey); 50] = [
    ((0, 7), CalcKey::Alpha),
    ((0, 6), CalcKey::Diamond),
    ((0, 5), CalcKey::Shift),
    ((0, 4), CalcKey::_2nd),
    ((0, 3), CalcKey::CursorRight),
    ((0, 2), CalcKey::CursorDown),
    ((0, 1), CalcKey::CursorLeft),
    ((0, 0), CalcKey::CursorUp),
    ((1, 7), CalcKey::F5),
    ((1, 6), CalcKey::CLEAR),
    ((1, 5), CalcKey::Exponentiation),
    ((1, 4), CalcKey::Division),
    ((1, 3), CalcKey::Multiplication),
    ((1, 2), CalcKey::Subtraction),
    ((1, 1), CalcKey::Addition),
    ((1, 0), CalcKey::ENTER1),
    ((2, 7), CalcKey::F4),
    ((2, 6), CalcKey::BckSpc),
    ((2, 5), CalcKey::T),
    ((2, 4), CalcKey::Comma),
    ((2, 3), CalcKey::_9),
    ((2, 2), CalcKey::_6),
    ((2, 1), CalcKey::_3),
    ((2, 0), CalcKey::Negative),
    ((3, 7), CalcKey::F3),
    ((3, 6), CalcKey::CATLG),
    ((3, 5), CalcKey::Z),
    ((3, 4), CalcKey::RightParenthesis),
    ((3, 3), CalcKey::_8),
    ((3, 2), CalcKey::_5),
    ((3, 1), CalcKey::_2),
    ((3, 0), CalcKey::Period),
    ((4, 7), CalcKey::F2),
    ((4, 6), CalcKey::MODE),
    ((4, 5), CalcKey::Y),
    ((4, 4), CalcKey::LeftParenthesis),
    ((4, 3), CalcKey::_7),
    ((4, 2), CalcKey::_4),
    ((4, 1), CalcKey::_1),
    ((4, 0), CalcKey::_0),
    ((5, 7), CalcKey::F1),
    ((5, 6), CalcKey::HOME),
    ((5, 5), CalcKey::X),
    ((5, 4), CalcKey::Equals),
    ((5, 3), CalcKey::Bar),
    ((5, 2), CalcKey::EE),
    ((5, 1), CalcKey::STO),
    ((5, 0), CalcKey::APPS),
    ((6, 0), CalcKey::ESC),
    ((6, 7), CalcKey::ON),
];

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct TI89 {
    key_matrix: [u8; KEY_MATRIX_LEN],
    prev_key_matrix: [u8; KEY_MATRIX_LEN],
}
impl CalcHandle for TI89 {
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
impl TI89 {
    pub fn new() -> TI89 {
        TI89 {
            key_matrix: [0; KEY_MATRIX_LEN],
            prev_key_matrix: [0; KEY_MATRIX_LEN],
        }
    }
}
