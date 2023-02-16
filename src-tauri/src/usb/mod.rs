use futures::StreamExt;
use log::*;
use tauri::{AppHandle, Manager};
use usb_enumeration::Event as UsbEvent;

use self::device::ConnectedDevice;

pub mod device;
mod watcher;

pub fn setup_usb_listener(handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    tauri::async_runtime::spawn(async move {
        let mut subscription = watcher::subscribe();
        loop {
            let event = subscription.select_next_some().await;
            match event {
                UsbEvent::Initial(devices) => {
                    trace!("initial devices detected: {:?}", devices);
                    // filter out devices
                    let connected_devices: Vec<ConnectedDevice> = devices
                        .iter()
                        .map(|device| ConnectedDevice::from(device))
                        .filter(|device| device.device_type.is_some())
                        .collect();

                    // sent out events about the newly connected devices
                    for device in connected_devices {
                        debug!("supported device connected: {:?}", device);
                        handle.emit_all("device_connected", device).unwrap();
                    }
                }
                UsbEvent::Connect(device) => {
                    trace!("new device connected: {:?}", device);
                    let connected_device = ConnectedDevice::from(&device);
                    if connected_device.device_type.is_some() {
                        debug!("supported device connected: {:?}", connected_device);
                        handle
                            .emit_all("device_connected", connected_device)
                            .unwrap();
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
