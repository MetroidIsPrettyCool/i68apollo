use std::time::Duration;

use cable::Cable;
use calc::{ti92p::TI92Plus, Calc, KeyMatrixDelta};
use keyboard::VirtualKeyboard;

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
    let keymap = calc.get_keymap().to_owned();

    loop {
        let KeyMatrixDelta {
            curr: matrix_state,
            prev: prev_matrix_state,
        } = calc.read_key_matrix(cable);

        if matrix_state[1] & 1 == 1 {
            break;
        }

        for key_to_key_pair in &keymap {
            let ((row, col), key_event) = key_to_key_pair;
            if matrix_state[*row] & (1 << *col) != prev_matrix_state[*row] & (1 << *col) {
                if matrix_state[*row] & (1 << *col) == 0 {
                    virtual_kbd
                        .handle
                        .release(key_event)
                        .expect("Unable to release key!");

                    println!("Release {key_event:?}");
                } else {
                    virtual_kbd
                        .handle
                        .press(key_event)
                        .expect("Unable to press key!");

                    println!("Press {key_event:?}");
                }
            }
        }

        virtual_kbd
            .handle
            .synchronize()
            .expect("Unable to synchronize");
    }
}
