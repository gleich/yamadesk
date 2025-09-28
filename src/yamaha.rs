use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusResponse {
    pub response_code: i32,
    pub power: String,
    pub sleep: i32,
    pub volume: i32,
    pub mute: bool,
    pub max_volume: i32,
    pub input: String,
    pub input_text: String,
    pub distribution_enable: bool,
    pub link_control: String,
    pub link_audio_delay: String,
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
