use std::time::Instant;

use i68apollo::{
    apollo_version, cable::Cable, calc::ti92p::TI92Plus, handshake, keyboard::VirtualKeyboard, run,
};

fn main() {
    // ---------------startup message---------------

    let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();
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

    let mut calc = TI92Plus::new();

    // ---------------wait---------------

    println!("Waiting for soyuz...");
    handshake(&mut cable);
    println!("Versions match");

    // ---------------main loop---------------

    println!("Awaiting first packet...");

    let loop_start = Instant::now();

    run(&mut cable, &mut calc, &mut virtual_kbd);

    let time_elapsed = Instant::now().duration_since(loop_start);
    println!(
        "{} bytes read overall in {:.2} seconds",
        cable.bytes_read_overall,
        time_elapsed.as_secs_f64(),
    );
    println!(
        "overreads: {}, malformed reads: {}",
        cable.overreads, cable.malformed_reads
    );
}
