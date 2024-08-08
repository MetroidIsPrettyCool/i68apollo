use std::time::Instant;

use i68apollo::{
    cable::{Cable, CableCreationError},
    calc::ti92p::TI92Plus,
    handshake::{apollo_version, I68Config, HandshakeError},
    keyboard::{VirtualKeyboard, VirtualKeyboardCreationError},
    run,
};

fn main() {
    // ---------------startup message---------------

    let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();
    println!("i68 local component \"apollo\"\n\nVersion: {apollo_ver_major}.{apollo_ver_minor}.{apollo_ver_patch}\n");

    // ---------------init cable---------------

    println!("Initializing SilverLink cable...");
    let mut cable = match Cable::new() {
        Ok(cable) => cable,

        Err(e) => {
            println!("Initialization failed\n");
            match e {
                CableCreationError::GetDevicesListFailed(e) => {
                    println!("Couldn't get USB devices list. Reason: {e}");
                }
                CableCreationError::NoCableFound => {
                    println!("Couldn't find SilverLink cable. Is it plugged in?");
                }
                CableCreationError::ClaimInterfaceFailed(e) => {
                    println!("Couldn't claim cable interface 0x00. Reason: {e}");
                    println!("Is another program using this cable?");
                }
                CableCreationError::ConfigurationFailed(e) => {
                    println!("Couldn't set cable active configuration. Reason: {e}");
                    println!("Is another program using this cable?");
                }
                CableCreationError::ResetFailed(e) => {
                    println!("Couldn't reset cable. Reason: {e}");
                    println!("Is another program using this cable?");
                }
            }
            return;
        }
    };
    println!("SilverLink successfully initialized\n");

    // ---------------init uinput device---------------

    println!("Creating virtual keyboard...");
    let mut virtual_kbd = match VirtualKeyboard::new() {
        Ok(vkbd) => vkbd,

        Err(e) => {
            println!("Creation failed\n");
            match e {
                VirtualKeyboardCreationError::UinputNotFound => {
                    println!("uinput device file not found. Is it loaded?");
                    println!("Try running \"sudo modprobe uinput\"");
                }
                VirtualKeyboardCreationError::DefaultFailed(e) => {
                    println!("Default uinput device construction failed. Reason: {e}");
                }
                VirtualKeyboardCreationError::SetNameFailed(e) => {
                    println!("Setting uinput device name failed. Reason: {e}");
                }
                VirtualKeyboardCreationError::EnableKeyFailed(key, e) => {
                    println!("Enabling key {key:?} for uinput device failed. Reason: {e}");
                }
                VirtualKeyboardCreationError::CreationFailed(e) => {
                    println!("Finalizing uinput device failed. Reason: {e}");
                }
            }
            return;
        }
    };
    println!("Virtual keyboard created\n");

    // ---------------init calc---------------

    let calc = TI92Plus::new();

    println!("Waiting for handshake...");
    let i68_config = match I68Config::handshake(&mut cable) {
        Ok(conf) => conf,

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
                }

                HandshakeError::OtherError => {
                    println!("Error during handshake");
                }
            }
            return;
        }
    };
    println!("Handshake success\n");
    println!(
        "soyuz ver: {}.{}.{}\n",
        i68_config.soyuz_ver.0, i68_config.soyuz_ver.1, i68_config.soyuz_ver.2
    );

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
