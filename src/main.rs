use std::{thread::sleep, time::Duration};

use anyhow::Result;
use microdotphat::MicrodotPHAT;
use reqwest::Client;
use rppal::i2c::I2c;

use crate::config::Config;

mod config;
mod yamaha;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = Config::read()?;
    let client = Client::new();
    let mut i2c = I2c::new().expect("failed to open i2c bus");
    let mut display = MicrodotPHAT::new(&mut i2c).expect("failed to create microdotphat");

    loop {
        let decimal_index = 3;
        display.set_decimal(decimal_index, true);
        display
            .show(&mut i2c, true)
            .expect("failed to write to display");
        let resp = yamaha::status(&client, &conf).await?;
        display.set_decimal(decimal_index, false);
        display
            .show(&mut i2c, true)
            .expect("failed to write to display");

        let db = yamaha::volume_to_db(resp.volume).abs();
        display.write_string(&format!("{:.1}db", db), 0, 0);
        display
            .show(&mut i2c, true)
            .expect("failed to write to display");

        println!("updated display");
        sleep(Duration::from_secs(1));
    }
}
