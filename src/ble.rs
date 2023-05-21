use alloc::sync::Arc;
use esp32_nimble::{
    utilities::mutex::Mutex, BLEAdvertising, BLECharacteristic, BLEDevice, BLEHIDDevice,
};
use log::*;

use crate::constants::*;

use super::keyboard::Report;

type SharedBleCharacteristic = Arc<Mutex<BLECharacteristic>>;

pub struct BLEKeyboard<'a> {
    key_report: Report,
    keys: SharedBleCharacteristic,
    media_keys: SharedBleCharacteristic,
    recv: SharedBleCharacteristic,
    advertising: &'a mut BLEAdvertising,
    connected_devices: u32,
}

impl BLEKeyboard<'_> {
    pub fn new(name: &str, manufacturer: &str) -> Self {
        debug!("Getting BLE Device");
        let ble_device = BLEDevice::take();
        ble_device.security().set_auth(true, true, true);

        debug!("Creating BLE Service");
        let server = ble_device.get_server();

        let mut hid = BLEHIDDevice::new(server);
        hid.manufacturer(manufacturer);
        hid.pnp(0x02, VENDOR_ID, PRODUCT_ID, VERSION);
        hid.hid_info(0x00, 0x01);
        hid.set_battery_level(100);

        hid.report_map(HID_REPORT_DESCRIPTOR);

        // type SharedBleCharacteristic = Arc<Mutex<RawMutex, BLECharacteristics>>;
        let keys = hid.input_report(KEYBOARD_ID);
        let media_keys = hid.input_report(MEDIA_KEYS_ID);
        let recv = hid.output_report(KEYBOARD_ID);

        let advertising: &mut BLEAdvertising = ble_device.get_advertising();
        advertising
            .appearance(HID_KEYBOARD)
            .name(name)
            .scan_response(false)
            .add_service_uuid(hid.hid_service().lock().uuid());

        let mut device = BLEKeyboard {
            key_report: Report::default(),
            keys,
            media_keys,
            recv,
            advertising,
            connected_devices: 0,
        };

        device.start_advertising();

        server.on_connect(move |_| {
            device.connected_devices += 1;
            info!("Client connected");
            info!("Multi-connect support: start advertising again!");
            ble_device.get_advertising().start().unwrap();
        });

        device.keys.lock().on_write(|buffer, size| {
            debug!("Input keyboard on write");
        });

        device.media_keys.lock().on_write(|buffer, size| {
            debug!("Input media keys on write");
        });

        device
    }

    pub fn start_advertising(&mut self) {
        match self.advertising.start() {
            Ok(_) => info!("Advertising started"),
            Err(e) => error!("Error starting advertising: {:?}", e),
        }
    }

    unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
    }

    pub fn send_report(&mut self, report: &Report) {
        debug!("Sending report!");
        if self.connected_devices > 0 {
            let data = unsafe { Self::any_as_u8_slice(&self.key_report) };
            let mut keys_ble = self.keys.lock();

            keys_ble.set_value(data);
            keys_ble.notify();
        }
    }
}
