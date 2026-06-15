//! Basic FT-991 control example
//!
//! This example demonstrates:
//! - Connecting to the rig
//! - Setting frequency
//! - Changing mode
//! - Reading meters
//! - Enabling transmit

use ft991_rs::{FT991, Mode, SerialConfig};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Configure for typical USB-serial adapter
    let config = SerialConfig {
        port: "/dev/ttyUSB0".to_string(),
        baud_rate: 38400,
    };

    info!("Connecting to FT-991 on {}...", config.port);
    let mut rig = FT991::new(config)?;

    // Read initial frequency
    info!("Reading initial frequency...");
    let freq = rig.get_frequency().await?;
    info!("Current frequency: {} Hz ({} MHz)", freq, freq as f64 / 1_000_000.0);

    // Read current mode
    let mode = rig.get_mode().await?;
    info!("Current mode: {}", mode);

    // Set to 14.2 MHz (20m band)
    info!("Setting frequency to 14.2 MHz...");
    rig.set_frequency(14_200_000).await?;

    // Verify frequency was set
    let freq = rig.get_frequency().await?;
    info!("New frequency: {} Hz", freq);

    // Change to USB mode
    info!("Setting mode to USB...");
    rig.set_mode(Mode::USB).await?;

    // Verify mode
    let mode = rig.get_mode().await?;
    info!("New mode: {}", mode);

    // Read metering
    info!("Reading meters...");
    let s_meter = rig.get_s_meter().await?;
    let swr_meter = rig.get_swr_meter().await?;
    let alc_meter = rig.get_alc_meter().await?;
    let power_meter = rig.get_power_meter().await?;

    info!("S-Meter:   {}", s_meter);
    info!("SWR Meter: {}", swr_meter);
    info!("ALC Meter: {}", alc_meter);
    info!("Power:     {}", power_meter);

    // Check split mode
    let split = rig.get_split().await?;
    info!("Split mode: {:?}", split);

    info!("Example complete!");
    Ok(())
}
