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
    // ---------------startup message---------------

    println!("i68 local component \"apollo\"\n\nExpecting build 29");

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

    // ---------------main loop---------------

    println!("Awaiting first packet...");

    let mut matrix_state: [u8; 11] = [0; 11];
    let mut prev_matrix_state: [u8; 11] = [0; 11];

    let loop_start = Instant::now();
    let mut packets = 0;

    loop {
        prev_matrix_state.copy_from_slice(&matrix_state);
        matrix_state.copy_from_slice(&cable.next_packet());

        if matrix_state[10] == 1 {
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
        (packets * 11) as f64 / secs_since_loop_start,
        (packets * 11 * 8) as f64 / secs_since_loop_start
    );

    cable.release().expect("Unable to release interface 0"); // in from calc
}
