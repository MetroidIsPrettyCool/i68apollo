use uinput::{Device, Event, Result};

pub struct VirtualKeyboard {
    pub handle: Device,
}
impl VirtualKeyboard {
    pub fn new() -> Result<VirtualKeyboard> {
        let mut virtual_kbd = uinput::default()?.name("i68apollo")?;

        for key_to_key_pair in crate::calc::ti92p::KEY_TO_KEY_MAP {
            let (_, key_event) = key_to_key_pair;
            virtual_kbd = virtual_kbd.event(Event::Keyboard(key_event))?;
        }

        let handle = virtual_kbd.create()?;

        Ok(VirtualKeyboard { handle })
    }
}
