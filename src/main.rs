use anyhow::Result;

use crate::config::Config;

mod config;

fn main() -> Result<()> {
    let conf = Config::read()?;
    dbg!(conf.ip);
    Ok(())
}
