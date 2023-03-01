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

use log::debug;
use state::InstallState;
use tauri::Manager;
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
const USB_BRIDGE_VENDOR_ID: u16 = 0x0483;
const USB_BRIDGE_PRODUCT_DFU_ID: u16 = 0xDF11;
const USB_DEFAULT_BAUD_RATE: u32 = 9600;
const USB_POLL_INTERVAL: u32 = 1; // in seconds
const USB_RPI_BOOTLOADER_BAUD_RATE: u32 = 1200;
// github
const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_BRIDGE_REPO: &str = "Pirate-MIDI-BridgeOS";
const GITHUB_CLICK_REPO: &str = "Pirate-MIDI-CLiCK";
const GITHUB_ORG: &str = "Pirate-MIDI";

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .manage(InstallState::default())
        .setup(|app| {
            // listen for the 'ready' event - but we only need to hear it one time
            let handle = app.app_handle();
            app.app_handle().once_global("ready", move |_| {
                debug!("ready event recieved");
                usb::setup_usb_listener(handle);
            });

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            crate::commands::github::fetch_releases,
            crate::commands::install::local_binary,
            crate::commands::install::remote_binary,
            crate::commands::install::post_install,
        ])
        .run(context)
        .expect("error while running tauri application");
}
