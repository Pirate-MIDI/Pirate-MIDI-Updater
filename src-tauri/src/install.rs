use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    time::Duration,
};

use log::{debug, info};
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};
use tauri::{AppHandle, Manager};

use crate::{InstallState, InstallerState};

pub fn install_rpi(handle: AppHandle) {
    // get our state, and our mutex
    let state = handle.state::<InstallState>();
    let mut state_guard = state.current_state.lock().unwrap();

    match state_guard.clone() {
        InstallerState::Init => todo!(),
        InstallerState::EnterBootloader { device, binary } => {
            // send install signal
            *state_guard = InstallerState::Installing {
                device: device.clone(),
                binary: binary.clone(),
                message: "installing".to_string(),
                progress: 0,
            };

            handle
                .emit_all("installer_state", state_guard.clone())
                .unwrap();

            // sleep to allow disk to mount
            std::thread::sleep(Duration::from_secs(3));

            // get disk info from system
            let mut sys = System::new_with_specifics(RefreshKind::new().with_disks_list());

            // retrieve our disk info
            sys.refresh_disks_list();
            sys.refresh_disks();

            // brittle... but works
            let disks = sys.disks();
            debug!("available disks: {:?}", disks);

            let rpi_disk = disks
                .iter()
                .find(|&disk| disk.is_removable() && disk.name().eq_ignore_ascii_case("RPI-RP2"));

            match rpi_disk {
                Some(disk) => {
                    // let mut pos = 0;

                    // get our file handlers
                    let read_handle = File::open(&binary);
                    let write_handle = OpenOptions::new().write(true).create(true).open(
                        disk.mount_point()
                            .join(PathBuf::from(binary.file_name().unwrap())),
                    );

                    // while pos < data.len() {
                    //     let bytes_written = buffer.write(&data[pos..])?;
                    //     pos += bytes_written;
                    // }

                    match write_handle {
                        Ok(mut writer) => match read_handle {
                            Ok(mut reader) => match std::io::copy(&mut reader, &mut writer) {
                                Ok(bytes_written) => info!("moved {} bytes", bytes_written),
                                Err(err) => panic!("issue copying file to device: {:?}", err),
                            },
                            Err(err) => panic!("issue opening reader: {:?}", err),
                        },
                        Err(err) => panic!("issue opening writer: {:?}", err),
                    }
                }
                None => todo!(),
            }

            // send updated state
            *state_guard = InstallerState::Init;

            handle
                .emit_all("installer_state", state_guard.clone())
                .unwrap();
        }
        InstallerState::Installing {
            device: _,
            binary: _,
            message: _,
            progress: _,
        } => todo!(),
    }
}

// pub fn install_bridge(handle: AppHandle) {
//     // open the binary file and get the file size
//     let file = std::fs::File::open(binary_path)
//         .map_err(|e| CommandError::IO(format!("could not open firmware file: {}", e)))?;

//     let mut dfu_iface = {
//         //match raw_device {
//         // Some(device) => {
//         //     // get device descriptor
//         //     let (vid, pid) = match device.device_descriptor() {
//         //         Ok(desc) => (desc.vendor_id(), desc.product_id()),
//         //         Err(err) => panic!(
//         //             "unable to get device descriptors from usb device! - error: {}",
//         //             err
//         //         ),
//         //     };
//         //     // open the DFU interface
//         //     info!("opening interface: {:#06x}:{:#06x}", vid, pid);
//         //     DfuLibusb::open(device.context(), vid, pid, 0, 0)
//         //         .map_err(|e| CommandError::Dfu(e.to_string()))?
//         // }
//         // // if we didn't pass in a device, just try to guess via VID and PID
//         // None => {
//         // create new usb context
//         info!("device was not passed in - creating new usb context");
//         let context = rusb::Context::new()
//             .map_err(|e| CommandError::Device(format!("unable to create usb context: {}", e)))?;
//         // open the DFU interface
//         DfuLibusb::open(&context, USB_VENDOR_ID, USB_PRODUCT_DFU_ID, 0, 0)
//             .map_err(|e| CommandError::Dfu(e.to_string()))?
//         //}
//     };

//     // setup our progress bar - if available
//     // if progress.is_some() {
//     //     dfu_iface.with_progress(progress.unwrap());
//     // }

//     // PERFORM THE INSTALL
//     match dfu_iface.download_all(file) {
//         Ok(_) => Ok(()),
//         Err(dfu_libusb::Error::LibUsb(rusb::Error::Io)) => Ok(()),
//         Err(err) => {
//             error!("dfu download error: {}", err);
//             Err(CommandError::Dfu(err.to_string()))
//         }
//     }
// }
