use log::debug;
use pirate_midi_rs::{Command, ControlArgs, PirateMIDIDevice};
use serialport::{SerialPortBuilder, SerialPortType};

#[cfg(target_family = "unix")]
use serialport::available_ports;

#[cfg(target_family = "windows")]
use windows::available_ports;

use crate::error::{Error, Result};
use crate::{device::ConnectedDevice, USB_DEFAULT_BAUD_RATE, USB_RPI_BOOTLOADER_BAUD_RATE};

fn build_serialport_builder(device: &ConnectedDevice, baud_rate: u32) -> Result<SerialPortBuilder> {
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
            err!(Error::Serial("unable to locate device".to_string()))
        }
        Err(err) => err!(Error::Serial(err.to_string())),
    }
}

pub fn enter_bridge_bootloader(device: &ConnectedDevice) -> Result<()> {
    match build_serialport_builder(device, USB_DEFAULT_BAUD_RATE) {
        Ok(builder) => match PirateMIDIDevice::new()
            .with_serialport_builder(builder)
            .send(Command::Control(ControlArgs::EnterBootloader))
        {
            Ok(_) => Ok(()),
            Err(err) => err!(Error::Bootloader(format!(
                "Unable to enter bootloader due to error: {}",
                err
            ))),
        },
        Err(err) => Err(err),
    }
}

// the RP2040 will immidately enter bootloader mode if you connect to it with
// a baud rate of 1200, so we're just going to quickly connect and bail
pub fn enter_rpi_bootloader(device: &ConnectedDevice) -> Result<()> {
    match build_serialport_builder(device, USB_RPI_BOOTLOADER_BAUD_RATE) {
        Ok(builder) => match builder.open() {
            Ok(_) => Ok(()),
            Err(err) => match err.kind() {
                serialport::ErrorKind::Io(sub_kind) => match sub_kind {
                    std::io::ErrorKind::Other => Ok(()), // ignore this because on windows this can get thrown
                    _ => err!(Error::Serial(format!(
                        "Unable to open RP serial port: {}",
                        err
                    ))),
                },
                _ => err!(Error::Serial(format!(
                    "Unable to open RP serial port: {}",
                    err
                ))),
            },
        },
        Err(err) => Err(err),
    }
}
