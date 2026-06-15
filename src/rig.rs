//! Main FT-991 driver implementation

use crate::error::{Error, Result};
use crate::types::*;
use log::{debug, warn};
use serialport::{SerialPort, SerialPortBuilder};
use std::io::{Read, Write};
use std::time::Duration;

/// Serial port configuration
#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub port: String,
    pub baud_rate: u32,
}

impl Default for SerialConfig {
    fn default() -> Self {
        SerialConfig {
            port: "/dev/ttyUSB0".to_string(),
            baud_rate: 38400,
        }
    }
}

/// FT-991 Transceiver driver
pub struct FT991 {
    port: Box<dyn SerialPort>,
    timeout: Duration,
}

impl FT991 {
    /// Create a new FT-991 instance and connect to the rig
    pub fn new(config: SerialConfig) -> Result<Self> {
        let port = SerialPortBuilder::new(&config.port, config.baud_rate)
            .timeout(Duration::from_millis(2000))
            .open()
            .map_err(|e| Error::SerialError(e.to_string()))?;

        debug!("Connected to {} at {} baud", config.port, config.baud_rate);

        Ok(FT991 {
            port: Box::new(port),
            timeout: Duration::from_millis(2000),
        })
    }

    /// Set the frequency on VFO A
    pub async fn set_frequency(&mut self, freq: u32) -> Result<()> {
        let cmd = crate::commands::SetFrequencyCmd::new(freq)?;
        let bytes = cmd.build_bytes();
        self.send_command(&bytes[..11]).await
    }

    /// Get the current frequency from VFO A
    pub async fn get_frequency(&mut self) -> Result<u32> {
        let response = self.send_query(b"FA;").await?;
        self.parse_frequency_response(&response)
    }

    /// Set the operating mode
    pub async fn set_mode(&mut self, mode: Mode) -> Result<()> {
        let cmd = crate::commands::SetModeCmd::new(VFO::A, mode);
        let bytes = cmd.build_bytes();
        self.send_command(&bytes).await
    }

    /// Get the current operating mode
    pub async fn get_mode(&mut self) -> Result<Mode> {
        let response = self.send_query(b"MD0;").await?;
        self.parse_mode_response(&response)
    }

    /// Enable or disable transmit (PTT)
    pub async fn set_ptt(&mut self, transmit: bool) -> Result<()> {
        let cmd = crate::commands::SetPTTCmd::new(transmit);
        let bytes = cmd.build_bytes();
        self.send_command(&bytes[..4]).await
    }

    /// Get PTT status
    pub async fn get_ptt(&mut self) -> Result<bool> {
        let response = self.send_query(b"TX;").await?;
        Ok(response.contains('1'))
    }

    /// Get power supply voltage and other meter readings
    pub async fn read_meter(&mut self, meter_type: u8) -> Result<u8> {
        let cmd = format!("RM{};", meter_type).into_bytes();
        let response = self.send_query(&cmd).await?;
        self.parse_meter_response(&response)
    }

    /// Get S-meter reading
    pub async fn get_s_meter(&mut self) -> Result<u8> {
        self.read_meter(0).await
    }

    /// Get SWR meter reading
    pub async fn get_swr_meter(&mut self) -> Result<u8> {
        self.read_meter(3).await
    }

    /// Get ALC (Automatic Level Control) meter reading
    pub async fn get_alc_meter(&mut self) -> Result<u8> {
        self.read_meter(4).await
    }

    /// Get power output meter reading
    pub async fn get_power_meter(&mut self) -> Result<u8> {
        self.read_meter(5).await
    }

    /// Enable or disable split mode
    pub async fn set_split(&mut self, enabled: bool) -> Result<()> {
        let cmd = if enabled { b"FT1;" } else { b"FT0;" };
        self.send_command(cmd).await
    }

    /// Get split mode status
    pub async fn get_split(&mut self) -> Result<Split> {
        let response = self.send_query(b"FT;").await?;
        if response.contains('1') {
            Ok(Split::On)
        } else {
            Ok(Split::Off)
        }
    }

    /// Get rig information (IF command)
    pub async fn get_info(&mut self) -> Result<RigStatus> {
        let response = self.send_query(b"IF;").await?;
        self.parse_info_response(&response)
    }

    // Private helper methods

    async fn send_command(&mut self, cmd: &[u8]) -> Result<()> {
        debug!("Sending command: {:?}", String::from_utf8_lossy(cmd));
        self.port
            .write_all(cmd)
            .map_err(|e| Error::SerialError(e.to_string()))?;
        Ok(())
    }

    async fn send_query(&mut self, cmd: &[u8]) -> Result<String> {
        self.send_command(cmd).await?;

        // Read response
        let mut buffer = [0u8; 256];
        let n = self
            .port
            .read(&mut buffer)
            .map_err(|e| Error::SerialError(e.to_string()))?;

        if n == 0 {
            return Err(Error::Timeout);
        }

        String::from_utf8(buffer[..n].to_vec()).map_err(|_| Error::InvalidResponse)
    }

    fn parse_frequency_response(&self, response: &str) -> Result<u32> {
        // Response format: FA000014200000;
        if !response.starts_with("FA") {
            return Err(Error::InvalidResponse);
        }

        let freq_str = response[2..11].trim_end_matches(';');
        freq_str
            .parse::<u32>()
            .map_err(|_| Error::InvalidResponse)
    }

    fn parse_mode_response(&self, response: &str) -> Result<Mode> {
        // Response format: MD0{mode};
        if !response.starts_with("MD") {
            return Err(Error::InvalidResponse);
        }

        if response.len() < 4 {
            return Err(Error::InvalidResponse);
        }

        let mode_char = response.chars().nth(3).ok_or(Error::InvalidResponse)?;
        Mode::from_cat_char(mode_char)
            .ok_or(Error::InvalidMode(mode_char.to_string()))
    }

    fn parse_meter_response(&self, response: &str) -> Result<u8> {
        // Response format: RM{type}{value};
        if !response.starts_with("RM") || response.len() < 6 {
            return Err(Error::InvalidResponse);
        }

        let value_str = response[3..6].trim_end_matches(';');
        value_str
            .parse::<u8>()
            .map_err(|_| Error::InvalidResponse)
    }

    fn parse_info_response(&self, response: &str) -> Result<RigStatus> {
        // IF response format is complex, simplified here
        // Real implementation would parse the full IF response
        debug!("Parsing IF response: {}", response);
        Err(Error::Unsupported) // TODO: Implement full IF parsing
    }
}
