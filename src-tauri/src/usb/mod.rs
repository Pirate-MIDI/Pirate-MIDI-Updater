use futures::StreamExt;
use log::debug;
use tauri::{AppHandle, Manager};
use usb_enumeration::Event as UsbEvent;

use crate::{device::ConnectedDevice, InstallState};

mod watcher;

pub fn setup_usb_listener(handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // use a new handle to add the global listener - but we only need to hear it one time
    handle.app_handle().once_global("ready", move |_| {
        debug!("ready event recieved");
        // when the ready event is detected, spawn the connection emitters
        let emitter = handle.app_handle();
        tauri::async_runtime::spawn(async move {
            // get the global state object
            let state = handle.state::<InstallState>();

            // kick off the USB subscription
            let mut subscription = watcher::subscribe();
            loop {
                let event = subscription.select_next_some().await;
                // get the mutex
                let mut device_guard = state.devices.lock().unwrap();

                debug!("new event: {:?}", event);

                // detemine what to do based on the event type
                match event {
                    UsbEvent::Initial(devices) => {
                        // change our device type
                        let mut connected_devices: Vec<ConnectedDevice> = devices
                            .iter()
                            .map(|device| ConnectedDevice::from(device))
                            .filter(|device| device.device_type.is_some())
                            .collect();

                        device_guard.append(&mut connected_devices);
                    }
                    UsbEvent::Connect(device) => {
                        let arriving = ConnectedDevice::from(&device);
                        if arriving.device_type.is_some() {
                            device_guard.push(arriving);
                        }
                    }
                    UsbEvent::Disconnect(device) => {
                        let leaving = ConnectedDevice::from(&device);
                        if leaving.device_type.is_some() {
                            device_guard.retain(|d| d.serial_number != leaving.serial_number);
                        }
                    }
                }

                // send the updated devices to the front end
                emitter
                    .emit_all("devices_update", device_guard.clone())
                    .unwrap();
            }
        });
    });
    Ok(())
}
