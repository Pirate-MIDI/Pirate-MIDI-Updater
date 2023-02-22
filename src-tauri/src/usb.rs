use fs_extra::file::TransitProcess;
use futures::channel::mpsc;
use futures::channel::mpsc::Receiver;
use futures::SinkExt;
use futures::StreamExt;
use log::debug;
use log::error;
use log::info;
use tauri::{AppHandle, Manager};
use usb_enumeration::Event as UsbEvent;
use usb_enumeration::{Event, Observer};

use crate::device::ConnectedDevice;
use crate::device::ConnectedDeviceType;
use crate::dfu::install_rpi;
use crate::state::InstallState;
use crate::state::InstallerState;
use crate::USB_POLL_INTERVAL;

fn subscribe() -> Receiver<Event> {
    let (mut sender, receiver) = mpsc::channel(0);

    tauri::async_runtime::spawn(async move {
        let subscription = Observer::new()
            .with_poll_interval(USB_POLL_INTERVAL)
            .subscribe();

        for event in subscription.rx_event.iter() {
            let _ = sender.send(event).await;
        }
    });

    receiver
}

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
            let mut subscription = subscribe();
            loop {
                let event = subscription.select_next_some().await;
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

                        state.add_devices(&mut connected_devices, &emitter).unwrap();
                    }
                    UsbEvent::Connect(device) => {
                        let arriving = ConnectedDevice::from(&device);
                        let read_guard = state.current_state.read().unwrap();

                        match read_guard.clone() {
                            InstallerState::Init => match &arriving.device_type {
                                Some(device_type) => match device_type {
                                    ConnectedDeviceType::Bridge4 
                                    | ConnectedDeviceType::Bridge6  
                                    | ConnectedDeviceType::Click 
                                    | ConnectedDeviceType::ULoop => state.add_device(arriving, &emitter).unwrap(),
                                    _ => ()
                                },
                                None => (),
                            },
                            InstallerState::Bootloader { device, binary } => {
                                // drop the reader so we don't deadlock in case we need to write
                                drop(read_guard);

                                // REMEMBER: the device in this step is NOT the bootloader version
                                match device.device_type {
                                    Some(device_type) => match device_type {
                                        ConnectedDeviceType::Bridge4 | ConnectedDeviceType::Bridge6 => todo!(),
                                        ConnectedDeviceType::Click => {
                                            let progress_handler =
                                            |process_info: TransitProcess| {
                                                handle
                                                    .emit_all(
                                                        "install_progress",
                                                        ((process_info.copied_bytes as f32 / process_info.total_bytes as f32) * 100.).round() as u64,
                                                    )
                                                    .unwrap();
                                            };

                                            match install_rpi(binary, progress_handler) {
                                                Ok(bytes_written) => 
                                                {
                                                    info!("successfully wrote {bytes_written} bytes to device");
                                                    state.init_transition(&handle).unwrap();
                                                },
                                                Err(err) => error!(
                                                    "unable to upload file to device: {:?}",
                                                    err
                                                ),
                                            }
                                        },
                                        _ => (),
                                    },
                                    None => todo!(),
                                }
                            },
                        };
                    }
                    UsbEvent::Disconnect(device) => {
                        let leaving = ConnectedDevice::from(&device);
                        if leaving.device_type.is_some() {
                            state.remove_device(leaving, &emitter).unwrap();
                        }
                    }
                }
            }
        });
    });
    Ok(())
}
