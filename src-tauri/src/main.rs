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
use simplelog::{CombinedLogger, Config, SimpleLogger, WriteLogger};
use state::InstallState;
use std::{fs::File, path::PathBuf, time::Duration};
use tauri::{api::path::app_log_dir, Manager};

// modules
mod commands;
mod device;
mod dfu;
mod error;
mod github;
mod state;
mod usb;
mod validation;

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

    // launch time
    let mut launch_time = chrono::offset::Utc::now()
        .format("%Y-%m-%d-%H:%M")
        .to_string();

    // append ".txt" to the end of the launch time for filename usage
    launch_time.push_str(".txt");

    // setup the log file path
    let log_file_path = match app_log_dir(context.config()) {
        Some(path) => path,
        None => PathBuf::from("."),
    }
    .join(launch_time);

    // setup the app
    sentry_tauri::init(
        sentry::release_name!(),
        |_| {
            // setup the terminal logger
            let term_logger = SimpleLogger::new(log::LevelFilter::Info, Config::default());

            // setup the local file logger
            let comb_logger = match File::create(&log_file_path) {
                Ok(writer) => {
                    let file_logger =
                        WriteLogger::new(log::LevelFilter::Trace, Config::default(), writer);
                    CombinedLogger::new(vec![term_logger, file_logger])
                }
                Err(_) => CombinedLogger::new(vec![term_logger]),
            };

            // tie the local logger(s) to sentry
            let logger = sentry_log::SentryLogger::with_dest(comb_logger);
            log::set_max_level(log::LevelFilter::Trace);
            log::set_boxed_logger(Box::new(logger)).unwrap();

            // print where the log is going to get written
            info!("log file location: {}", log_file_path.display());

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
