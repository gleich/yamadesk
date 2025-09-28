use std::{thread::sleep, time::Duration};

use anyhow::Result;
use microdotphat::MicrodotPHAT;
use reqwest::Client;
use rppal::i2c::I2c;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

use crate::config::Config;

mod config;
mod yamaha;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();

    let conf = Config::read()?;
    info!("read config");
    let client = Client::new();
    let mut i2c = I2c::new().expect("failed to open i2c bus");
    let mut display = MicrodotPHAT::new(&mut i2c).expect("failed to create microdotphat");
    info!("connected to microdotphat display");

    loop {
        set_decimals(&mut display, true);
        display
            .show(&mut i2c, true)
            .expect("failed to write to display");
        let resp = yamaha::status(&client, &conf).await;
        set_decimals(&mut display, false);
        display
            .show(&mut i2c, true)
            .expect("failed to write to display");

        let (volume, power) = match resp {
            Ok(r) => (yamaha::volume_to_db(r.volume).abs(), r.power),
            Err(e) => {
                error!(%e, "failed to make request for receiver status");
                display.write_string("ERROR");
                display
                    .show(&mut i2c, true)
                    .expect("failed to write to display");
                sleep(Duration::from_secs(10));
                continue;
            }
        };

        let standby = power == "standby";

        if standby {
            display.write_string("SYSOFF");
            display
                .set_brightness(&mut i2c, 0.1)
                .expect("failed to set brightness");
        } else {
            display
                .set_brightness(&mut i2c, 1.0)
                .expect("failed to set brightness");
            display.write_string(&format!("{:.1}db", volume));
        }

        display
            .show(&mut i2c, true)
            .expect("failed to write to display");

        if standby {
            info!("updated display with off message");
        } else {
            info!("updated display with volume of {}", volume);
        }

        sleep(Duration::from_secs(1));
    }
}

fn set_decimals(display: &mut MicrodotPHAT, on: bool) {
    for i in 0..6 {
        display.set_decimal(i, on);
    }
}
