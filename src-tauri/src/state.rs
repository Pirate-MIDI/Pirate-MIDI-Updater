use std::{
    path::PathBuf,
    sync::{PoisonError, RwLock, RwLockWriteGuard},
};

use log::{debug, error};
use serde::Serialize;
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use crate::{device::ConnectedDevice, error::Result};

#[derive(Default, TS, Serialize, Clone, Debug)]
#[ts(export)]
#[serde(tag = "type")]
pub enum InstallerState {
    #[default]
    Init,
    Bootloader {
        device: Box<ConnectedDevice>,
        binary: PathBuf,
    },
    PostInstall,
}

#[derive(Default)]
pub struct InstallState {
    pub devices: RwLock<Vec<ConnectedDevice>>,
    pub current_state: RwLock<InstallerState>,
}

impl InstallState {
    fn emit_device_update(&self, handle: &AppHandle) {
        let devices = self.devices.read().unwrap();
        debug!("emitted: {:?}", devices);
        handle.emit_all("devices_update", devices.clone()).unwrap();
    }

    fn emit_state_update(&self, handle: &AppHandle) {
        let current_state = self.current_state.read().unwrap();
        debug!("emitted: {:?}", current_state);
        handle
            .emit_all("installer_state", current_state.clone())
            .unwrap();
    }

    pub fn add_device(
        &self,
        device: ConnectedDevice,
        handle: &AppHandle,
    ) -> std::result::Result<(), PoisonError<RwLockWriteGuard<Vec<ConnectedDevice>>>> {
        let write = match self.devices.write() {
            Ok(mut guard) => {
                guard.push(device);
                Ok(())
            }
            Err(err) => {
                error!("unable to get lock: {:?}", err);
                Err(err)
            }
        };

        match write {
            Ok(_) => {
                self.emit_device_update(handle);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn remove_device(
        &self,
        device: ConnectedDevice,
        handle: &AppHandle,
    ) -> std::result::Result<(), PoisonError<RwLockWriteGuard<Vec<ConnectedDevice>>>> {
        let write = match self.devices.write() {
            Ok(mut guard) => {
                guard.retain(|d| d.serial_number != device.serial_number);
                Ok(())
            }
            Err(err) => {
                error!("unable to get lock: {:?}", err);
                Err(err)
            }
        };

        match write {
            Ok(_) => {
                self.emit_device_update(handle);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn init_transition(&self, handle: &AppHandle) -> Result<()> {
        let write = match self.current_state.write() {
            Ok(mut guard) => {
                *guard = InstallerState::Init;
                Ok(())
            }
            Err(err) => {
                error!("unable to get lock: {:?}", err);
                Err(crate::error::Error::Other(format!(
                    "unable to get lock: {:?}",
                    err
                )))
            }
        };

        match write {
            Ok(_) => {
                self.emit_state_update(handle);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn bootloader_transition(
        &self,
        device: ConnectedDevice,
        binary: PathBuf,
        handle: &AppHandle,
    ) -> Result<()> {
        let write = match self.current_state.write() {
            Ok(mut guard) => {
                // enter the bootloader
                match &device.enter_bootloader() {
                    Ok(_) => {
                        // update the state and emit it
                        *guard = InstallerState::Bootloader {
                            device: Box::new(device),
                            binary,
                        };
                        Ok(())
                    }
                    Err(err) => err!(crate::error::Error::Bootloader(err.to_string())),
                }
            }
            Err(err) => Err(crate::error::Error::Other(format!(
                "unable to get lock: {:?}",
                err
            ))),
        };

        match write {
            Ok(_) => {
                self.emit_state_update(handle);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn post_install_transition(&self, handle: &AppHandle) -> Result<()> {
        let write = match self.current_state.write() {
            Ok(mut guard) => {
                *guard = InstallerState::PostInstall;
                Ok(())
            }
            Err(err) => {
                error!("unable to get lock: {:?}", err);
                Err(crate::error::Error::Other(format!(
                    "unable to get lock: {:?}",
                    err
                )))
            }
        };

        match write {
            Ok(_) => {
                self.emit_state_update(handle);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
