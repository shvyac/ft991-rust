//! Type definitions for FT-991

use std::fmt;

/// Operating mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    LSB,     // Lower Sideband (SSB)
    USB,     // Upper Sideband (SSB)
    CW,      // Continuous Wave (CW)
    CWR,     // CW Reverse
    AM,      // Amplitude Modulation
    FM,      // Frequency Modulation
    RTTY,    // Radio Teletype
    RTTYR,   // RTTY Reverse
    PKTLSB,  // Packet LSB
    PKTUSB,  // Packet USB
    PKTFM,   // Packet FM
    C4FM,    // 4-Level Frequency Shift Keying
    FMN,     // Narrow FM
    AMN,     // Narrow AM
}

impl Mode {
    /// Convert mode to CAT command character
    pub fn to_cat_char(self) -> char {
        match self {
            Mode::LSB => '0',
            Mode::USB => '1',
            Mode::CW => '2',
            Mode::CWR => '3',
            Mode::AM => '4',
            Mode::FM => '5',
            Mode::RTTY => '6',
            Mode::RTTYR => '7',
            Mode::PKTLSB => '8',
            Mode::PKTUSB => '9',
            Mode::PKTFM => 'A',
            Mode::C4FM => 'B',
            Mode::FMN => 'C',
            Mode::AMN => 'D',
        }
    }

    /// Parse mode from CAT command character
    pub fn from_cat_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Mode::LSB),
            '1' => Some(Mode::USB),
            '2' => Some(Mode::CW),
            '3' => Some(Mode::CWR),
            '4' => Some(Mode::AM),
            '5' => Some(Mode::FM),
            '6' => Some(Mode::RTTY),
            '7' => Some(Mode::RTTYR),
            '8' => Some(Mode::PKTLSB),
            '9' => Some(Mode::PKTUSB),
            'A' => Some(Mode::PKTFM),
            'B' => Some(Mode::C4FM),
            'C' => Some(Mode::FMN),
            'D' => Some(Mode::AMN),
            _ => None,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::LSB => write!(f, "LSB"),
            Mode::USB => write!(f, "USB"),
            Mode::CW => write!(f, "CW"),
            Mode::CWR => write!(f, "CWR"),
            Mode::AM => write!(f, "AM"),
            Mode::FM => write!(f, "FM"),
            Mode::RTTY => write!(f, "RTTY"),
            Mode::RTTYR => write!(f, "RTTYR"),
            Mode::PKTLSB => write!(f, "PKT-LSB"),
            Mode::PKTUSB => write!(f, "PKT-USB"),
            Mode::PKTFM => write!(f, "PKT-FM"),
            Mode::C4FM => write!(f, "C4FM"),
            Mode::FMN => write!(f, "FM-N"),
            Mode::AMN => write!(f, "AM-N"),
        }
    }
}

/// VFO selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VFO {
    A,
    B,
}

impl VFO {
    pub fn to_cat_char(self) -> char {
        match self {
            VFO::A => '0',
            VFO::B => '1',
        }
    }

    pub fn from_cat_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(VFO::A),
            '1' => Some(VFO::B),
            _ => None,
        }
    }
}

/// Split mode status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Split {
    Off,
    On,
}

/// Power status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerStatus {
    Off,
    On,
}

/// Passband width (in Hz)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Passband {
    Narrow,
    Normal,
    Wide,
}

/// AGC (Automatic Gain Control) level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AGCLevel {
    Off,
    Fast,
    Medium,
    Slow,
    Auto,
}

/// Repeater shift direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RptShift {
    None,
    Plus,
    Minus,
}

/// Meter readings from the rig
#[derive(Debug, Clone, Copy)]
pub struct MeterReading {
    pub raw_value: u8,
    pub interpreted_value: f32,
}

/// Complete rig status
#[derive(Debug, Clone)]
pub struct RigStatus {
    pub frequency: u32,
    pub mode: Mode,
    pub vfo: VFO,
    pub split: Split,
    pub power: PowerStatus,
    pub s_meter: u8,
    pub swr: u8,
    pub alc: u8,
    pub power_meter: u8,
}
