use std::{
    path::PathBuf,
    sync::{RwLock, RwLockWriteGuard, TryLockError},
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
        device: ConnectedDevice,
        binary: PathBuf,
    },
}

#[derive(Default)]
pub struct InstallState {
    pub devices: RwLock<Vec<ConnectedDevice>>,
    pub current_state: RwLock<InstallerState>,
}

impl InstallState {
    fn emit_device_update(guard: RwLockWriteGuard<Vec<ConnectedDevice>>, handle: &AppHandle) {
        debug!("emitted: {:?}", guard);
        handle.emit_all("devices_update", guard.clone()).unwrap();
    }

    fn emit_state_update(guard: RwLockWriteGuard<InstallerState>, handle: &AppHandle) {
        debug!("emitted: {:?}", guard);
        handle.emit_all("installer_state", guard.clone()).unwrap();
    }

    pub fn add_devices(
        &self,
        devices: &mut Vec<ConnectedDevice>,
        handle: &AppHandle,
    ) -> std::result::Result<(), TryLockError<RwLockWriteGuard<Vec<ConnectedDevice>>>> {
        match self.devices.try_write() {
            Ok(mut guard) => {
                guard.append(devices);
                InstallState::emit_device_update(guard, handle);

                Ok(())
            }
            Err(err) => {
                error!("unable to get lock: {:?}", err);
                Err(err)
            }
        }
    }

    pub fn add_device(
        &self,
        device: ConnectedDevice,
        handle: &AppHandle,
    ) -> std::result::Result<(), TryLockError<RwLockWriteGuard<Vec<ConnectedDevice>>>> {
        match self.devices.try_write() {
            Ok(mut guard) => {
                guard.push(device);
                InstallState::emit_device_update(guard, handle);

                Ok(())
            }
            Err(err) => {
                error!("unable to get lock: {:?}", err);
                Err(err)
            }
        }
    }

    pub fn remove_device(
        &self,
        device: ConnectedDevice,
        handle: &AppHandle,
    ) -> std::result::Result<(), TryLockError<RwLockWriteGuard<Vec<ConnectedDevice>>>> {
        match self.devices.try_write() {
            Ok(mut guard) => {
                guard.retain(|d| d.serial_number != device.serial_number);
                InstallState::emit_device_update(guard, handle);
                Ok(())
            }
            Err(err) => {
                error!("unable to get lock: {:?}", err);
                Err(err)
            }
        }
    }

    pub fn init_transition(&self, handle: &AppHandle) -> Result<()> {
        match self.current_state.try_write() {
            Ok(mut guard) => {
                *guard = InstallerState::Init;
                InstallState::emit_state_update(guard, handle);
                Ok(())
            }
            Err(err) => {
                error!("unable to get lock: {:?}", err);
                Err(crate::error::Error::Other(format!(
                    "unable to get lock: {:?}",
                    err
                )))
            }
        }
    }

    pub fn bootloader_transition(
        &self,
        device: ConnectedDevice,
        binary: PathBuf,
        handle: &AppHandle,
    ) -> Result<()> {
        match self.current_state.try_write() {
            Ok(mut guard) => {
                *guard = InstallerState::Bootloader {
                    device: device.clone(),
                    binary: binary,
                };

                // enter the bootloader
                match device.enter_bootloader() {
                    Ok(_) => {
                        InstallState::emit_state_update(guard, handle);
                        Ok(())
                    }
                    Err(err) => err!(crate::error::Error::Bootloader(err.to_string())),
                }
            }
            Err(err) => Err(crate::error::Error::Other(format!(
                "unable to get lock: {:?}",
                err
            ))),
        }
    }
}
