use log::debug;
use pirate_midi_rs::{Command, ControlArgs, PirateMIDIDevice};
use serialport::{available_ports, SerialPortBuilder, SerialPortType};

use crate::{
    commands::CommandError,
    device::{ConnectedDevice, ConnectedDeviceType},
    USB_DEFAULT_BAUD_RATE, USB_RPI_BOOTLOADER_BAUD_RATE,
};

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
    match build_serialport_builder(device, USB_DEFAULT_BAUD_RATE) {
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
    match build_serialport_builder(device, USB_RPI_BOOTLOADER_BAUD_RATE) {
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
