use uinput::event::Keyboard;

use crate::{cable::Cable, keyboard::CalcKey};

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
    fn get_key_matrix_len(&self) -> usize;
    fn read_key_matrix(&'a mut self, cable: &mut Cable) -> KeyMatrixDelta<'a>;

    fn get_keys(&mut self, cable: &mut Cable) -> Vec<(CalcKey, bool)>;
}
