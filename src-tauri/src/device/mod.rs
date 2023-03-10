use std::{thread::sleep, time::Duration};

use self::bootloader::{enter_bridge_bootloader, enter_rpi_bootloader};
use crate::{
    commands::github::fetch_releases,
    error::{Error, Result},
    github::Release,
    USB_DEFAULT_BAUD_RATE, USB_TIMEOUT,
};

use log::{debug, info, trace};
use pirate_midi_rs::{check::CheckResponse, Command, PirateMIDIDevice, Response};
use serde::{Deserialize, Serialize};
use serialport::{SerialPortBuilder, SerialPortType};
use ts_rs::TS;
use usb_enumeration::UsbDevice;

mod bootloader;

// list of the supported devices
#[derive(Deserialize, Serialize, TS, Debug, Clone, PartialEq)]
#[ts(export)]
pub enum ConnectedDeviceType {
    Bridge4,
    Bridge6,
    BridgeBootloader,
    Click,
    ULoop,
    RPBootloader,
    Unknown,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DeviceDetails {
    pub uid: String,
    pub device_model: String,
    pub firmware_version: String,
    pub hardware_version: String,
    pub device_name: String,
    pub profile_id: String,
}

impl DeviceDetails {
    pub fn get_hardware_revision(&self) -> Option<u32> {
        if self.hardware_version.is_empty() {
            None
        } else {
            self.hardware_version.chars().last().unwrap().to_digit(10)
        }
    }
}

impl From<CheckResponse> for DeviceDetails {
    fn from(value: CheckResponse) -> Self {
        DeviceDetails {
            uid: value.uid,
            device_model: value.device_model,
            firmware_version: value.firmware_version,
            hardware_version: value.hardware_version,
            device_name: value.device_name,
            profile_id: value.profile_id,
        }
    }
}

#[derive(Deserialize, Serialize, TS, Debug, Clone)]
#[ts(export)]
pub struct ConnectedDevice {
    /// Platform specific unique ID
    pub id: String,
    /// Available Releases
    pub releases: Option<Vec<Release>>,
    /// Vendor ID
    pub vendor_id: u16,
    /// Product ID
    pub product_id: u16,
    /// Device Type
    pub device_type: ConnectedDeviceType,
    /// Optional device description
    pub description: Option<String>,
    /// Optional serial number
    pub serial_number: Option<String>,
    /// Device Details (Currently only Bridge Devices)
    pub device_details: Option<DeviceDetails>,
}

impl ConnectedDevice {
    // { id: "16926237606252", vendor_id: 11914, product_id: 61450, description: Some("RP2040"), serial_number: Some("E661343213701439") }
    // { id: "17037353476373", vendor_id: 11914, product_id: 3, description: Some("RP2 Boot"), serial_number: Some("E0C912952D54") }
    // { id: "16928040556979", vendor_id: 1155, product_id: 22336, description: Some("Bridge 6"), serial_number: Some("208133813536") }

    // FYI, this is a hack for discoverability until other devices support device API
    fn determine_device_type(device: &UsbDevice) -> ConnectedDeviceType {
        match &device.description {
            Some(value) => match value.as_str() {
                "Bridge 6" | "Bridge6" => ConnectedDeviceType::Bridge6,
                "Bridge 4" | "Bridge4" => ConnectedDeviceType::Bridge4,
                "CLiCK" => ConnectedDeviceType::Click,
                "uLoop" => ConnectedDeviceType::ULoop,
                "RP2 Boot" => ConnectedDeviceType::RPBootloader,
                "DFU in FS Mode" => ConnectedDeviceType::BridgeBootloader,
                _ => ConnectedDeviceType::Unknown,
            },
            None => ConnectedDeviceType::Unknown,
        }
    }

    pub fn get_serial_port(&self, baud_rate: u32) -> Result<SerialPortBuilder> {
        for port in serialport::available_ports().map_err(|e| Error::Serial(e.to_string()))? {
            debug!("reviewing port: {:?}", port);
            if let SerialPortType::UsbPort(usb_info) = &port.port_type {
                if usb_info.serial_number == self.serial_number {
                    info!("found device via serial number");
                    return Ok(serialport::new(port.port_name, baud_rate).timeout(USB_TIMEOUT));
                }
                if usb_info.vid == self.vendor_id && usb_info.pid == self.product_id {
                    info!("found device via vid/pid");
                    return Ok(serialport::new(port.port_name, baud_rate).timeout(USB_TIMEOUT));
                }
            }
        }
        Err(Error::Serial("unable to locate device".to_string()))
    }

    pub fn try_get_device_details(&mut self, delay: Option<Duration>) -> Result<()> {
        // ports might not be immidately available, so delay will delay this operation for the duration
        if let Some(duration) = delay {
            sleep(duration);
        }

        // find our serial port
        match self.get_serial_port(USB_DEFAULT_BAUD_RATE) {
            Ok(builder) => {
                trace!("serialport builder: {:?}", builder);
                match PirateMIDIDevice::new()
                    .with_serialport_builder(builder)
                    .send(Command::Check)
                {
                    Ok(res) => match res {
                        Response::Check(details) => {
                            trace!("rx: {:?}", details);
                            self.device_details = Some(DeviceDetails::from(details));
                            Ok(())
                        }
                        _ => err!(Error::Serial(
                            "invalid response type from device!".to_string()
                        )),
                    },
                    Err(err) => err!(Error::Serial(err.to_string())),
                }
            }
            Err(err) => err!(Error::Serial(err.to_string())),
        }
    }

    pub async fn try_get_github_releases(&mut self) -> Result<()> {
        let releases = fetch_releases(self.clone()).await?;
        debug!("releases: {:?}", releases);
        self.releases = Some(releases);
        Ok(())
    }

    pub async fn try_get_all_device_info(&mut self) -> Result<()> {
        // get device details, then retrieve the github releases - the order of this is important!
        match self.try_get_device_details(Some(USB_TIMEOUT)) {
            Ok(_) => (),
            Err(err) => info!("unable to get device details: {:?}", err),
        }
        self.try_get_github_releases().await
    }

    pub fn enter_bootloader(&self) -> Result<()> {
        match &self.device_type {
            ConnectedDeviceType::Bridge6 | ConnectedDeviceType::Bridge4 => {
                enter_bridge_bootloader(self)
            }
            ConnectedDeviceType::Click | ConnectedDeviceType::ULoop => enter_rpi_bootloader(self),
            ConnectedDeviceType::BridgeBootloader | ConnectedDeviceType::RPBootloader => Ok(()), // already in bootloader mode
            ConnectedDeviceType::Unknown => {
                err!(Error::Bootloader("unsupported device".to_string()))
            }
        }
    }
}

impl From<&UsbDevice> for ConnectedDevice {
    fn from(value: &UsbDevice) -> Self {
        ConnectedDevice {
            id: value.id.clone(),
            releases: None,
            vendor_id: value.vendor_id,
            product_id: value.product_id,
            description: value.description.clone(),
            serial_number: value.serial_number.clone(),
            device_type: ConnectedDevice::determine_device_type(value),
            device_details: None,
        }
    }
}
