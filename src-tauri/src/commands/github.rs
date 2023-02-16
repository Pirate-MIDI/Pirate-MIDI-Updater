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

use crate::usb::device::ConnectedDeviceType;
use crate::{BRIDGE_GITHUB_REPO, CLICK_GITHUB_REPO, GITHUB_API_URL, GITHUB_ORG};

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
/// retrieve all available github releases
pub async fn fetch_releases(
    device_type: ConnectedDeviceType,
) -> Result<Vec<Release>, CommandError> {
    // perform the fetch
    info!("fetching releases from github...");

    // determine which repo to get
    let repo = match device_type {
        ConnectedDeviceType::Bridge6 | ConnectedDeviceType::Bridge4 => BRIDGE_GITHUB_REPO,
        ConnectedDeviceType::Click => CLICK_GITHUB_REPO,
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

#[tauri::command]
/// retrieve specific binary asset and save to the filesystem
pub async fn fetch_asset(asset: Asset) -> Result<PathBuf, CommandError> {
    // download the binary
    info!("fetching asset from github: {}", asset.browser_download_url);
    let request = reqwest::Client::new()
        .get(asset.browser_download_url)
        .headers(build_headers())
        .send();

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
                                info!("successfully downloaded - total bytes written: {}", written);
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
