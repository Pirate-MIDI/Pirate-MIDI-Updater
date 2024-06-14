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
use tauri::{api::path::app_log_dir, CustomMenuItem, Manager, Menu, Submenu};

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
const DFUSE_DEFAULT_ADDRESS: u32 = 0x08000000;
// github
const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_BRIDGE_REPO: &str = "Pirate-MIDI-BridgeOS";
const GITHUB_CLICK_REPO: &str = "Pirate-MIDI-CLiCK";
const GITHUB_ULOOP_REPO: &str = "Pirate-MIDI-uLoop";
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
    let logging_path = match app_log_dir(context.config()) {
        Some(path) => path,
        None => PathBuf::from("."),
    };
    let log_file_path = logging_path.join(launch_time);

    // setup the sentry client
    let client = sentry_tauri::sentry::init((
        "https://c01c6e44f7ba49dab4908e3654de6dc5@o4504839482507264.ingest.sentry.io/4504839485652992",
        sentry_tauri::sentry::ClientOptions{
            release: sentry_tauri::sentry::release_name!(),
            ..Default::default()
        }
    ));

    // setup the terminal logger
    let term_logger = SimpleLogger::new(log::LevelFilter::Info, Config::default());

    // setup the local file logger
    let comb_logger = match File::create(&log_file_path) {
        Ok(writer) => {
            let file_logger = WriteLogger::new(log::LevelFilter::Trace, Config::default(), writer);
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

    let _guard = sentry_tauri::minidump::init(&client);

    // log menu
    let menu_log_path = CustomMenuItem::new("open_log_path", "Open Log Directory");
    let menu_log_file = CustomMenuItem::new("open_log_file", "Open Current Log");
    let log_submenu = Submenu::new(
        "Logs",
        Menu::new().add_item(menu_log_path).add_item(menu_log_file),
    );

    // help menu
    let menu_help_discord =
        CustomMenuItem::new("open_help_discord", "Community Support via Discord");
    let menu_help_facebook =
        CustomMenuItem::new("open_menu_facebook", "Community Support via Facebook");
    let menu_help_learn = CustomMenuItem::new("open_help_learn", "Visit the Learning Center");
    let menu_help_email = CustomMenuItem::new("open_help_email", "Email Support");
    let help_submenu = Submenu::new(
        "Help",
        Menu::new()
            .add_item(menu_help_email)
            .add_item(menu_help_learn)
            .add_item(menu_help_discord)
            .add_item(menu_help_facebook),
    );

    let menu = tauri::Menu::os_default(&context.package_info().name)
        .add_submenu(log_submenu)
        .add_submenu(help_submenu);

    // build app + run
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(move |event| match event.menu_item_id() {
            "open_log_path" => open::that_detached(&logging_path).unwrap(),
            "open_log_file" => open::that_detached(&log_file_path).unwrap(),
            "open_help_email" => open::that_detached("mailto:info@piratemidi.com").unwrap(),
            "open_help_learn" => open::that_detached("https://learn.piratemidi.com").unwrap(),
            "open_help_discord" => open::that_detached("https://discord.gg/x722K7ksA6").unwrap(),
            "open_menu_facebook" => {
                open::that_detached("https://facebook.com/groups/pirate.midi.users").unwrap()
            }
            _ => todo!("unimplemented menu item!"),
        })
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
        .plugin(sentry_tauri::plugin())
        .invoke_handler(tauri::generate_handler![
            crate::commands::github::fetch_releases,
            crate::commands::install::local_binary,
            crate::commands::install::remote_binary,
            crate::commands::install::post_install,
        ])
        .run(context)
        .expect("error while running tauri application");
}
