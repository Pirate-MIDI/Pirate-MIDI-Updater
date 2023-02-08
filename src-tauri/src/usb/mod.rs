use futures::StreamExt;
use log::*;
use tauri::{AppHandle, Manager};
use usb_enumeration::{Event as UsbEvent, UsbDevice};

mod watcher;

pub fn setup_usb_listener(handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    tauri::async_runtime::spawn(async move {
        let mut subscription = watcher::subscribe();
        loop {
            let event = subscription.select_next_some().await;
            match event {
                UsbEvent::Initial(devices) => match devices
                    .iter()
                    .find(|device| try_get_board_info(device).is_ok())
                {
                    Some(device) => {
                        debug!("device connected: {:?}", device);
                        handle
                            .emit_all(
                                "device_connected",
                                format!("VID: {}, PID: {}", device.vendor_id, device.product_id),
                            )
                            .unwrap();
                    }
                    None => debug!("no devices connected"),
                },
                UsbEvent::Connect(device) => match try_get_board_info(&device) {
                    Ok(_) => {
                        debug!("device connected: {:?}", device);
                        handle
                            .emit_all(
                                "device_connected",
                                format!("VID: {}, PID: {}", device.vendor_id, device.product_id),
                            )
                            .unwrap();
                    }
                    Err(e) => error!("error connecting to board: {:?}", e),
                },
                UsbEvent::Disconnect(device) => {
                    debug!("device disconnected: {:?}", device);
                    handle
                        .emit_all(
                            "device_disconnected",
                            format!("VID: {}, PID: {}", device.vendor_id, device.product_id),
                        )
                        .unwrap();
                }
            }
        }
    });
    Ok(())
}

fn try_get_board_info(_device: &UsbDevice) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
