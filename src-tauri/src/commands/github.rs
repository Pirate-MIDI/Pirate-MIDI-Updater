use log::{info, trace};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::env::{self, temp_dir};
use std::fs::File;
use std::io::{copy, Cursor};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use ts_rs::TS;

use crate::device::{ConnectedDevice, ConnectedDeviceType};
use crate::{GITHUB_API_URL, GITHUB_BRIDGE_REPO, GITHUB_CLICK_REPO, GITHUB_ORG};

use super::CommandError;

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

#[derive(Serialize, Deserialize)]
struct Query {
    per_page: u32,
    page: u32,
}

fn build_headers() -> HeaderMap {
    // create some headers for our fetching
    let mut headers = HeaderMap::new();

    // add the user-agent header required by github
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    // add the authorization header if the enviroment variable GITHUB_TOKEN is defined
    // this is good for developing, as the rate limit for unauthencated requests is 65 requests/hour
    match env::var("GITHUB_TOKEN") {
        Ok(token) => match HeaderValue::from_str(format!("Bearer {}", token).as_str()) {
            Ok(val) => headers.insert(AUTHORIZATION, val),
            Err(_) => todo!(),
        },
        Err(_) => todo!(),
    };

    headers
}

#[tauri::command]
/// retrieve all compatable github releases
pub async fn fetch_releases(
    device_type: ConnectedDeviceType,
) -> Result<Vec<Release>, CommandError> {
    // perform the fetch
    info!("fetching releases from github...");

    // determine which repo to get
    let repo = match device_type {
        ConnectedDeviceType::Bridge6 | ConnectedDeviceType::Bridge4 => GITHUB_BRIDGE_REPO,
        ConnectedDeviceType::Click => GITHUB_CLICK_REPO,
        ConnectedDeviceType::ULoop => todo!(),
        ConnectedDeviceType::RPBootloader | ConnectedDeviceType::BridgeBootloader => todo!(),
    };

    // retrieve the releases!
    let url = format!("{}/repos/{}/{}/releases", GITHUB_API_URL, GITHUB_ORG, repo);
    let request = reqwest::Client::new()
        .get(url)
        .headers(build_headers())
        .send();
    match request.await {
        Ok(res) => {
            trace!("success [raw]: {:?}", res);
            match res.status() {
                StatusCode::OK => match res.json::<Vec<Release>>().await {
                    Ok(releases) => Ok(releases),
                    Err(err) => Err(CommandError::Http(err.to_string())),
                },
                StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => {
                    log::error!("Rate limited from Github - headers: {:?}", res.headers());
                    Err(CommandError::Http("Github rate limit hit!".to_string()))
                }
                _ => todo!(),
            }
        }
        Err(err) => {
            trace!("error [raw]: {:?}", err);
            Err(CommandError::Http(err.to_string()))
        }
    }
}

/// retrieve specific binary asset and save to the filesystem
pub async fn fetch_compatable_asset(
    device: ConnectedDevice,
    release: Release,
) -> Result<PathBuf, CommandError> {
    match release.assets.iter().find(|a| a.is_compatible(&device)) {
        Some(asset) => {
            // download the binary
            info!("fetching asset from github: {}", asset.browser_download_url);
            let request = reqwest::Client::new()
                .get(asset.browser_download_url.clone())
                .headers(build_headers())
                .send();

            // TODO: clean up this fuckin mess
            match request.await {
                Ok(response) => match response.bytes().await {
                    Ok(payload) => {
                        // create timestamp
                        let time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        // create temp file
                        let temp_file_path = temp_dir().join(format!("{time}-{}", asset.name));
                        info!("downloading file to: {}", temp_file_path.display());
                        // create temp file
                        match File::create(&temp_file_path) {
                            Ok(mut file) => {
                                let mut content = Cursor::new(payload);
                                match copy(&mut content, &mut file) {
                                    Ok(written) => {
                                        info!(
                                            "successfully downloaded - total bytes written: {}",
                                            written
                                        );
                                        Ok(temp_file_path)
                                    }
                                    Err(err) => Err(CommandError::IO(err.to_string())),
                                }
                            }
                            Err(err) => Err(CommandError::IO(err.to_string())),
                        }
                    }
                    Err(err) => Err(CommandError::Retieval(err.to_string())),
                },
                Err(err) => Err(CommandError::Retieval(err.to_string())),
            }
        }
        None => Err(CommandError::Http(String::from(
            "unable to find compatible asset from release!",
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::github::Asset,
        device::{ConnectedDevice, ConnectedDeviceType, DeviceDetails},
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
