use uinput::event::Keyboard;

use crate::cable::Cable;

pub mod ti92p;

// pub enum Calc {
//     TI92P,
// }
// impl Calc {
//     pub fn get_keymap(&self) -> &[((u8, u8), Keyboard)] {
//         match self {
//             Calc::TI92P => &ti92p::KEY_TO_KEY_MAP,
//         }
//     }
//     pub fn get_key_matrix_len(&self) -> usize {
//         match self {
//             Calc::TI92P => ti92p::KEY_MATRIX_LEN,
//         }
//     }
// }

pub struct KeyMatrixDelta<'a> {
    pub curr: &'a [u8],
    pub prev: &'a [u8],
}

pub trait Calc<'a> {
    fn get_keymap(&self) -> &[((u8, u8), Keyboard)];
    fn get_key_matrix_len(&self) -> usize;
    fn read_key_matrix(&'a mut self, cable: &mut Cable) -> KeyMatrixDelta<'a>;
}
