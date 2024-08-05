use std::time::Duration;

use rusb::{DeviceHandle, GlobalContext};

pub struct Cable {
    handle: DeviceHandle<GlobalContext>,
    // the SilverLink has its own internal 32-byte buffer, but since our data packets are 11 bytes they don't align and
    // we need a second buffer to store the raw data
    packet_buffer: Vec<u8>,
}
impl Cable {
    pub fn new() -> Result<Cable, String> {
        let cable_handle = match get_link_cable() {
            Some(handle) => handle,
            None => {
                return Err("unable to find cable. (is it plugged in?)".to_string());
            }
        };

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
        })
    }

    pub fn read_bytes(&mut self, bytes: usize, timeout: Duration) -> Vec<u8> {
        while self.packet_buffer.len() < bytes {
            let mut buf: [u8; 32] = [0; 32];
            let bytes_read = self.handle.read_bulk(0x81, &mut buf, timeout).unwrap();
            self.packet_buffer.extend_from_slice(&buf[0..bytes_read]);
        }
        return self.packet_buffer.drain(0..bytes).collect::<Vec<u8>>();
    }

    pub fn write_bytes(&mut self, bytes: &[u8], timeout: Duration) {
        let _bytes_written = self.handle.write_bulk(0x02, bytes, timeout).unwrap();
    }

    pub fn release(&mut self) -> rusb::Result<()> {
        self.handle.release_interface(0)
    }
}

fn get_link_cable() -> Option<DeviceHandle<GlobalContext>> {
    let devices = rusb::devices().expect("unable to access USB device list");

    for device in devices.iter() {
        println!("Trying device...");

        let handle = match device.open() {
            Ok(handle) => handle,
            Err(e) => {
                println!("Unable to open device, skipping. Reason: {e}");
                continue;
            }
        };
        let descriptor = match device.device_descriptor() {
            Ok(descriptor) => descriptor,
            Err(e) => {
                println!("Unable to access device descriptor, skipping. Reason: {e}");
                continue;
            }
        };

        let product_string = match handle.read_product_string_ascii(&descriptor) {
            Ok(product_string) => product_string,
            Err(e) => {
                println!("Unable to access device product string, skipping. Reason: {e}");
                continue;
            }
        };

        if product_string == "TI-GRAPH LINK USB" {
            println!("Found SilverLink");
            return Some(handle);
        } else {
            println!("Device was not SilverLink, skipping");
            continue;
        }
    }
    return None;
}
