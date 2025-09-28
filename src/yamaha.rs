use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // can we maybe change this to snake case?
pub struct StatusResponse {
    #[serde(rename = "response_code")]
    pub response_code: i32,
    pub power: String,
    pub sleep: i32,
    pub volume: i32,
    pub mute: bool,
    #[serde(rename = "max_volume")]
    pub max_volume: i32,
    pub input: String,
    #[serde(rename = "input_text")]
    pub input_text: String,
    #[serde(rename = "distribution_enable")]
    pub distribution_enable: bool,
    #[serde(rename = "link_control")]
    pub link_control: String,
    #[serde(rename = "link_audio_delay")]
    pub link_audio_delay: String,
    #[serde(rename = "disable_flags")]
    pub disable_flags: i64,
}

pub async fn status(client: &Client, conf: &Config) -> Result<StatusResponse> {
    Ok(client
        .get(format!(
            "http://{}/YamahaExtendedControl/v1/main/getStatus",
            conf.ip
        ))
        .send()
        .await
        .context("sending request")?
        .json()
        .await
        .context("decoding json")?)
}

pub fn volume_to_db(volume: i32) -> f32 {
    -96.5 + 0.5 * (volume as f32) - 0.5
}
