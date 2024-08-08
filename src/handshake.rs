use std::time::Duration;

use crate::cable::Cable;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum HandshakeError {
    VersionMismatch(u8, u8, u8),
    OtherError,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct I68Config {
    pub soyuz_ver: (u8, u8, u8),
}
impl I68Config {
    pub fn handshake(cable: &mut Cable) -> Result<I68Config, HandshakeError> {
        let (apollo_ver_major, apollo_ver_minor, apollo_ver_patch) = apollo_version();

        // ready?

        let ready_byte = cable.read_bytes(1, Duration::from_secs(0), false);
        if ready_byte[0] != 0x50 {
            return Err(HandshakeError::OtherError);
        }

        // version check

        let soyuz_ver = cable.read_bytes(3, Duration::from_secs(0), false);

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

        Ok(I68Config {
            soyuz_ver: (soyuz_ver_major, soyuz_ver_minor, soyuz_ver_patch),
        })
    }
}

pub fn apollo_version() -> (u8, u8, u8) {
    let major = u8::from_str_radix(env!("CARGO_PKG_VERSION_MAJOR"), 10).unwrap();
    let minor = u8::from_str_radix(env!("CARGO_PKG_VERSION_MINOR"), 10).unwrap();
    let patch = u8::from_str_radix(env!("CARGO_PKG_VERSION_PATCH"), 10).unwrap();

    (major, minor, patch)
}
