#[allow(unused_imports)]
use rusb::{DeviceHandle, GlobalContext, UsbContext};
use std::time::Instant;
#[allow(unused_imports)]
use std::{
    io::{stdout, Write},
    mem::transmute,
    thread,
    time::Duration,
};
#[allow(unused_imports)]
use uinput::{
    event::keyboard::{Key, KeyPad},
    Event,
};

use i68apollo::{cable::Cable, calc::Calc, keyboard::VirtualKeyboard};

fn calculate_checksum(data: Vec<u8>) -> u16 {
    let mut checksum: u16 = 0;
    for byte in data {
        checksum = checksum.wrapping_add(byte as u16);
    }
    checksum
}

fn main() {
    let apollo_ver_major = u8::from_str_radix(env!("CARGO_PKG_VERSION_MAJOR"), 10)
        .expect("unable to parse crate major version as u8");
    let apollo_ver_minor = u8::from_str_radix(env!("CARGO_PKG_VERSION_MINOR"), 10)
        .expect("unable to parse crate minor version as u8");
    let apollo_ver_patch = u8::from_str_radix(env!("CARGO_PKG_VERSION_PATCH"), 10)
        .expect("unable to parse crate patch version as u8");

    // ---------------startup message---------------

    println!("i68 local component \"apollo\"\n\nVersion: {apollo_ver_major}.{apollo_ver_minor}.{apollo_ver_patch}");

    // ---------------init cable---------------

    println!("Initializing SilverLink cable...");

    let mut cable = Cable::new().expect("Error initializing cable");

    println!("SilverLink successfully initialized");

    // ---------------init uinput device---------------

    println!("Creating virtual keyboard...");

    let mut virtual_kbd = VirtualKeyboard::new()
        .expect("Unable to create virtual keyboard. Is uinput loaded? Reason");

    println!("Virtual keyboard created");

    // ---------------init calc---------------

    let calc = Calc::TI92P;

    // ---------------wait---------------

    println!("Waiting for soyuz...");
    let ready_byte = cable.read_bytes(1, Duration::from_secs(0));
    if ready_byte[0] != 0x50 {
        println!("Received non-ready signal, aborting");
        cable.release().expect("Unable to release interface 0"); // in from calc
        return;
    }
    println!("soyuz ready");

    // ---------------version check---------------

    println!("Performing version check...");

    let soyuz_ver = cable.read_bytes(3, Duration::from_secs(0));

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

    println!("Versions match");

    // ---------------main loop---------------

    println!("Awaiting first packet...");

    let key_matrix_len = calc.get_key_matrix_len();

    let mut matrix_state: [u8; 10] = [0; 10]; // just some arbitrary length that's "big enough"
    let mut prev_matrix_state: [u8; 10] = [0; 10];

    let loop_start = Instant::now();
    let mut packets = 0;

    loop {
        prev_matrix_state.copy_from_slice(&matrix_state);
        matrix_state.copy_from_slice(&cable.read_bytes(key_matrix_len, Duration::from_secs(0)));

        if matrix_state[1] & 1 == 1 {
            break;
        }

        for key_to_key_pair in calc.get_keymap() {
            let ((row, col), key_event) = key_to_key_pair;
            if matrix_state[row as usize] & (1 << col)
                != prev_matrix_state[row as usize] & (1 << col)
            {
                if matrix_state[row as usize] & (1 << col) == 0 {
                    virtual_kbd
                        .handle
                        .release(&key_event)
                        .expect("Unable to release key!");

                    println!("Release {key_event:?}");
                } else {
                    virtual_kbd
                        .handle
                        .press(&key_event)
                        .expect("Unable to press key!");

                    println!("Press {key_event:?}");
                }
            }
        }
        virtual_kbd
            .handle
            .synchronize()
            .expect("Unable to synchronize");

        packets += 1;
    }

    let secs_since_loop_start = Instant::now().duration_since(loop_start).as_secs_f64();
    println!(
        "\n{} packets in {:.2} seconds\n{:.2} packets/sec, {:.2} bytes/sec, or {:.2} baud",
        packets,
        secs_since_loop_start,
        packets as f64 / secs_since_loop_start,
        (packets * key_matrix_len) as f64 / secs_since_loop_start,
        (packets * key_matrix_len * 8) as f64 / secs_since_loop_start
    );

    cable.release().expect("Unable to release interface 0"); // in from calc
}
