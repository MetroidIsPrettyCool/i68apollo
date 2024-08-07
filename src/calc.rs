use crate::{cable::Cable, keyboard::CalcKey};

pub mod ti92p;

pub trait Calc {
    fn get_keys(&mut self, cable: &mut Cable) -> Vec<(CalcKey, bool)>;
}
