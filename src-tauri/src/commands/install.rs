use tauri::api::dialog::blocking::FileDialogBuilder;

use crate::{
    commands::github::fetch_compatable_asset,
    device::{ConnectedDevice, ConnectedDeviceType},
    error::{Error, Result},
    github::Release,
    state::InstallState,
};

#[tauri::command]
pub fn local_binary(
    device: ConnectedDevice,
    state: tauri::State<InstallState>,
    handle: tauri::AppHandle,
) -> Result<()> {
    // select the file type filter based on the device type
    let file_type = match &device.device_type {
        Some(device_type) => match device_type {
            ConnectedDeviceType::Bridge6 | ConnectedDeviceType::Bridge4 => "bin",
            ConnectedDeviceType::Click | ConnectedDeviceType::ULoop => "uf2",
            _ => "",
        },
        None => "",
    };

    // get the local file path
    // let local_file_path = match dialog::select(file_type, Some("")) {
    //     Ok(response) => match response {
    //         dialog::Response::Okay(selected_path) => Some(selected_path),
    //         dialog::Response::OkayMultiple(_) | dialog::Response::Cancel => {
    //             debug!("local file selection cancelled");
    //             None
    //         }
    //     },
    //     Err(e) => {
    //         info!("local file selection cancelled: {:?}", e);
    //         None
    //     }
    // };

    let local_file_path = FileDialogBuilder::new()
        .add_filter("Firmware Binary", &[file_type])
        .set_title("Select the firmware file")
        .pick_file();

    match local_file_path {
        Some(file_path) => {
            state
                .bootloader_transition(device, file_path.into(), &handle)
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
        Ok(file_path) => state.bootloader_transition(device, file_path.into(), &handle),
        Err(err) => err!(Error::Other(format!(
            "unable to retrieve asset: {:?}",
            err.to_string()
        ))),
    }
}
