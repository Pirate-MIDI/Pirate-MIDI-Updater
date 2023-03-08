use tauri::api::dialog::blocking::FileDialogBuilder;

use crate::{
    commands::github::fetch_compatable_asset,
    device::{ConnectedDevice, ConnectedDeviceType},
    error::{Error, Result},
    github::Release,
    state::InstallState,
};

#[tauri::command]
pub async fn local_binary(
    device: ConnectedDevice,
    state: tauri::State<'_, InstallState>,
    handle: tauri::AppHandle,
) -> Result<()> {
    // select the file type filter based on the device type
    let file_type = match &device.device_type {
        ConnectedDeviceType::Bridge6 | ConnectedDeviceType::Bridge4 => "bin",
        ConnectedDeviceType::Click | ConnectedDeviceType::ULoop => "uf2",
        _ => "",
    };

    let local_file_path = FileDialogBuilder::new()
        .add_filter("Firmware Binary", &[file_type])
        .set_title("Select the firmware file")
        .pick_file();

    match local_file_path {
        Some(file_path) => {
            state
                .bootloader_transition(device, file_path, &handle)
                .unwrap();

            Ok(())
        }
        None => Err(Error::IO("local file selection cancelled".to_string())),
    }
}

#[tauri::command]
pub async fn remote_binary(
    device: ConnectedDevice,
    release: Release,
    state: tauri::State<'_, InstallState>,
    handle: tauri::AppHandle,
) -> Result<()> {
    // retrieve the remote binary
    match fetch_compatable_asset(&device, release).await {
        Ok(file_path) => state.bootloader_transition(device, file_path, &handle),
        Err(err) => err!(Error::Other(format!(
            "unable to retrieve asset: {:?}",
            err.to_string()
        ))),
    }
}

#[tauri::command]
pub fn post_install(state: tauri::State<'_, InstallState>, handle: tauri::AppHandle) -> Result<()> {
    state.init_transition(&handle)
}
