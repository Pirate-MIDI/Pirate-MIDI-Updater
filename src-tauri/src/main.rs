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

use log::info;
use state::InstallState;
use std::time::Duration;
use tauri::Manager;

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
const USB_TIMEOUT: Duration = Duration::from_secs(1);
// github
const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_BRIDGE_REPO: &str = "Pirate-MIDI-BridgeOS";
const GITHUB_CLICK_REPO: &str = "Pirate-MIDI-CLiCK";
const GITHUB_ORG: &str = "Pirate-MIDI";

fn main() {
    let context = tauri::generate_context!();

    // setup the app
    sentry_tauri::init(
        sentry::release_name!(),
        |_| {
            // set up logging for sentry - assign it to the global logger
            let dest = stderrlog::new().verbosity(log::Level::Trace).clone();
            let logger = sentry_log::SentryLogger::with_dest(dest);
            log::set_max_level(log::LevelFilter::Info);
            log::set_boxed_logger(Box::new(logger)).unwrap();

            // initialize the sentry instance
            sentry::init(("https://c01c6e44f7ba49dab4908e3654de6dc5@o4504839482507264.ingest.sentry.io/4504839485652992", sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            }))
        },
        |sentry_plugin| {
            tauri::Builder::default()
                .menu(tauri::Menu::os_default(&context.package_info().name))
                .manage(InstallState::default())
                .setup(|app| {
                    // listen for the 'ready' event - but we only need to hear it one time
                    let handle = app.app_handle();
                    app.app_handle().once_global("ready", move |_| {
                        info!("ready event recieved");
                        usb::setup_usb_listener(handle);
                    });

                    Ok(())
                })
                .plugin(sentry_plugin)
                .invoke_handler(tauri::generate_handler![
                    crate::commands::github::fetch_releases,
                    crate::commands::install::local_binary,
                    crate::commands::install::remote_binary,
                    crate::commands::install::post_install,
                ])
                .run(context)
                .expect("error while running tauri application");
        },
    );
}
