use std::time::Duration;

use crate::{cable::Cable, keyboard::CalcKey};

use super::CalcHandle;

pub const KEY_MATRIX_LEN: usize = 7;

pub const KEY_TO_KEY_MAP: [((usize, u8), CalcKey); 50] = [
    ((0, 3), CalcKey::CursorUp),
    ((0, 2), CalcKey::CursorRight),
    ((0, 1), CalcKey::CursorLeft),
    ((0, 0), CalcKey::CursorDown),
    ((1, 6), CalcKey::CLEAR),
    ((1, 5), CalcKey::Exponentiation),
    ((1, 4), CalcKey::Division),
    ((1, 3), CalcKey::Multiplication),
    ((1, 2), CalcKey::Subtraction),
    ((1, 1), CalcKey::Addition),
    ((1, 0), CalcKey::ENTER1),
    ((2, 6), CalcKey::VARS),
    ((2, 5), CalcKey::TAN),
    ((2, 4), CalcKey::RightParenthesis),
    ((2, 3), CalcKey::_9),
    ((2, 2), CalcKey::_6),
    ((2, 1), CalcKey::_3),
    ((2, 0), CalcKey::Negative),
    ((3, 7), CalcKey::STAT),
    ((3, 6), CalcKey::PRGM),
    ((3, 5), CalcKey::COS),
    ((3, 4), CalcKey::LeftParenthesis),
    ((3, 3), CalcKey::_8),
    ((3, 2), CalcKey::_5),
    ((3, 1), CalcKey::_2),
    ((3, 0), CalcKey::Period),
    ((4, 7), CalcKey::XCommaTCommaThetaCommaN),
    ((4, 6), CalcKey::APPS),
    ((4, 5), CalcKey::SIN),
    ((4, 4), CalcKey::Comma),
    ((4, 3), CalcKey::_7),
    ((4, 2), CalcKey::_4),
    ((4, 1), CalcKey::_1),
    ((4, 0), CalcKey::_0),
    ((5, 7), CalcKey::Alpha),
    ((5, 6), CalcKey::MATH),
    ((5, 5), CalcKey::Inverse),
    ((5, 4), CalcKey::Square),
    ((5, 3), CalcKey::LOG),
    ((5, 2), CalcKey::LN),
    ((5, 1), CalcKey::STO),
    ((5, 0), CalcKey::ON),
    ((6, 7), CalcKey::DEL),
    ((6, 6), CalcKey::MODE),
    ((6, 5), CalcKey::_2nd),
    ((6, 4), CalcKey::F1),
    ((6, 3), CalcKey::F2),
    ((6, 2), CalcKey::F3),
    ((6, 1), CalcKey::F4),
    ((6, 0), CalcKey::F5),
];

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct TI83Plus {
    key_matrix: [u8; KEY_MATRIX_LEN],
    prev_key_matrix: [u8; KEY_MATRIX_LEN],
}
impl CalcHandle for TI83Plus {
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
impl TI83Plus {
    pub fn new() -> TI83Plus {
        TI83Plus {
            key_matrix: [0; KEY_MATRIX_LEN],
            prev_key_matrix: [0; KEY_MATRIX_LEN],
        }
    }
}
