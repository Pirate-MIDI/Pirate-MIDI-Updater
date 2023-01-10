use tauri::{App, Manager};

use crate::UsbState;

use self::observer::{Event, Observer};

pub mod observer;

pub fn setup_usb_listener(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    // create a subscription and a moveable app handle
    let subscription = Observer::new()?.subscribe();
    let handle = app.app_handle();

    // start in it's own thread so we don't block the main thread
    std::thread::spawn(move || {
        // get the state so we can update it
        let state = handle.state::<UsbState>();

        // iterate through events
        for event in subscription.rx_event.iter() {
            match event {
                Event::Initial(devices) => devices.iter().for_each(|device| {
                    if device.is_stm_device() || device.is_dfu_device() {
                        state.devices.lock().unwrap().insert(device.clone());
                        let _ = handle.emit_all("device_arrived", device);
                    }
                }),
                Event::Connected(device) => {
                    if device.is_stm_device() || device.is_dfu_device() {
                        state.devices.lock().unwrap().insert(device.clone());
                        let _ = handle.emit_all("device_arrived", device);
                    }
                }
                Event::Disconnected(device) => {
                    if device.is_stm_device() || device.is_dfu_device() {
                        state.devices.lock().unwrap().remove(&device);
                        let _ = handle.emit_all("device_left", device);
                    }
                }
            }
        }
    });

    Ok(())
}
