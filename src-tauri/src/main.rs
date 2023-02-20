#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use device::ConnectedDevice;
use serde::Serialize;
use std::{path::PathBuf, sync::Mutex};
use tauri_plugin_log::LogTarget;
use ts_rs::TS;

// modules
mod bootloader;
mod commands;
mod device;
mod install;
mod usb;

/* GLOBAL CONSTANTS */
// usb / device
const USB_POLL_INTERVAL: u32 = 1; // in seconds
const USB_VENDOR_ID: u16 = 0x0483;
// const USB_PRODUCT_ID: u16 = 0x5740;
const USB_PRODUCT_DFU_ID: u16 = 0xDF11;
const USB_DEFAULT_BAUD_RATE: u32 = 9600;
const USB_RPI_BOOTLOADER_BAUD_RATE: u32 = 1200;
// github
const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_ORG: &str = "Pirate-MIDI";
const GITHUB_BRIDGE_REPO: &str = "Pirate-MIDI-BridgeOS";
const GITHUB_CLICK_REPO: &str = "Pirate-MIDI-CLiCK";

#[derive(Default, TS, Serialize, Clone)]
#[ts(export)]
#[serde(tag = "type")]
pub enum InstallerState {
    #[default]
    Init,
    EnterBootloader {
        device: ConnectedDevice,
        binary: PathBuf,
    },
    Installing {
        device: ConnectedDevice,
        binary: PathBuf,
        message: String,
        progress: i32,
    },
}

#[derive(Default)]
pub struct InstallState {
    pub devices: Mutex<Vec<ConnectedDevice>>,
    pub current_state: Mutex<InstallerState>,
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .manage(InstallState::default())
        .setup(|app| usb::setup_usb_listener(app.handle()))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            crate::commands::github::fetch_releases,
            crate::commands::github::fetch_asset,
            crate::commands::install::local_binary,
            crate::commands::dfu::install_remote_binary,
        ])
        .run(context)
        .expect("error while running tauri application");
}
