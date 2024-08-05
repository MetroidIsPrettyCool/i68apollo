use uinput::event::Keyboard;

pub mod ti92p;

pub enum Calc {
    TI92P,
}
impl Calc {
    pub fn get_keymap(&self) -> [((u8, u8), Keyboard); 78] {
        match self {
            Calc::TI92P => ti92p::KEY_TO_KEY_MAP,
        }
    }
}
