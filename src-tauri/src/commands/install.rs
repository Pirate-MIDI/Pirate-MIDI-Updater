use std::path::PathBuf;

use log::{debug, info};
use tauri::Manager;
use tauri_api::dialog;

use crate::{
    bootloader::enter_bootloader,
    device::{ConnectedDevice, ConnectedDeviceType},
    InstallState, InstallerState,
};

use super::CommandError;

#[tauri::command]
pub fn local_binary(
    device: ConnectedDevice,
    state: tauri::State<InstallState>,
    handle: tauri::AppHandle,
) -> Result<(), CommandError> {
    // select the file type filter based on the device type
    let file_type = match &device.device_type {
        Some(device_type) => match device_type {
            ConnectedDeviceType::Bridge6 | ConnectedDeviceType::Bridge4 => Some("bin"),
            ConnectedDeviceType::Click | ConnectedDeviceType::ULoop => Some("uf2"),
            _ => None,
        },
        None => None,
    };

    // get the local file path
    let local_file_path = match dialog::select(file_type, Some("")) {
        Ok(response) => match response {
            dialog::Response::Okay(selected_path) => Some(selected_path),
            dialog::Response::OkayMultiple(_) | dialog::Response::Cancel => {
                debug!("local file selection cancelled");
                None
            }
        },
        Err(e) => {
            info!("local file selection cancelled: {:?}", e);
            None
        }
    };

    match local_file_path {
        Some(file_path) => match enter_bootloader(&device) {
            Ok(_) => {
                // update the state to bootloading mode!
                let mut state_guard = state.current_state.lock().unwrap();
                *state_guard = InstallerState::EnterBootloader {
                    device: device,
                    binary: PathBuf::from(file_path),
                };

                // signal to the frontend that we're entering the installer
                handle
                    .emit_all("installer_state", state_guard.clone())
                    .unwrap();
                Ok(())
            }
            Err(err) => Err(err),
        },
        None => Err(CommandError::Device(
            "Unable to find local file, cancelling install".to_string(),
        )),
    }
}
