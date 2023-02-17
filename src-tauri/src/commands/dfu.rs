use std::path::PathBuf;

use dfu_libusb::DfuLibusb;
use log::{debug, error, info};
use pirate_midi_rs::{Command, ControlArgs, PirateMIDIDevice};
use serialport::{available_ports, SerialPortBuilder, SerialPortType};

pub const DEFAULT_USB_BAUD_RATE: u32 = 9600;
pub const RPI_BOOTLOADER_BAUD_RATE: u32 = 1200;

use crate::{
    usb::device::ConnectedDevice, usb::device::ConnectedDeviceType, USB_PRODUCT_DFU_ID,
    USB_VENDOR_ID,
};

use super::CommandError;

#[tauri::command]
pub async fn install_remote_binary(binary_path: PathBuf) -> Result<(), CommandError> {
    // open the binary file and get the file size
    let file = std::fs::File::open(binary_path)
        .map_err(|e| CommandError::IO(format!("could not open firmware file: {}", e)))?;

    let mut dfu_iface = {
        //match raw_device {
        // Some(device) => {
        //     // get device descriptor
        //     let (vid, pid) = match device.device_descriptor() {
        //         Ok(desc) => (desc.vendor_id(), desc.product_id()),
        //         Err(err) => panic!(
        //             "unable to get device descriptors from usb device! - error: {}",
        //             err
        //         ),
        //     };
        //     // open the DFU interface
        //     info!("opening interface: {:#06x}:{:#06x}", vid, pid);
        //     DfuLibusb::open(device.context(), vid, pid, 0, 0)
        //         .map_err(|e| CommandError::Dfu(e.to_string()))?
        // }
        // // if we didn't pass in a device, just try to guess via VID and PID
        // None => {
        // create new usb context
        info!("device was not passed in - creating new usb context");
        let context = rusb::Context::new()
            .map_err(|e| CommandError::Device(format!("unable to create usb context: {}", e)))?;
        // open the DFU interface
        DfuLibusb::open(&context, USB_VENDOR_ID, USB_PRODUCT_DFU_ID, 0, 0)
            .map_err(|e| CommandError::Dfu(e.to_string()))?
        //}
    };

    // setup our progress bar - if available
    // if progress.is_some() {
    //     dfu_iface.with_progress(progress.unwrap());
    // }

    // PERFORM THE INSTALL
    match dfu_iface.download_all(file) {
        Ok(_) => Ok(()),
        Err(dfu_libusb::Error::LibUsb(rusb::Error::Io)) => Ok(()),
        Err(err) => {
            error!("dfu download error: {}", err);
            Err(CommandError::Dfu(err.to_string()))
        }
    }
}

fn build_serialport_builder(
    device: &ConnectedDevice,
    baud_rate: u32,
) -> Result<SerialPortBuilder, CommandError> {
    match available_ports() {
        Ok(ports) => {
            for p in ports {
                debug!("reviewing port: {:?}", p);
                if let SerialPortType::UsbPort(usb_info) = p.port_type {
                    if usb_info.serial_number == device.serial_number {
                        return Ok(serialport::new(p.port_name, baud_rate));
                    }
                }
            }
            Err(CommandError::Device("unable to locate device".to_string()))
        }
        Err(err) => Err(CommandError::Device(err.to_string())),
    }
}

fn enter_bridge_bootloader(device: &ConnectedDevice) -> Result<(), CommandError> {
    match build_serialport_builder(device, DEFAULT_USB_BAUD_RATE) {
        Ok(builder) => match PirateMIDIDevice::new()
            .with_serialport_builder(builder)
            .send(Command::Control(ControlArgs::EnterBootloader))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(CommandError::Device(format!(
                "Unable to enter bootloader due to error: {}",
                err
            ))),
        },
        Err(err) => Err(err),
    }
}

// the RP2040 will immidately enter bootloader mode if you connect to it with
// a baud rate of 1200, so we're just going to quickly connect and bail
fn enter_rpi_bootloader(device: &ConnectedDevice) -> Result<(), CommandError> {
    match build_serialport_builder(device, RPI_BOOTLOADER_BAUD_RATE) {
        Ok(builder) => match builder.open() {
            Ok(_) => Ok(()),
            Err(err) => Err(CommandError::Device(format!(
                "Unable to open RP serial port due to error: {}",
                err
            ))),
        },
        Err(err) => Err(err),
    }
}

pub fn enter_bootloader(device: &ConnectedDevice) -> Result<(), CommandError> {
    match &device.device_type {
        Some(device_type) => match device_type {
            ConnectedDeviceType::Bridge6 | ConnectedDeviceType::Bridge4 => {
                enter_bridge_bootloader(device)
            }
            ConnectedDeviceType::Click | ConnectedDeviceType::ULoop => enter_rpi_bootloader(device),
            ConnectedDeviceType::BridgeBootloader | ConnectedDeviceType::RPBootloader => Ok(()), // already in bootloader mode
        },
        None => Err(CommandError::Device(
            "Unable to enter bootloader mode for unsupported device!".to_string(),
        )),
    }
}
