use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::device::{ConnectedDevice, ConnectedDeviceType};

#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub struct Release {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
    pub discussion_url: Option<String>,
    pub id: u64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: Option<String>,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub struct Asset {
    pub url: String,
    pub browser_download_url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
}

impl Asset {
    // https://www.dotnetperls.com/between-before-after-rust
    // finds the substring between the value of two strings
    fn between<'value>(value: &'value str, a: &str, b: &str) -> &'value str {
        // Find the two strings.
        if let Some(pos_a) = value.find(a) {
            if let Some(pos_b) = value.rfind(b) {
                // Return the part in between the 2 strings.
                let adjusted_pos_a = &pos_a + a.len();
                if adjusted_pos_a < pos_b {
                    return &value[adjusted_pos_a..pos_b];
                }
            }
        }
        return "";
    }

    fn _is_compatible(&self, device: &ConnectedDevice, device_str: &str) -> bool {
        let version = Asset::between(&self.name, format!("{device_str}_v").as_str(), ".bin");
        match &device.device_details {
            Some(details) => match version.chars().last() {
                Some(last) => last.to_digit(10) == details.get_hardware_revision(),
                None => false,
            },
            None => false,
        }
    }

    pub fn is_compatible(&self, device: &ConnectedDevice) -> bool {
        match &device.device_type {
            // assume format: bridgeX_v1.2.1.1.bin
            // the last number in the version is the compatible revision
            Some(device_type) => match device_type {
                ConnectedDeviceType::Bridge6 => self._is_compatible(&device, "bridge6"),
                ConnectedDeviceType::Bridge4 => self._is_compatible(&device, "bridge4"),
                _ => true, // assume it's true by default if we have a device type
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        device::{ConnectedDevice, ConnectedDeviceType, DeviceDetails},
        github::Asset,
    };

    #[test]
    fn _is_compatible() {
        let mut mock_devices: Vec<ConnectedDevice> = vec![];
        let mut mock_assets: Vec<Asset> = vec![];

        for i in 1..3 {
            mock_devices.push(ConnectedDevice {
                id: String::from("test"),
                vendor_id: 0,
                product_id: 0,
                description: Some(String::from("test")),
                serial_number: Some(String::from("test")),
                device_type: Some(ConnectedDeviceType::Bridge4),
                device_details: Some(DeviceDetails {
                    uid: String::from("test"),
                    device_model: String::from("test"),
                    firmware_version: String::from("test"),
                    hardware_version: format!("1.0.{i}"),
                    device_name: String::from("test"),
                    profile_id: String::from("test"),
                }),
            })
        }

        for i in 1..4 {
            mock_assets.push(Asset {
                url: String::from("test"),
                browser_download_url: String::from("test"),
                id: 0,
                node_id: String::from("test"),
                name: format!("bridge4_v1.0.1.{i}.bin"),
                label: None,
                state: String::from("test"),
                content_type: String::from("test"),
                size: 0,
                download_count: 0,
                created_at: String::from("test"),
                updated_at: String::from("test"),
            })
        }

        println!("assets: {:?}", mock_assets);
        println!("devices: {:?}", mock_devices);

        assert_eq!(
            mock_assets[0]._is_compatible(&mock_devices[0], "bridge4"),
            true
        );
        assert_eq!(
            mock_assets[0]._is_compatible(&mock_devices[1], "bridge4"),
            false
        );

        assert_eq!(
            mock_assets[1]._is_compatible(&mock_devices[0], "bridge4"),
            false
        );
        assert_eq!(
            mock_assets[1]._is_compatible(&mock_devices[1], "bridge4"),
            true
        );

        assert_eq!(
            mock_assets[2]._is_compatible(&mock_devices[0], "bridge4"),
            false
        );
        assert_eq!(
            mock_assets[2]._is_compatible(&mock_devices[1], "bridge4"),
            false
        );

        assert_eq!(
            mock_assets[0]._is_compatible(&mock_devices[0], "bridge6"),
            false
        );
        assert_eq!(
            mock_assets[0]._is_compatible(&mock_devices[1], "bridge6"),
            false
        );
        // assert_eq!(result, 4);
    }
}
