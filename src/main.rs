use std::time::Instant;

use i68apollo::{
    cable::{Cable, CableCreationError},
    calc::{ti92p::TI92Plus, Calc},
    handshake::{apollo_version, HandshakeError, I68Config},
    keyboard::{VirtualKeyboard, VirtualKeyboardCreationError},
    run,
};

fn init_cable() -> Result<Cable, ()> {
    println!("Initializing SilverLink cable...");
    return match Cable::new() {
        Ok(cable) => {
            println!("SilverLink successfully initialized\n");
            Ok(cable)
        },

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
            Err(())
        }
    };
}

fn init_vkbd() -> Result<VirtualKeyboard, ()> {
    println!("Creating virtual keyboard...");
    return match VirtualKeyboard::new() {
        Ok(vkbd) => {
            println!("Virtual keyboard created\n");
            Ok(vkbd)
        },

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
            Err(())
        }
    };
}

fn init_calc(cable: &mut Cable) -> Result<(Box<dyn Calc>, I68Config), ()> {
    let calc = TI92Plus::new();

    println!("Waiting for handshake...");
    let i68_config = match I68Config::handshake(cable) {
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
            return Err(());
        }
    };
    println!("Handshake success\n");
    println!(
        "soyuz ver: {}.{}.{}\n",
        i68_config.soyuz_ver.0, i68_config.soyuz_ver.1, i68_config.soyuz_ver.2
    );

    return Ok((Box::new(calc), i68_config));
}

fn main() {
    // ---------------startup message---------------

    let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();
    println!("i68 local component \"apollo\"\n\nVersion: {apollo_ver_major}.{apollo_ver_minor}.{apollo_ver_patch}\n");

    // ---------------init---------------

    let mut cable = match init_cable() {
        Ok(cable) => cable,
        Err(_) => { return; }
    };

    let mut virtual_kbd = match init_vkbd() {
        Ok(vkbd) => vkbd,
        Err(_) => { return; }
    };

    let calc = match init_calc(&mut cable) {
        Ok((calc, _)) => calc,
        Err(_) => { return; }
    };

    // ---------------main loop---------------

    println!("Press ON at any time to quit.\n");
    let loop_start = Instant::now();
    run(&mut cable, calc, &mut virtual_kbd);

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
