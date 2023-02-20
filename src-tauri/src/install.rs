use std::time::Duration;

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

            // sleep to allow disk to mount
            std::thread::sleep(Duration::from_secs(1));

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
                Some(disk) => match std::fs::copy(
                    binary.clone(),
                    disk.mount_point()
                        .with_file_name(binary.file_name().unwrap()),
                ) {
                    Ok(bytes) => info!("moved {} bytes", bytes),
                    Err(err) => panic!("issue copying file to device: {:?}", err),
                },
                None => panic!("Unable to find UF2 device!"),
            }
        }
        InstallerState::Installing {
            device,
            binary,
            message,
            progress,
        } => todo!(),
    }
}
