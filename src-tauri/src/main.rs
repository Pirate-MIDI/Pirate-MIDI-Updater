#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::PathBuf, sync::Mutex};
use tauri_plugin_log::LogTarget;
use usb::device::ConnectedDevice;

// modules
mod commands;
mod usb;

// GLOBALS
const USB_VENDOR_ID: u16 = 0x0483;
// const USB_PRODUCT_ID: u16 = 0x5740;
const USB_PRODUCT_DFU_ID: u16 = 0xDF11;
const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_ORG: &str = "Pirate-MIDI";
const BRIDGE_GITHUB_REPO: &str = "Pirate-MIDI-BridgeOS";
const CLICK_GITHUB_REPO: &str = "Pirate-MIDI-CLiCK";

#[derive(Default)]
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
    },
}

pub struct InstallState {
    pub current_state: Mutex<InstallerState>,
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .setup(|app| {
            // setup our global usb listener
            usb::setup_usb_listener(app.handle())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        // .menu(menu)
        .manage(InstallState {
            current_state: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::github::fetch_releases,
            crate::commands::github::fetch_asset,
            crate::commands::install::local_binary,
            crate::commands::dfu::install_remote_binary,
        ])
        .run(context)
        .expect("error while running tauri application");
}
