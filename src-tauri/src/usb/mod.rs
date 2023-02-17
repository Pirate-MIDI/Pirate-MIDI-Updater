use std::{thread, time::Duration};

use futures::StreamExt;
use log::*;
use tauri::{AppHandle, Manager};
use usb_enumeration::Event as UsbEvent;

use self::device::ConnectedDevice;

pub mod device;
mod watcher;

fn determine_connected_emit_event(handle: &AppHandle, device: &ConnectedDevice) {
    match &device.device_type {
        Some(device_type) => match device_type {
            device::ConnectedDeviceType::Bridge6
            | device::ConnectedDeviceType::Bridge4
            | device::ConnectedDeviceType::Click
            | device::ConnectedDeviceType::ULoop => {
                handle.emit_all("device_connected", device).unwrap()
            }
            device::ConnectedDeviceType::BridgeBootloader
            | device::ConnectedDeviceType::RPBootloader => {
                handle.emit_all("installing", device).unwrap()
            }
        },
        None => (), // do nothing
    }
}

pub fn setup_usb_listener(handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    tauri::async_runtime::spawn(async move {
        let mut subscription = watcher::subscribe();
        loop {
            let event = subscription.select_next_some().await;
            match event {
                UsbEvent::Initial(devices) => {
                    trace!("initial devices detected: {:?}", devices);

                    // change our device type
                    let connected_devices: Vec<ConnectedDevice> = devices
                        .iter()
                        .map(|device| ConnectedDevice::from(device))
                        .collect();

                    // pause the thread for 0.5 second to give our UI a chance to get setup
                    // alternatively, we could have our UI pull the available devices
                    // ...but pushing is much easier
                    thread::sleep(Duration::from_millis(500));

                    // send out events about the newly connected devices
                    for device in connected_devices {
                        debug!("supported device connected: {:?}", device);
                        determine_connected_emit_event(&handle, &device);
                    }
                }
                UsbEvent::Connect(device) => {
                    trace!("new device connected: {:?}", device);
                    let connected_device = ConnectedDevice::from(&device);
                    if connected_device.device_type.is_some() {
                        debug!("supported device connected: {:?}", connected_device);
                        determine_connected_emit_event(&handle, &connected_device);
                    }
                }
                UsbEvent::Disconnect(device) => {
                    trace!("new device disconnected: {:?}", device);
                    let disconnected_device = ConnectedDevice::from(&device);
                    if disconnected_device.device_type.is_some() {
                        debug!("supported device disconnected: {:?}", disconnected_device);
                        handle
                            .emit_all("device_disconnected", disconnected_device)
                            .unwrap();
                    }
                }
            }
        }
    });
    Ok(())
}
