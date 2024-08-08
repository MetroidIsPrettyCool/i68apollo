use cable::Cable;
use calc::Calc;
use debug_print::debug_eprintln;
use keyboard::{CalcKey, VirtualKeyboard};

pub mod cable;
pub mod calc;
pub mod handshake;
pub mod keyboard;

pub fn run(cable: &mut Cable, mut calc: Box<dyn Calc>, virtual_kbd: &mut VirtualKeyboard) {
    'outer: loop {
        for keystate in calc.get_keys(cable) {
            let (key, pressed) = keystate;

            debug_eprintln!("{key:?}, pressed?: {pressed}");

            if key == CalcKey::ON && pressed {
                break 'outer;
            }

            if pressed {
                virtual_kbd.press_key(&key).expect("can't press key!");
            } else {
                virtual_kbd.release_key(&key).expect("can't release key!");
            }
        }

        virtual_kbd.sync().expect("can't sync!");
    }
}
