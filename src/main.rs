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
        set_decimals(&mut display, &mut i2c, true);
        let resp = yamaha::status(&client, &conf).await;
        set_decimals(&mut display, &mut i2c, false);

        let response_data = match resp {
            Ok(r) => r,
            Err(e) => {
                error!(%e, "failed to make request for receiver status");
                for _ in 0..50 {
                    let delay = Duration::from_millis(40);
                    set_decimals(&mut display, &mut i2c, true);
                    sleep(delay);
                    set_decimals(&mut display, &mut i2c, false);
                    sleep(delay);
                }
                continue;
            }
        };

        let volume = yamaha::volume_to_db(response_data.volume).abs();
        let standby = response_data.power == "standby";

        let mut brightness = 0.1;
        if standby {
            display.write_string("SYSOFF");
        } else if response_data.sleep != 0 {
            display.write_string("SLEEP");
        } else {
            brightness = 1.0;
            display.write_string(&format!("{:.1}db", volume));
        }

        display
            .set_brightness(&mut i2c, brightness)
            .expect("failed to set brightness");
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

fn set_decimals(display: &mut MicrodotPHAT, i2c: &mut I2c, on: bool) {
    for i in 0..6 {
        display.set_decimal(i, on);
    }
    display.show(i2c, true).expect("failed to write to display");
}
