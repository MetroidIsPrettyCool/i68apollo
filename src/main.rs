use std::time::Instant;

use i68apollo::{
    cable::{Cable, CableCreationError},
    calc::{apollo_version, HandshakeError, I68MetaInfo},
    keyboard::{VirtualKeyboard, VirtualKeyboardCreationError},
    run,
};

fn init_cable() -> Result<Cable, ()> {
    eprintln!("Initializing SilverLink cable...");
    return match Cable::new() {
        Ok(cable) => {
            eprintln!("SilverLink successfully initialized\n");
            Ok(cable)
        }

        Err(e) => {
            eprintln!("Initialization failed\n");
            match e {
                CableCreationError::GetDevicesListFailed(e) => {
                    eprintln!("Couldn't get USB devices list. Reason: {e}");
                }
                CableCreationError::NoCableFound => {
                    eprintln!("Couldn't find SilverLink cable. Is it plugged in?");
                }
                CableCreationError::ClaimInterfaceFailed(e) => {
                    eprintln!("Couldn't claim cable interface 0x00. Reason: {e}");
                    eprintln!("Is another program using this cable?");
                }
                CableCreationError::ConfigurationFailed(e) => {
                    eprintln!("Couldn't set cable active configuration. Reason: {e}");
                    eprintln!("Is another program using this cable?");
                }
                CableCreationError::ResetFailed(e) => {
                    eprintln!("Couldn't reset cable. Reason: {e}");
                    eprintln!("Is another program using this cable?");
                }
            }
            Err(())
        }
    };
}

fn init_vkbd() -> Result<VirtualKeyboard, ()> {
    eprintln!("Creating virtual keyboard...");
    return match VirtualKeyboard::new() {
        Ok(vkbd) => {
            eprintln!("Virtual keyboard created\n");
            Ok(vkbd)
        }

        Err(e) => {
            eprintln!("Creation failed\n");
            match e {
                VirtualKeyboardCreationError::UinputNotFound => {
                    eprintln!("uinput device file not found. Is it loaded?");
                    eprintln!("Try running \"sudo modprobe uinput\"");
                }
                VirtualKeyboardCreationError::DefaultFailed(e) => {
                    eprintln!("Default uinput device construction failed. Reason: {e}");
                }
                VirtualKeyboardCreationError::SetNameFailed(e) => {
                    eprintln!("Setting uinput device name failed. Reason: {e}");
                }
                VirtualKeyboardCreationError::EnableKeyFailed(key, e) => {
                    eprintln!("Enabling key {key:?} for uinput device failed. Reason: {e}");
                }
                VirtualKeyboardCreationError::CreationFailed(e) => {
                    eprintln!("Finalizing uinput device failed. Reason: {e}");
                }
            }
            Err(())
        }
    };
}

fn init_calc(cable: &mut Cable) -> Result<I68MetaInfo, ()> {
    println!("Press any key on calculator to continue");
    eprintln!("Waiting for handshake...");

    let i68_config = match I68MetaInfo::handshake(cable) {
        Ok(conf) => conf,

        Err(e) => {
            eprintln!("Handshake failed\n");
            match e {
                HandshakeError::VersionMismatch(
                    soyuz_ver_major,
                    soyuz_ver_minor,
                    soyuz_ver_patch,
                ) => {
                    eprintln!("Version mismatch");
                    eprintln!(
                        "soyuz ver: {}.{}.{}\n",
                        soyuz_ver_major, soyuz_ver_minor, soyuz_ver_patch
                    );
                }

                HandshakeError::UnknownMachineId(machine_id) => {
                    eprintln!("Unknown/unsupported Machine ID: {machine_id}");
                    eprintln!("Are you running the current version of i68apollo?");
                }

                HandshakeError::OtherError => {
                    eprintln!("Error during handshake");
                }
            }
            return Err(());
        }
    };
    eprintln!("Handshake success\n");
    eprintln!(
        "soyuz ver: {}.{}.{}\n",
        i68_config.soyuz_ver.0, i68_config.soyuz_ver.1, i68_config.soyuz_ver.2
    );

    return Ok(i68_config);
}

fn main() -> Result<(), ()> {
    // ---------------startup message---------------

    let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();
    println!("i68 local component \"apollo\"\n");
    println!(
        "Version: {}.{}.{}\n",
        apollo_ver_major, apollo_ver_minor, apollo_ver_patch
    );

    // ---------------init---------------

    let mut cable = init_cable()?;

    let mut virtual_kbd = init_vkbd()?;

    let calc = init_calc(&mut cable)?;

    // ---------------main loop---------------

    eprintln!("Begin async key matrix data transfer");

    println!("Press ON at any time to quit.\n");
    let loop_start = Instant::now();
    run(&mut cable, calc.calc_handle, &mut virtual_kbd);

    // ---------------print stats---------------

    let time_elapsed = Instant::now().duration_since(loop_start);
    println!(
        "{} overall bytes read in {:.2} seconds",
        cable.stat_bytes_read_overall,
        time_elapsed.as_secs_f64(),
    );
    println!(
        "overreads: {}, malformed reads: {}",
        cable.stat_overreads, cable.stat_malformed_reads
    );

    Ok(())
}
