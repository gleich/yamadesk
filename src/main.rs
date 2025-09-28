use anyhow::Result;
use reqwest::Client;

use crate::config::Config;

mod config;
mod yamaha;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = Config::read()?;
    let client = Client::new();

    let resp = yamaha::status(&client, &conf).await?;
    dbg!(resp.volume);

    dbg!(yamaha::volume_to_db(resp.volume,));

    Ok(())
}
