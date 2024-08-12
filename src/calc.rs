use std::time::Duration;

use debug_print::debug_eprintln;
use ti83p::TI83Plus;
use ti89::TI89;
use ti92p::TI92Plus;

use crate::{cable::Cable, keyboard::CalcKey};

pub mod ti92p;
pub mod ti89;
pub mod ti83p;

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
    pub machine_id: u8,
    pub calc_handle: Box<dyn CalcHandle>,
}
impl I68MetaInfo {
    pub fn handshake(cable: &mut Cable) -> Result<I68MetaInfo, HandshakeError> {
        let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();

        // ready?

        let ready_byte = cable.read_bytes(1, Duration::from_secs(0));
        debug_eprintln!("ready_byte: {:?}", ready_byte);
        if ready_byte[0] != 0x50 {
            return Err(HandshakeError::OtherError);
        }

        // version check

        let soyuz_ver = cable.read_bytes(3, Duration::from_secs(0));

        debug_eprintln!("soyuz ver: {:?}", soyuz_ver);

        let apollo_ver: [u8; 3] = [apollo_ver_major, apollo_ver_minor, apollo_ver_patch];
        cable.write_bytes(&apollo_ver, Duration::from_secs(0));

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

        let machine_id = cable.read_bytes(1, Duration::from_secs(0))[0];
        debug_eprintln!("machine id: {machine_id}");

        let calc_handle: Box<dyn CalcHandle> = match machine_id {
            192 => Box::new(TI92Plus::new()),
            089 => Box::new(TI89::new()),
            183 => Box::new(TI83Plus::new()),
            _ => {
                return Err(HandshakeError::UnknownMachineId(machine_id));
            }
        };

        Ok(I68MetaInfo {
            soyuz_ver: (soyuz_ver_major, soyuz_ver_minor, soyuz_ver_patch),
            machine_id,
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
