//! High-level command builders for FT-991

use crate::error::{Error, Result};
use crate::types::*;

/// Extended command for setting frequency
/// The FT-991 uses a special protocol for setting frequency
pub struct SetFrequencyCmd {
    pub frequency: u32,
}

impl SetFrequencyCmd {
    pub fn new(freq: u32) -> Result<Self> {
        // FT-991 frequency range: 30 kHz - 56 MHz, 118-164 MHz, 420-470 MHz
        if !Self::is_valid_frequency(freq) {
            return Err(Error::InvalidFrequency(freq));
        }
        Ok(SetFrequencyCmd { frequency: freq })
    }

    fn is_valid_frequency(freq: u32) -> bool {
        matches!(
            freq,
            30_000..=56_000_000 | 118_000_000..=164_000_000 | 420_000_000..=470_000_000
        )
    }

    /// Build the CAT command bytes for frequency setting
    pub fn build_bytes(&self) -> [u8; 14] {
        let mut cmd = [0u8; 14];
        cmd[0] = b'F';
        cmd[1] = b'A';

        // Format frequency as 9-digit string (right-aligned)
        let freq_str = format!("{:09}", self.frequency);
        for (i, c) in freq_str.chars().enumerate() {
            cmd[i + 2] = c as u8;
        }
        cmd[11] = b';';
        cmd
    }
}

/// Command to set operating mode
pub struct SetModeCmd {
    pub vfo: VFO,
    pub mode: Mode,
}

impl SetModeCmd {
    pub fn new(vfo: VFO, mode: Mode) -> Self {
        SetModeCmd { vfo, mode }
    }

    pub fn build_bytes(&self) -> [u8; 5] {
        [
            b'M',
            b'D',
            self.vfo.to_cat_char() as u8,
            self.mode.to_cat_char() as u8,
            b';',
        ]
    }
}

/// Command to control PTT (Push-To-Talk)
pub struct SetPTTCmd {
    pub transmit: bool,
}

impl SetPTTCmd {
    pub fn new(transmit: bool) -> Self {
        SetPTTCmd { transmit }
    }

    pub fn build_bytes(&self) -> [u8; 5] {
        [b'T', b'X', if self.transmit { b'1' } else { b'0' }, b';', 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_frequency_cmd() {
        let cmd = SetFrequencyCmd::new(14_200_000).unwrap();
        let bytes = cmd.build_bytes();
        assert_eq!(bytes[0], b'F');
        assert_eq!(bytes[1], b'A');
        // Verify frequency string
        let freq_str = std::str::from_utf8(&bytes[2..11]).unwrap();
        assert_eq!(freq_str, "014200000");
    }

    #[test]
    fn test_invalid_frequency() {
        // Outside any valid band
        let result = SetFrequencyCmd::new(10_000);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_mode_cmd() {
        let cmd = SetModeCmd::new(VFO::A, Mode::USB);
        let bytes = cmd.build_bytes();
        assert_eq!(bytes, [b'M', b'D', b'0', b'1', b';']);
    }
}
