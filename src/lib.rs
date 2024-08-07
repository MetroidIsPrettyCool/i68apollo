use std::time::Duration;

use cable::Cable;
use calc::{ti92p::TI92Plus, Calc, KeyMatrixDelta};
use keyboard::{CalcKey, VirtualKeyboard};

pub mod cable;
pub mod calc;
pub mod keyboard;

pub fn apollo_version() -> (u8, u8, u8) {
    let apollo_ver_major = u8::from_str_radix(env!("CARGO_PKG_VERSION_MAJOR"), 10)
        .expect("unable to parse crate major version as u8");
    let apollo_ver_minor = u8::from_str_radix(env!("CARGO_PKG_VERSION_MINOR"), 10)
        .expect("unable to parse crate minor version as u8");
    let apollo_ver_patch = u8::from_str_radix(env!("CARGO_PKG_VERSION_PATCH"), 10)
        .expect("unable to parse crate patch version as u8");

    (apollo_ver_major, apollo_ver_minor, apollo_ver_patch)
}

pub fn handshake(cable: &mut Cable) {
    let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();

    let ready_byte = cable.read_bytes(1, Duration::from_secs(0), false);
    if ready_byte[0] != 0x50 {
        println!("Received non-ready signal, aborting");
        return;
    }
    println!("soyuz ready");

    // ---------------version check---------------

    println!("Performing version check...");

    let soyuz_ver = cable.read_bytes(3, Duration::from_secs(0), false);

    let apollo_ver: [u8; 3] = [apollo_ver_major, apollo_ver_minor, apollo_ver_patch];
    cable.write_bytes(&apollo_ver, Duration::from_secs(0));

    let soyuz_ver_major = soyuz_ver[0];
    let soyuz_ver_minor = soyuz_ver[1];
    let soyuz_ver_patch = soyuz_ver[2];

    println!("soyuz: {soyuz_ver_major}.{soyuz_ver_minor}.{soyuz_ver_patch}");
    println!("apollo: {apollo_ver_major}.{apollo_ver_minor}.{apollo_ver_patch}");

    if apollo_ver_major != soyuz_ver_major || apollo_ver_minor != soyuz_ver_minor {
        println!("Version mismatch, aborting");
        cable.release().expect("Unable to release interface 0"); // in from calc
        return;
    }
}

pub fn run(cable: &mut Cable, calc: &mut TI92Plus, virtual_kbd: &mut VirtualKeyboard) {
    'outer: loop {
        for keystate in calc.get_keys(cable) {
            let (key, pressed) = keystate;

            if key == CalcKey::ON && pressed {
                break 'outer;
            }

            if pressed {
                virtual_kbd.press_key(&key);
            } else {
                virtual_kbd.release_key(&key);
            }
        }

        virtual_kbd
            .handle
            .synchronize()
            .expect("Unable to synchronize");
    }
}
