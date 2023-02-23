use self::bootloader::{enter_bridge_bootloader, enter_rpi_bootloader};
use crate::error::{Error, Result};

use log::error;
use pirate_midi_rs::{check::CheckResponse, Command, PirateMIDIDevice, Response};
use serde::{Deserialize, Serialize};
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
    /// Vendor ID
    pub vendor_id: u16,
    /// Product ID
    pub product_id: u16,
    /// Optional device description
    pub description: Option<String>,
    /// Optional serial number
    pub serial_number: Option<String>,
    /// Supported Device Type
    pub device_type: Option<ConnectedDeviceType>,
    /// Device Details (Currently only Bridge Devices)
    pub device_details: Option<DeviceDetails>,
}

impl ConnectedDevice {
    // { id: "16926237606252", vendor_id: 11914, product_id: 61450, description: Some("RP2040"), serial_number: Some("E661343213701439") }
    // { id: "17037353476373", vendor_id: 11914, product_id: 3, description: Some("RP2 Boot"), serial_number: Some("E0C912952D54") }
    // { id: "16928040556979", vendor_id: 1155, product_id: 22336, description: Some("Bridge 6"), serial_number: Some("208133813536") }

    // FYI, this is a hack for discoverability until other devices support device API
    fn parse_device_from_description(device: &UsbDevice) -> Option<ConnectedDeviceType> {
        match &device.description {
            Some(value) => match value.as_str() {
                "Bridge 6" => Some(ConnectedDeviceType::Bridge6),
                "Bridge 4" => Some(ConnectedDeviceType::Bridge4),
                "CLiCK" => Some(ConnectedDeviceType::Click), // TODO: verify this with a production board
                "RP2 Boot" => Some(ConnectedDeviceType::RPBootloader),
                "DFU in FS Mode" => Some(ConnectedDeviceType::BridgeBootloader),
                _ => None,
            },
            None => None,
        }
    }

    fn get_device_details(device: &UsbDevice) -> Option<DeviceDetails> {
        // connect to specific device
        let possible_device = PirateMIDIDevice::new()
            .with_vendor_id(device.vendor_id)
            .with_product_id(device.product_id);

        // attempt to get device details
        match possible_device.send(Command::Check) {
            Ok(response) => match response {
                Response::Check(details) => Some(DeviceDetails::from(details)),
                _ => {
                    error!("invalid response type from device!");
                    None
                }
            },
            Err(err) => {
                error!("unable to connect to device: {:?}", err);
                None
            }
        }
    }

    pub fn enter_bootloader(&self) -> Result<()> {
        match &self.device_type {
            Some(device_type) => match device_type {
                ConnectedDeviceType::Bridge6 | ConnectedDeviceType::Bridge4 => {
                    enter_bridge_bootloader(self)
                }
                ConnectedDeviceType::Click | ConnectedDeviceType::ULoop => {
                    enter_rpi_bootloader(self)
                }
                ConnectedDeviceType::BridgeBootloader | ConnectedDeviceType::RPBootloader => Ok(()), // already in bootloader mode
            },
            None => err!(Error::Bootloader("unsupported device".to_string())),
        }
    }
}

impl From<&UsbDevice> for ConnectedDevice {
    fn from(value: &UsbDevice) -> Self {
        ConnectedDevice {
            id: value.id.clone(),
            vendor_id: value.vendor_id.clone(),
            product_id: value.product_id.clone(),
            description: value.description.clone(),
            serial_number: value.serial_number.clone(),
            device_type: ConnectedDevice::parse_device_from_description(value),
            device_details: ConnectedDevice::get_device_details(value),
        }
    }
}
