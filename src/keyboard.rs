use super::{ble::BLEKeyboard, constants::MAX_KEYPRESSES, keycode::Keycode};

#[derive(Debug, Default)]
#[repr(C)]
pub struct Report {
    modifiers: u8,
    unused: u8,
    keys: [u8; 6],
}

impl Report {
    fn add_keycode(&mut self, k: &Keycode) {
        let modifier = Keycode::modifier_bit(k);
        if modifier != 0 {
            self.modifiers |= modifier;
        } else {
            // Don't press twice.
            for i in 0..MAX_KEYPRESSES {
                let key = &mut self.keys[i];
                // If this slot is free
                if *key == 0 {
                    *key = *k as u8;
                    return;
                }

                if *key == *k as u8 {
                    // Already pressed.
                    return;
                }
            }

            // All slots are filled. Shuffle down and reuse last slot.
            for i in 0..MAX_KEYPRESSES - 1 {
                self.keys[i] = self.keys[i + 1]
            }
            *self.keys.last_mut().unwrap() = *k as u8;
        }
    }

    fn remove_keycode(&mut self, k: &Keycode) {
        let modifier = Keycode::modifier_bit(k);
        if modifier != 0 {
            self.modifiers &= !modifier;
        } else {
            let mut j = 0;
            for i in 0..MAX_KEYPRESSES {
                let key = self.keys[i];
                if key == 0 {
                    // At this point all the slots with pressed keys have been visited.
                    break;
                }
                if key == *k as u8 {
                    continue;
                    // We will have to remove this key from the reporting.
                }
                if i != j {
                    self.keys[j] = self.keys[i];
                }
                j += 1;
            }

            // Remove the remaining keys reported.
            while j < MAX_KEYPRESSES && self.keys[j] != 0 {
                self.keys[j] = 0;
                j += 1;
            }
        }
    }
}

pub struct Keyboard<'a> {
    report: Report,
    leds: u8,
    device: BLEKeyboard<'a>,
}

impl Keyboard<'_> {
    pub fn new(name: &str, manufacturer: &str) -> Self {
        let device = BLEKeyboard::new(name, manufacturer);
        let report = Report::default();

        let keyboard = Keyboard {
            report,
            leds: 0,
            device,
        };

        keyboard
    }

    pub fn press(&mut self, keycodes: &[Keycode]) {
        for keycode in keycodes {
            self.report.add_keycode(keycode);
        }
        self.device.send_report(&self.report);
    }

    pub fn release(&mut self, keycodes: &[Keycode]) {
        for keycode in keycodes {
            self.report.remove_keycode(keycode);
        }
        self.device.send_report(&self.report);
    }

    pub fn release_all(&mut self) {
        self.report = Report::default();
        self.device.send_report(&self.report)
    }

    pub fn send(&mut self, keycodes: &[Keycode]) {
        self.press(keycodes);
        self.release_all();
    }

    fn led_status(&self) {
        todo!()
    }

    // Returns whether an LED is on based on the led code
    fn led_on(&mut self, led_code: u8) -> bool {
        self.leds & led_code > 0
    }
}
