use std::time::Duration;

use debug_print::debug_eprintln;
use rusb::{DeviceHandle, DeviceList, GlobalContext};

// constants sourced from lsusb
const WRITE_ENDPOINT: u8 = 0x02;
const READ_ENDPOINT: u8 = 0x81;

const TI_VENDOR_ID: u16 = 0x0451;
const SILVERLINK_PRODUCT_ID: u16 = 0xe001;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum CableCreationError {
    GetDevicesListFailed(rusb::Error),
    NoCableFound,
    ResetFailed(rusb::Error),
    ConfigurationFailed(rusb::Error),
    ClaimInterfaceFailed(rusb::Error),
}

pub struct Cable {
    handle: DeviceHandle<GlobalContext>,
    // the SilverLink has its own internal buffer, but our reads don't always align neatly with individual packets so we
    // need a buffer here too
    byte_buffer: Vec<u8>,

    pub stat_bytes_read_overall: usize,

    pub stat_malformed_reads: u64,
    pub stat_overreads: u64,
}
impl Cable {
    pub fn new() -> Result<Cable, CableCreationError> {
        let devices = match rusb::devices() {
            Ok(devices) => devices,
            Err(e) => {
                return Err(CableCreationError::GetDevicesListFailed(e));
            }
        };

        let cable_handle = match get_link_cable(devices) {
            Some(handle) => handle,
            None => {
                return Err(CableCreationError::NoCableFound);
            }
        };

        let reset_result = cable_handle.reset();
        if let Err(e) = reset_result {
            return Err(CableCreationError::ResetFailed(e));
        }

        let set_conf_result = cable_handle.set_active_configuration(1);
        if let Err(e) = set_conf_result {
            return Err(CableCreationError::ConfigurationFailed(e));
        }

        let claim_interface_result = cable_handle.claim_interface(0);
        if let Err(e) = claim_interface_result {
            return Err(CableCreationError::ClaimInterfaceFailed(e));
        }

        Ok(Cable {
            handle: cable_handle,
            byte_buffer: Vec::new(),
            stat_bytes_read_overall: 0,
            stat_malformed_reads: 0,
            stat_overreads: 0,
        })
    }

    pub fn read_bytes(&mut self, bytes_expected: usize, timeout: Duration) -> Vec<u8> {
        while self.byte_buffer.len() < bytes_expected {
            let mut buf: [u8; 512] = [0; 512]; // the cable /advertises/ that the max packet size is 32 bytes. This is apparently a lie.
            let read_size = self
                .handle
                .read_bulk(READ_ENDPOINT, &mut buf, timeout)
                .unwrap();

            self.stat_bytes_read_overall += read_size;

            self.byte_buffer.extend_from_slice(&buf[0..read_size]);
        }

        return self
            .byte_buffer
            .drain(0..bytes_expected)
            .collect::<Vec<u8>>();
    }

    pub fn write_bytes(&mut self, bytes: &[u8], timeout: Duration) {
        let _bytes_written = self
            .handle
            .write_bulk(WRITE_ENDPOINT, bytes, timeout)
            .unwrap();
    }

    pub fn release(&mut self) -> rusb::Result<()> {
        self.handle.release_interface(0)
    }
}

fn get_link_cable(devices: DeviceList<GlobalContext>) -> Option<DeviceHandle<GlobalContext>> {
    for device in devices.iter() {
        debug_eprintln!(
            "slvnk: Trying device {}:{}...",
            device.bus_number(),
            device.address()
        );

        let descriptor = match device.device_descriptor() {
            Ok(descriptor) => descriptor,
            #[allow(unused_variables)]
            Err(e) => {
                debug_eprintln!("slvnk: unable to access device descriptor, skipping. Reason: {e}");
                continue;
            }
        };

        if descriptor.vendor_id() != TI_VENDOR_ID
            || descriptor.product_id() != SILVERLINK_PRODUCT_ID
        {
            debug_eprintln!("slvnk: device is not SilverLink cable, skipping.");
            continue;
        }

        let handle = match device.open() {
            Ok(handle) => handle,
            #[allow(unused_variables)]
            Err(e) => {
                debug_eprintln!("slvnk: unable to open SilverLink cable, skipping. Reason: {e}");
                continue;
            }
        };

        return Some(handle);
    }
    None
}

// fn calculate_checksum(data: Vec<u8>) -> u16 {
//     let mut checksum: u16 = 0;
//     for byte in data {
//         checksum = checksum.wrapping_add(byte as u16);
//     }
//     checksum
// }
