#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

macro_rules! err {
    ($kind:expr) => {
        return Err($kind)
    };

    ($text:expr) => {
        err!(ErrorKind::Other($text))
    };
}

use state::InstallState;
use tauri_plugin_log::LogTarget;

// modules
mod commands;
mod device;
mod dfu;
mod error;
mod github;
mod state;
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
            crate::commands::install::local_binary,
            crate::commands::install::remote_binary,
        ])
        .run(context)
        .expect("error while running tauri application");
}
