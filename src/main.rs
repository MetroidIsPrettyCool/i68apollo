#[allow(unused_imports)]
use rusb::{DeviceHandle, GlobalContext, UsbContext};
use std::time::Instant;
#[allow(unused_imports)]
use std::{
    io::{stdout, Write},
    mem::transmute,
    thread,
    time::Duration,
};
use uinput::{
    event::{
        keyboard::{Key, KeyPad},
        Keyboard,
    },
    Event,
};

const KEY_TO_KEY_MAP: [((u8, u8), Keyboard); 78] = [
    ((0, 7), Keyboard::Key(Key::Down)),           // Down
    ((0, 6), Keyboard::Key(Key::Right)),          // Right
    ((0, 5), Keyboard::Key(Key::Up)),             // Up
    ((0, 4), Keyboard::Key(Key::Left)),           // Left
    ((0, 3), Keyboard::Key(Key::LeftMeta)),       // Hand
    ((0, 2), Keyboard::Key(Key::LeftShift)),      // Shift
    ((0, 1), Keyboard::Key(Key::LeftControl)),    // Diamnd
    ((0, 0), Keyboard::Key(Key::LeftAlt)),        // 2nd
    ((1, 7), Keyboard::Key(Key::_3)),             // 3
    ((1, 6), Keyboard::Key(Key::_2)),             // 2
    ((1, 5), Keyboard::Key(Key::_1)),             // 1
    ((1, 4), Keyboard::Key(Key::F8)),             // F8
    ((1, 3), Keyboard::Key(Key::W)),              // W
    ((1, 2), Keyboard::Key(Key::S)),              // S
    ((1, 1), Keyboard::Key(Key::Z)),              // Z
    ((2, 7), Keyboard::Key(Key::_6)),             // 6
    ((2, 6), Keyboard::Key(Key::_5)),             // 5
    ((2, 5), Keyboard::Key(Key::_4)),             // 4
    ((2, 4), Keyboard::Key(Key::F3)),             // F3
    ((2, 3), Keyboard::Key(Key::E)),              // E
    ((2, 2), Keyboard::Key(Key::D)),              // D
    ((2, 1), Keyboard::Key(Key::X)),              // X
    ((3, 7), Keyboard::Key(Key::_9)),             // 9
    ((3, 6), Keyboard::Key(Key::_8)),             // 8
    ((3, 5), Keyboard::Key(Key::_7)),             // 7
    ((3, 4), Keyboard::Key(Key::F7)),             // F7
    ((3, 3), Keyboard::Key(Key::R)),              // R
    ((3, 2), Keyboard::Key(Key::F)),              // F
    ((3, 1), Keyboard::Key(Key::C)),              // C
    ((3, 0), Keyboard::Key(Key::F24)),            // STO
    ((4, 7), Keyboard::Key(Key::Comma)),          // ,
    ((4, 6), Keyboard::Key(Key::RightBrace)),     // )
    ((4, 5), Keyboard::Key(Key::LeftBrace)),      // (
    ((4, 4), Keyboard::Key(Key::F2)),             // F2
    ((4, 3), Keyboard::Key(Key::T)),              // T
    ((4, 2), Keyboard::Key(Key::G)),              // G
    ((4, 1), Keyboard::Key(Key::V)),              // V
    ((4, 0), Keyboard::Key(Key::Space)),          // Space
    ((5, 7), Keyboard::Key(Key::F23)),            // TAN
    ((5, 6), Keyboard::Key(Key::F22)),            // COS
    ((5, 5), Keyboard::Key(Key::F21)),            // SIN
    ((5, 4), Keyboard::Key(Key::F6)),             // F6
    ((5, 3), Keyboard::Key(Key::Y)),              // Y
    ((5, 2), Keyboard::Key(Key::H)),              // H
    ((5, 1), Keyboard::Key(Key::B)),              // B
    ((5, 0), Keyboard::Key(Key::Slash)),          // /
    ((6, 7), Keyboard::Key(Key::P)),              // P
    ((6, 6), Keyboard::Key(Key::LineFeed)),       // ENTER2
    ((6, 5), Keyboard::Key(Key::F20)),            // LN
    ((6, 4), Keyboard::Key(Key::F1)),             // F1
    ((6, 3), Keyboard::Key(Key::U)),              // U
    ((6, 2), Keyboard::Key(Key::J)),              // J
    ((6, 1), Keyboard::Key(Key::N)),              // N
    ((6, 0), Keyboard::Key(Key::F19)),            // ^
    ((7, 7), Keyboard::KeyPad(KeyPad::Asterisk)), // *
    ((7, 6), Keyboard::Key(Key::F18)),            // APPS
    ((7, 5), Keyboard::Key(Key::F17)),            // CLEAR
    ((7, 4), Keyboard::Key(Key::F5)),             // F5
    ((7, 3), Keyboard::Key(Key::I)),              // I
    ((7, 2), Keyboard::Key(Key::K)),              // K
    ((7, 1), Keyboard::Key(Key::M)),              // M
    ((7, 0), Keyboard::Key(Key::Equal)),          // =
    ((8, 6), Keyboard::Key(Key::Esc)),            // ESC
    ((8, 5), Keyboard::Key(Key::F16)),            // MODE
    ((8, 4), Keyboard::KeyPad(KeyPad::Plus)),     // +
    ((8, 3), Keyboard::Key(Key::O)),              // O
    ((8, 2), Keyboard::Key(Key::L)),              // L
    ((8, 1), Keyboard::Key(Key::F15)),            // θ
    ((8, 0), Keyboard::Key(Key::BackSpace)),      // BckSpc
    ((9, 7), Keyboard::KeyPad(KeyPad::Minus)),    // (-)
    ((9, 6), Keyboard::Key(Key::Dot)),            // .
    ((9, 5), Keyboard::Key(Key::_0)),             // 0
    ((9, 4), Keyboard::Key(Key::F4)),             // F4
    ((9, 3), Keyboard::Key(Key::Q)),              // Q
    ((9, 2), Keyboard::Key(Key::A)),              // A
    ((9, 1), Keyboard::KeyPad(KeyPad::Enter)),    // ENTER1
    ((9, 0), Keyboard::Key(Key::Minus)),          // -
    ((10, 0), Keyboard::Key(Key::SysRq)),         // ON
];

fn calculate_checksum(data: Vec<u8>) -> u16 {
    let mut checksum: u16 = 0;
    for byte in data {
        checksum = checksum.wrapping_add(byte as u16);
    }
    checksum
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

fn create_virtual_kbd() -> uinput::Result<uinput::Device> {
    let mut virtual_kbd = uinput::default()?.name("i68apollo")?;

    for key_to_key_pair in KEY_TO_KEY_MAP {
        let (_, key_event) = key_to_key_pair;
        virtual_kbd = virtual_kbd.event(Event::Keyboard(key_event))?;
    }

    virtual_kbd.create()
}

struct Cable {
    handle: DeviceHandle<GlobalContext>,
    // the SilverLink has its own internal 32-byte buffer, but since our data packets are 11 bytes they don't align and
    // we need a second buffer to store the raw data
    packet_buffer: Vec<u8>,
}
impl Cable {
    fn new() -> Result<Cable, String> {
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

    fn next_packet(&mut self) -> [u8; 11] {
        while self.packet_buffer.len() < 11 {
            let mut buf: [u8; 32] = [0; 32];
            let bytes_read = self.handle.read_bulk(0x81, &mut buf, Duration::from_secs(0)).unwrap();
            self.packet_buffer.extend_from_slice(&buf[0..bytes_read]);
        }
        return self.packet_buffer.drain(0..11).collect::<Vec<u8>>().try_into().unwrap();
    }

    fn release(&mut self) -> rusb::Result<()> {
        self.handle.release_interface(0)
    }
}

fn main() {
    // ---------------startup message---------------

    println!("i68 local component \"apollo\"\n\nExpecting build 28");

    // ---------------init cable---------------

    println!("Initializing SilverLink cable...");

    let mut cable = Cable::new().expect("Error initializing cable");

    println!("SilverLink successfully initialized");

    // ---------------init uinput device---------------

    println!("Creating virtual keyboard...");

    let mut virtual_kbd = create_virtual_kbd().expect("Unable to create virtual keyboard. Is uinput loaded? Reason");

    println!("Virtual keyboard created");

    // ---------------main loop---------------

    println!("Awaiting first packet...");

    let mut keymap: [u8; 11] = [0; 11];
    let mut prev_keymap: [u8; 11] = [0; 11];

    let loop_start = Instant::now();
    let mut packets = 0;

    loop {
        prev_keymap.copy_from_slice(&keymap);
        keymap.copy_from_slice(&cable.next_packet());

        if keymap[10] == 1 {
            break;
        }

        for key_to_key_pair in KEY_TO_KEY_MAP {
            let ((row, col), key_event) = key_to_key_pair;
            if keymap[row as usize] & (1 << col) != prev_keymap[row as usize] & (1 << col) {
                if keymap[row as usize] & (1 << col) == 0 {
                    virtual_kbd
                        .release(&key_event)
                        .expect("Unable to release key!");

                    println!("Release {key_event:?}");
                } else {
                    virtual_kbd.press(&key_event).expect("Unable to press key!");

                    println!("Press {key_event:?}");
                }
            }
        }
        virtual_kbd.synchronize().expect("Unable to synchronize");

        packets += 1;
    }

    let secs_since_loop_start = Instant::now().duration_since(loop_start).as_secs_f64();
    println!(
        "\n{} packets in {:.2} seconds\n{:.2} packets/sec, {:.2} bytes/sec, or {:.2} baud",
        packets,
        secs_since_loop_start,
        packets as f64 / secs_since_loop_start,
        (packets * 11) as f64 / secs_since_loop_start,
        (packets * 11 * 8) as f64 / secs_since_loop_start
    );

    cable.release()
        .expect("Unable to release interface 0"); // in from calc
}
