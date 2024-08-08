use std::time::Instant;

use i68apollo::{
    apollo_version, cable::Cable, calc::ti92p::TI92Plus, handshake, keyboard::VirtualKeyboard, run,
    HandshakeError,
};

fn main() {
    // ---------------startup message---------------

    let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();
    println!("i68 local component \"apollo\"\n\nVersion: {apollo_ver_major}.{apollo_ver_minor}.{apollo_ver_patch}\n");

    // ---------------init cable---------------

    println!("Initializing SilverLink cable...");
    let mut cable = Cable::new().expect("Error initializing cable");
    println!("SilverLink successfully initialized\n");

    // ---------------init uinput device---------------

    println!("Creating virtual keyboard...");
    let mut virtual_kbd = VirtualKeyboard::new()
        .expect("Unable to create virtual keyboard. Is uinput loaded? Reason");
    println!("Virtual keyboard created\n");

    // ---------------init calc---------------

    let calc = TI92Plus::new();

    println!("Waiting for handshake...");
    match handshake(&mut cable) {
        Ok(conf) => {
            println!("Handshake success\n");
            println!(
                "soyuz: {}.{}.{}\n",
                conf.soyuz_ver.0, conf.soyuz_ver.1, conf.soyuz_ver.2
            );
        }

        Err(e) => {
            println!("Handshake failed\n");
            match e {
                HandshakeError::VersionMismatch(
                    soyuz_ver_major,
                    soyuz_ver_minor,
                    soyuz_ver_patch,
                ) => {
                    println!("Version mismatch");
                    println!(
                        "soyuz ver: {}.{}.{}\n",
                        soyuz_ver_major, soyuz_ver_minor, soyuz_ver_patch
                    );
                    println!("Aborting");
                }

                HandshakeError::OtherError => {
                    println!("Error during handshake, aborting");
                }
            }
            return;
        }
    }

    // ---------------main loop---------------

    println!("Press ON at any time to quit.\n");
    let loop_start = Instant::now();
    run(&mut cable, Box::new(calc), &mut virtual_kbd);

    // ---------------print stats---------------

    let time_elapsed = Instant::now().duration_since(loop_start);
    println!(
        "{} bytes read overall in {:.2} seconds",
        cable.stat_bytes_read_overall,
        time_elapsed.as_secs_f64(),
    );
    println!(
        "overreads: {}, malformed reads: {}",
        cable.stat_overreads, cable.stat_malformed_reads
    );
}
