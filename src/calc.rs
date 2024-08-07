use uinput::event::Keyboard;

use crate::cable::Cable;

pub mod ti92p;

fn calculate_checksum(data: Vec<u8>) -> u16 {
    let mut checksum: u16 = 0;
    for byte in data {
        checksum = checksum.wrapping_add(byte as u16);
    }
    checksum
}

pub struct KeyMatrixDelta<'a> {
    pub curr: &'a [u8],
    pub prev: &'a [u8],
}

pub trait Calc<'a> {
    fn get_keymap(&self) -> &[((usize, u8), Keyboard)];
    fn get_key_matrix_len(&self) -> usize;
    fn read_key_matrix(&'a mut self, cable: &mut Cable) -> KeyMatrixDelta<'a>;
}
