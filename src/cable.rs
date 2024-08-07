use std::time::Duration;

use rusb::{DeviceHandle, GlobalContext};

// constants sourced from lsusb
const WRITE_ENDPOINT: u8 = 0x02;
const READ_ENDPOINT: u8 = 0x81;

const TI_VENDOR_ID: u16 = 0x0451;
const SILVERLINK_PRODUCT_ID: u16 = 0xe001;

pub struct Cable {
    handle: DeviceHandle<GlobalContext>,
    // the SilverLink has its own internal 32-byte buffer, but since our data packets are 11 bytes they don't align and
    // we need a second buffer to store the raw data
    packet_buffer: Vec<u8>,

    bytes_read_overall: usize,

    malformed_reads: u64,
    overreads: u64,
}
impl Cable {
    pub fn new() -> Result<Cable, String> {
        let cable_handle = match get_link_cable() {
            Some(handle) => handle,
            None => {
                return Err("unable to find cable. (is it plugged in?)".to_string());
            }
        };

        let reset_result = cable_handle.reset();
        if let Err(e) = reset_result {
            return Err(format!("unable to reset cable: {e}"));
        }

        let set_conf_result = cable_handle.set_active_configuration(1);
        if let Err(e) = set_conf_result {
            return Err(format!("unable to set active configuration: {e}"));
        }

        let claim_interface_result = cable_handle.claim_interface(0);
        if let Err(e) = claim_interface_result {
            return Err(format!("unable to claim interface: {e}"));
        }

        Ok(Cable {
            handle: cable_handle,
            packet_buffer: Vec::new(),
            bytes_read_overall: 0,
            malformed_reads: 0,
            overreads: 0,
        })
    }

    pub fn read_bytes(&mut self, bytes_expected: usize, timeout: Duration, attempt_repair: bool) -> Vec<u8> {
        let mut bytes_read = 0;

        while self.packet_buffer.len() < bytes_expected {
            let mut buf: [u8; 512] = [0; 512]; // the cable /advertises/ that the max packet size is 32 bytes. This is apparently a lie.
            let read_size = self.handle.read_bulk(READ_ENDPOINT, &mut buf, timeout).unwrap();

            self.bytes_read_overall += read_size;
            bytes_read += read_size;

            self.packet_buffer.extend_from_slice(&buf[0..read_size]);
        }

        if attempt_repair && bytes_read > bytes_expected {
            self.overreads += 1;

            let discrepancy = bytes_read - bytes_expected;
            if bytes_read % bytes_expected == 0 {
                println!("More bytes read than expected ({discrepancy}). Likely multiple packets, ignoring.");
            } else {
                self.malformed_reads += 1;

                println!("More bytes read than expected ({discrepancy}). Possible desync, attempting repair.");
                self.packet_buffer.drain(0..discrepancy);
            }
        }

        return self.packet_buffer.drain(0..bytes_expected).collect::<Vec<u8>>();
    }

    pub fn write_bytes(&mut self, bytes: &[u8], timeout: Duration) {
        let _bytes_written = self.handle.write_bulk(WRITE_ENDPOINT, bytes, timeout).unwrap();
    }

    pub fn release(&mut self) -> rusb::Result<()> {
        self.handle.release_interface(0)
    }

    pub fn bytes_read_overall(&self) -> usize {
        self.bytes_read_overall
    }

    pub fn malformed_reads(&self) -> u64 {
        self.malformed_reads
    }

    pub fn overreads(&self) -> u64 {
        self.overreads
    }
}

fn get_link_cable() -> Option<DeviceHandle<GlobalContext>> {
    let devices = rusb::devices().expect("Unable to access USB device list");

    for device in devices.iter() {
        println!("Trying device {}:{}...", device.bus_number(), device.address());

        let descriptor = match device.device_descriptor() {
            Ok(descriptor) => descriptor,
            Err(e) => {
                println!("Unable to access device descriptor, skipping. Reason: {e}");
                continue;
            }
        };

        if descriptor.vendor_id() != TI_VENDOR_ID || descriptor.product_id() != SILVERLINK_PRODUCT_ID {
            println!("Device is not SilverLink cable, skipping.");
            continue;
        }

        let handle = match device.open() {
            Ok(handle) => handle,
            Err(e) => {
                println!("Unable to open SilverLink cable, skipping. Reason: {e}");
                continue;
            }
        };

        return Some(handle);
    }
    None
}
