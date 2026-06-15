//! CAT Protocol implementation for Yaesu FT-991
//!
//! The CAT (Computer-Aided Transceiver) protocol uses 5-byte commands sent via serial.

use crate::error::{Error, Result};
use log::{debug, trace};

/// Maximum CAT command length in bytes
pub const CAT_CMD_LEN: usize = 5;

/// CAT command frame structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CATCommand {
    pub bytes: [u8; CAT_CMD_LEN],
}

impl CATCommand {
    /// Create a new CAT command
    pub fn new(cmd: [u8; 5]) -> Self {
        CATCommand { bytes: cmd }
    }

    /// Validate CAT command (last byte must be semicolon)
    pub fn is_valid(&self) -> bool {
        self.bytes[4] == b';'
    }
}

/// Parse a response from the rig
pub fn parse_response(data: &[u8]) -> Result<String> {
    if data.is_empty() {
        return Err(Error::InvalidResponse);
    }

    // Response should end with semicolon
    if data[data.len() - 1] != b';' {
        return Err(Error::InvalidResponse);
    }

    String::from_utf8(data.to_vec()).map_err(|_| Error::InvalidResponse)
}

/// CAT Command builders
pub mod cmd {
    use super::*;

    /// Set Frequency command (FA)
    /// Format: "FA" + 9 digit frequency + ";"
    pub fn set_frequency_a(freq: u32) -> Result<[u8; 5]> {
        // FA command with frequency encoded in next transmission
        let mut cmd = [0u8; 5];
        cmd[0] = b'F';
        cmd[1] = b'A';
        cmd[2] = 0;
        cmd[3] = 0;
        cmd[4] = b';';
        Ok(cmd)
    }

    /// Get Frequency command (FA)
    pub fn get_frequency_a() -> [u8; 5] {
        [b'F', b'A', b';', 0, 0]
    }

    /// Set Mode command (MD)
    /// Format: "MD" + 0 (VFO A) + mode_char + ";"
    pub fn set_mode(vfo: u8, mode_char: char) -> [u8; 5] {
        [b'M', b'D', vfo, mode_char as u8, b';']
    }

    /// Get Mode command (MD)
    pub fn get_mode(vfo: u8) -> [u8; 5] {
        [b'M', b'D', vfo, b';', 0]
    }

    /// Set PTT (Transmit) command (TX)
    /// Format: "TX" + 0 (receive) or 1 (transmit) + ";"
    pub fn set_ptt(tx: bool) -> [u8; 5] {
        [b'T', b'X', if tx { b'1' } else { b'0' }, b';', 0]
    }

    /// Get PTT Status command (TX)
    pub fn get_ptt() -> [u8; 5] {
        [b'T', b'X', b';', 0, 0]
    }

    /// Get Power Status command (PS)
    pub fn get_power_status() -> [u8; 5] {
        [b'P', b'S', b';', 0, 0]
    }

    /// Set Power Status command (PS)
    pub fn set_power_status(on: bool) -> [u8; 5] {
        [b'P', b'S', if on { b'1' } else { b'0' }, b';', 0]
    }

    /// Read Meter command (RM)
    /// Format: "RM" + meter_type + ";"
    pub fn read_meter(meter_type: u8) -> [u8; 5] {
        [b'R', b'M', meter_type, b';', 0]
    }

    /// Get RIT frequency command (IF)
    pub fn get_info() -> [u8; 5] {
        [b'I', b'F', b';', 0, 0]
    }

    /// Split mode command (FT)
    /// Format: "FT" + 0 (off) or 1 (on) + ";"
    pub fn set_split(on: bool) -> [u8; 5] {
        [b'F', b'T', if on { b'1' } else { b'0' }, b';', 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_command_validity() {
        let cmd = CATCommand::new([b'T', b'X', b'1', b';', 0]);
        assert!(cmd.is_valid());
    }

    #[test]
    fn test_get_frequency_command() {
        let cmd = cmd::get_frequency_a();
        assert_eq!(cmd[0], b'F');
        assert_eq!(cmd[1], b'A');
        assert_eq!(cmd[2], b';');
    }
}
