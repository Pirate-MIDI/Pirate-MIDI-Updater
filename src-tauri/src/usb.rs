use fs_extra::file::TransitProcess;
use futures::channel::mpsc;
use futures::channel::mpsc::Receiver;
use futures::SinkExt;
use futures::StreamExt;
use log::debug;
use log::error;
use serde::Serialize;
use std::path::Path;
use tauri::{AppHandle, Manager};
use ts_rs::TS;
use usb_enumeration::Event as UsbEvent;
use usb_enumeration::{Event, Observer};

use crate::device::ConnectedDevice;
use crate::device::ConnectedDeviceType;
use crate::dfu::install_bridge;
use crate::dfu::install_rpi;
use crate::error::Result;
use crate::state::InstallState;
use crate::state::InstallerState;
use crate::USB_POLL_INTERVAL;

#[derive(TS, Serialize, Clone, Debug)]
#[ts(export)]
pub enum InstallStatus {
    Preparing,
    Installing,
}

#[derive(TS, Serialize, Clone, Debug)]
#[ts(export)]
pub struct InstallProgress {
    status: InstallStatus,
    progress: u32,
}

// valid devices have a known device type, and have alphanumeric serial numbers
fn is_valid_device(device: &ConnectedDevice) -> bool {
    device.device_type != ConnectedDeviceType::Unknown
        && device.serial_number.is_some()
        && device
            .serial_number
            .as_ref()
            .unwrap()
            .chars()
            .all(char::is_alphanumeric)
}

fn install_bridge_devices(handle: AppHandle, binary: &Path) -> Result<()> {
    // these values are for tracking install progress
    let total_bytes = binary.metadata().unwrap().len() as f32;
    let mut total_copied_bytes: f32 = 0.0;

    // send initial update
    handle
        .emit_all(
            "install_progress",
            InstallProgress {
                status: InstallStatus::Preparing,
                progress: 0,
            },
        )
        .unwrap();

    // this is our install progress callback handler - passed to the installer
    let progress_handler = move |copied_bytes: usize| {
        // determine percentage
        total_copied_bytes += copied_bytes as f32;
        let percentage = ((total_copied_bytes / total_bytes) * 100.0).round() as u32;
        debug!("total bytes: {total_bytes}, total copied: {total_copied_bytes}, copied: {copied_bytes}, percentage: {percentage}");

        // send progress
        handle
            .emit_all(
                "install_progress",
                InstallProgress {
                    status: InstallStatus::Installing,
                    progress: percentage,
                },
            )
            .unwrap();

        // send post install message
        if percentage >= 100 {
            handle
                .state::<InstallState>()
                .post_install_transition(&handle)
                .unwrap();
        }
    };

    // call the installation method - returns Result<()>
    install_bridge(binary.to_path_buf(), progress_handler)
}

fn install_rpi_devices(handle: AppHandle, binary: &Path) -> Result<u64> {
    // send initial update
    handle
        .emit_all(
            "install_progress",
            InstallProgress {
                status: InstallStatus::Preparing,
                progress: 0,
            },
        )
        .unwrap();

    // this is our install progress callback handler - passed to the installer
    let progress_handler = |process_info: TransitProcess| {
        // determine percentage
        let percentage = ((process_info.copied_bytes as f32 / process_info.total_bytes as f32)
            * 100.0)
            .round() as u32;

        // send progress
        handle
            .emit_all(
                "install_progress",
                InstallProgress {
                    status: InstallStatus::Installing,
                    progress: percentage,
                },
            )
            .unwrap();

        // send post install message
        if percentage >= 100 {
            handle
                .state::<InstallState>()
                .post_install_transition(&handle)
                .unwrap();
        }
    };

    // call the installation method - returns Result<u64>
    install_rpi(binary.to_path_buf(), progress_handler)
}

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

pub fn setup_usb_listener(handle: AppHandle) {
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
                        .map(ConnectedDevice::from)
                        .filter(is_valid_device)
                        .collect();

                    // get all device info for all devices
                    for arriving in &mut connected_devices {
                        match arriving.try_get_all_device_info().await {
                            Ok(_) => state.add_device(arriving.clone(), &emitter).unwrap(),
                            Err(err) => error!("error getting device details: {:?}", err),
                        }
                    }
                }
                UsbEvent::Connect(device) => {
                    // convert the device to an expected structure
                    let mut arriving = ConnectedDevice::from(&device);

                    if is_valid_device(&arriving) {
                        // get all device info
                        match arriving.try_get_all_device_info().await {
                            Ok(_) => (), // do nothing on success
                            Err(err) => error!("error getting device details: {:?}", err),
                        }

                        // get the mutex to update the state
                        let read_guard = state.current_state.read().unwrap();

                        // read the current state
                        match read_guard.clone() {
                            // if we're in the initial state, and if the device matches an expected device type
                            // then add it to the list of connected devices
                            InstallerState::Init | InstallerState::PostInstall => {
                                match &arriving.device_type {
                                    ConnectedDeviceType::Bridge4
                                    | ConnectedDeviceType::Bridge6
                                    | ConnectedDeviceType::Click
                                    | ConnectedDeviceType::ULoop => {
                                        state.add_device(arriving, &emitter).unwrap()
                                    }
                                    _ => (),
                                }
                            }
                            // if we're in bootloader state, take the device and attempt to update it.
                            InstallerState::Bootloader { device, binary } => {
                                // drop the reader so we don't deadlock in case we need to write
                                drop(read_guard);

                                // REMEMBER: the device type is the device that was selected in the list before the bootloader mode
                                // if we have a bootloader mode device, then we're in a recovery mode for that device
                                match device.device_type {
                                    ConnectedDeviceType::Bridge4
                                    | ConnectedDeviceType::Bridge6
                                    | ConnectedDeviceType::BridgeBootloader => {
                                        match install_bridge_devices(emitter.app_handle(), &binary)
                                        {
                                            Ok(_) => (), // do nothing
                                            Err(err) => {
                                                error!("unable to continue install: {:?}", err)
                                            }
                                        }
                                    }
                                    ConnectedDeviceType::Click
                                    | ConnectedDeviceType::ULoop
                                    | ConnectedDeviceType::RPBootloader => {
                                        match install_rpi_devices(emitter.app_handle(), &binary) {
                                            Ok(_) => (), // do nothing
                                            Err(err) => {
                                                error!("unable to continue install: {:?}", err)
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        };
                    }
                }
                UsbEvent::Disconnect(device) => {
                    let leaving = ConnectedDevice::from(&device);
                    if leaving.device_type != ConnectedDeviceType::Unknown {
                        state.remove_device(leaving, &emitter).unwrap();
                    }
                }
            }
        }
    });
}
