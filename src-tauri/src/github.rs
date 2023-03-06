use log::debug;
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
    fn after<'value>(value: &'value str, a: &str) -> &'value str {
        // Find the string and return the part after.
        if let Some(pos_a) = value.rfind(a) {
            let adjusted_pos_a = pos_a + a.len();
            if adjusted_pos_a < value.len() {
                return &value[adjusted_pos_a..];
            }
        }
        return "";
    }

    fn _is_compatible(&self, device: &ConnectedDevice, device_str: &str) -> bool {
        // get the version string - should be 7 characters long
        let version = Asset::after(&self.name, format!("{device_str}_v").as_str())
            .chars()
            .take(7)
            .collect::<String>();

        // collect our device details and determine if the last character of the version matches our hardware revision
        match &device.device_details {
            Some(details) => match version.chars().last() {
                Some(last) => {
                    let result = last.to_digit(10) == details.get_hardware_revision();
                    debug!(
                        "{} is compatible: {result} - version: {version}",
                        &self.name
                    );
                    result
                }
                None => false,
            },
            None => false,
        }
    }

    fn _is_not_diag(&self, device_str: &str) -> bool {
        self.name.starts_with(format!("{device_str}_v").as_str())
    }

    pub fn is_compatible(&self, device: &ConnectedDevice) -> bool {
        match &device.device_type {
            // assume format: bridgeX_v1.2.1.1.bin || device_v1.0.0.0.uf2
            // the last number in the version is the compatible revision
            ConnectedDeviceType::Bridge6 => self._is_compatible(&device, "bridge6"),
            ConnectedDeviceType::Bridge4 => self._is_compatible(&device, "bridge4"),
            ConnectedDeviceType::Click => self._is_not_diag("click"),
            ConnectedDeviceType::Unknown => false,
            _ => true, // assume it's true by default if we have a device type
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{device::ConnectedDevice, github::Asset};

    #[test]
    fn _is_compatible() {
        let mut mock_devices: Vec<ConnectedDevice> = vec![];
        let mut mock_assets: Vec<Asset> = vec![];

        for i in 1..3 {
            mock_devices.push(ConnectedDevice {
                id: String::from("test"),
                vendor_id: 0,
                product_id: 0,
                description: Some(String::from("Bridge 4")),
                serial_number: Some(String::from("test")),
                device_type: crate::device::ConnectedDeviceType::Bridge4,
                device_details: None,
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
