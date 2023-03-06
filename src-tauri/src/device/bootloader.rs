use crate::error::{Error, Result};
use crate::{device::ConnectedDevice, USB_DEFAULT_BAUD_RATE, USB_RPI_BOOTLOADER_BAUD_RATE};
use pirate_midi_rs::{Command, ControlArgs, PirateMIDIDevice};

pub fn enter_bridge_bootloader(device: &ConnectedDevice) -> Result<()> {
    match device.get_serial_port(USB_DEFAULT_BAUD_RATE) {
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
    match device.get_serial_port(USB_RPI_BOOTLOADER_BAUD_RATE) {
        Ok(builder) => match builder.open() {
            Ok(_) => Ok(()),
            Err(err) => match err.kind() {
                serialport::ErrorKind::Io(std::io::ErrorKind::Other) => Ok(()), // ignore this specific error because on windows this can get thrown
                _ => err!(Error::Serial(format!(
                    "Unable to open RP serial port: {}",
                    err
                ))),
            },
        },
        Err(err) => Err(err),
    }
}
