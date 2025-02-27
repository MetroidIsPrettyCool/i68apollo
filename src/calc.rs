use std::time::Duration;

use debug_print::{debug_eprint, debug_eprintln};
use ti83p::TI83Plus;
use ti89::TI89;
use ti92p::TI92Plus;

use crate::{cable::Cable, keyboard::CalcKey};

pub mod ti92p;
pub mod ti89;
pub mod ti83p;

const TI83P_TI84P_OS_MACHINE_ID: u8 = 0x73;
const TI92P_V200_OS_MACHINE_ID: u8 = 0x88;
const TI89_TI89TI_OS_MACHINE_ID: u8 = 0x98;

// TMP LOCATION
fn write_key(command: [u8; 4], cable: &mut Cable, response_length: usize) {
    cable.write_bytes(&command, Duration::from_secs(1)).unwrap();
    let _ = cable.read_bytes(response_length, Duration::from_secs(1)).unwrap();
}

pub trait CalcHandle {
    fn get_keys(&mut self, cable: &mut Cable) -> Vec<(CalcKey, bool)>;
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum HandshakeError {
    VersionMismatch(u8, u8, u8),
    UnknownMachineId(u8),
    OtherError,
}

pub struct I68MetaInfo {
    pub soyuz_ver: (u8, u8, u8),
    pub soyuz_machine_id: u8,
    pub calc_handle: Box<dyn CalcHandle>,
}
impl I68MetaInfo {
    pub fn handshake(cable: &mut Cable) -> Result<I68MetaInfo, HandshakeError> {
        let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();

        // probe calculator for OS machine ID and ready status

        debug_eprintln!("calc: probing for architecture...");

        cable.write_bytes(&[0x00, 0x68, 0x00, 0x00], Duration::from_secs(1)).unwrap();
        let probe_result = cable.read_bytes(4, Duration::from_secs(1)).unwrap();

        debug_eprintln!("calc: os machine id: {:02x}h", probe_result[0]);

        // TODO: check if ready, else loop

        // remotely command the calculator to start soyuz

        debug_eprintln!("calc: remotely commanding soyuz to start...");

        match probe_result[0] {
            TI83P_TI84P_OS_MACHINE_ID => {
                write_key([0x23, 0x87, 0x40, 0x00], cable, 8); // Quit
                write_key([0x23, 0x87, 0x40, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x40, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x40, 0x00], cable, 8);

                write_key([0x23, 0x87, 0x09, 0x00], cable, 8); // Clear
                write_key([0x23, 0x87, 0x09, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x09, 0x00], cable, 8);

                write_key([0x23, 0x87, 0x9C, 0xFC], cable, 8); // Asm(

                write_key([0x23, 0x87, 0x3E, 0x00], cable, 8); // Catalog, scroll down 14 times
                write_key([0x23, 0x87, 0xA9, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x04, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x05, 0x00], cable, 8);

                write_key([0x23, 0x87, 0xA2, 0x00], cable, 8); // i68soyuz
                write_key([0x23, 0x87, 0x94, 0x00], cable, 8);
                write_key([0x23, 0x87, 0x96, 0x00], cable, 8);
                write_key([0x23, 0x87, 0xAC, 0x00], cable, 8);
                write_key([0x23, 0x87, 0xA8, 0x00], cable, 8);
                write_key([0x23, 0x87, 0xB2, 0x00], cable, 8);
                write_key([0x23, 0x87, 0xAE, 0x00], cable, 8);
                write_key([0x23, 0x87, 0xB3, 0x00], cable, 8);

                write_key([0x23, 0x87, 0x05, 0x00], cable, 4); // enter, acknowledge of key press gets eaten by soyuz
            }
            TI92P_V200_OS_MACHINE_ID | TI89_TI89TI_OS_MACHINE_ID => {
                write_key([0x08, 0x87, 0x01, 0x0B], cable, 4); // break

                write_key([0x08, 0x87, 0x08, 0x01], cable, 4); // ESC
                write_key([0x08, 0x87, 0x08, 0x01], cable, 4);
                write_key([0x08, 0x87, 0x08, 0x01], cable, 4);
                write_key([0x08, 0x87, 0x08, 0x01], cable, 4);

                write_key([0x08, 0x87, 0x08, 0x11], cable, 4); // Quit
                write_key([0x08, 0x87, 0x08, 0x11], cable, 4);
                write_key([0x08, 0x87, 0x08, 0x11], cable, 4);
                write_key([0x08, 0x87, 0x08, 0x11], cable, 4);

                write_key([0x08, 0x87, 0x51, 0x20], cable, 4); // HOME

                write_key([0x08, 0x87, 0x07, 0x01], cable, 4); // CLEAR
                write_key([0x08, 0x87, 0x07, 0x01], cable, 4);
                write_key([0x08, 0x87, 0x07, 0x01], cable, 4);

                write_key([0x08, 0x87, 0x69, 0x00], cable, 4); // i68soyuz
                write_key([0x08, 0x87, 0x36, 0x00], cable, 4);
                write_key([0x08, 0x87, 0x38, 0x00], cable, 4);
                write_key([0x08, 0x87, 0x73, 0x00], cable, 4);
                write_key([0x08, 0x87, 0x6F, 0x00], cable, 4);
                write_key([0x08, 0x87, 0x79, 0x00], cable, 4);
                write_key([0x08, 0x87, 0x75, 0x00], cable, 4);
                write_key([0x08, 0x87, 0x7A, 0x00], cable, 4);

                write_key([0x08, 0x87, 0x28, 0x00], cable, 4); // ()
                write_key([0x08, 0x87, 0x29, 0x00], cable, 4);

                write_key([0x08, 0x87, 0x0D, 0x00], cable, 4); // enter
            }
            _ => {
                return Err(HandshakeError::OtherError);
            }
        }

        debug_eprintln!("calc: attempting metadata exchange with local component...");

        // version check

        let soyuz_ver = cable.read_bytes(3, Duration::from_secs(5)).unwrap();

        debug_eprintln!("calc: soyuz ver: {:?}", soyuz_ver);

        let apollo_ver: [u8; 3] = [apollo_ver_major, apollo_ver_minor, apollo_ver_patch];
        cable.write_bytes(&apollo_ver, Duration::from_secs(0)).unwrap();

        let soyuz_ver_major = soyuz_ver[0];
        let soyuz_ver_minor = soyuz_ver[1];
        let soyuz_ver_patch = soyuz_ver[2];

        if apollo_ver_major != soyuz_ver_major || apollo_ver_minor != soyuz_ver_minor {
            return Err(HandshakeError::VersionMismatch(
                soyuz_ver_major,
                soyuz_ver_minor,
                soyuz_ver_patch,
            ));
        }

        // machine id

        let soyuz_machine_id = cable.read_bytes(1, Duration::from_secs(0)).unwrap()[0];
        debug_eprintln!("calc: soyuz machine id: {soyuz_machine_id}");

        let calc_handle: Box<dyn CalcHandle> = match soyuz_machine_id {
            192 => Box::new(TI92Plus::new()),
            089 => Box::new(TI89::new()),
            183 => Box::new(TI83Plus::new()),
            _ => {
                return Err(HandshakeError::UnknownMachineId(soyuz_machine_id));
            }
        };

        Ok(I68MetaInfo {
            soyuz_ver: (soyuz_ver_major, soyuz_ver_minor, soyuz_ver_patch),
            soyuz_machine_id,
            calc_handle,
        })
    }
}

pub fn apollo_version() -> (u8, u8, u8) {
    let major = u8::from_str_radix(env!("CARGO_PKG_VERSION_MAJOR"), 10).unwrap();
    let minor = u8::from_str_radix(env!("CARGO_PKG_VERSION_MINOR"), 10).unwrap();
    let patch = u8::from_str_radix(env!("CARGO_PKG_VERSION_PATCH"), 10).unwrap();

    (major, minor, patch)
}
