# FT-991 Rust Driver

A modern, type-safe Rust library for controlling the Yaesu FT-991 transceiver via the CAT (Computer-Aided Transceiver) protocol.

## Features

- ✅ Pure Rust with no unsafe code (except in serial port layer)
- ✅ Type-safe enumerations for modes, VFO, and other settings
- ✅ Async/await support with Tokio
- ✅ Comprehensive error handling
- ✅ Serial port abstraction
- ✅ Full FT-991 CAT protocol support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ft991-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use ft991_rs::{FT991, SerialConfig, Mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure serial connection
    let config = SerialConfig {
        port: "/dev/ttyUSB0".to_string(),
        baud_rate: 38400,
    };

    // Connect to the rig
    let mut rig = FT991::new(config)?;

    // Get current frequency
    let freq = rig.get_frequency().await?;
    println!("Current frequency: {} Hz", freq);

    // Set a new frequency
    rig.set_frequency(14_200_000).await?;

    // Change mode to USB
    rig.set_mode(Mode::USB).await?;

    // Enable transmit
    rig.set_ptt(true).await?;

    // Read S-meter
    let s_meter = rig.get_s_meter().await?;
    println!("S-meter reading: {}", s_meter);

    Ok(())
}
```

## Supported Operations

### Frequency Control
- `set_frequency(freq)` - Set VFO A frequency
- `get_frequency()` - Get VFO A frequency

### Mode Control
- `set_mode(mode)` - Set operating mode (LSB, USB, CW, AM, FM, etc.)
- `get_mode()` - Get current operating mode

### Transmit Control
- `set_ptt(bool)` - Enable/disable transmit
- `get_ptt()` - Get PTT status

### Metering
- `get_s_meter()` - Signal strength meter
- `get_swr_meter()` - Standing Wave Ratio
- `get_alc_meter()` - Automatic Level Control
- `get_power_meter()` - Output power meter

### Split Mode
- `set_split(bool)` - Enable/disable split mode
- `get_split()` - Get split mode status

### Rig Information
- `get_info()` - Get complete rig status

## Serial Port Configuration

Default settings for FT-991:
- **Baud Rate**: 38400 bps
- **Data Bits**: 8
- **Stop Bits**: 2
- **Parity**: None
- **Handshake**: Hardware (RTS/CTS)

## Supported Modes

- LSB - Lower Sideband
- USB - Upper Sideband
- CW - Continuous Wave
- CWR - CW Reverse
- AM - Amplitude Modulation
- FM - Frequency Modulation
- RTTY - Radio Teletype
- RTTYR - RTTY Reverse
- PKTLSB - Packet LSB
- PKTUSB - Packet USB
- PKTFM - Packet FM
- C4FM - 4-Level Frequency Shift Keying
- FMN - Narrow FM
- AMN - Narrow AM

## Frequency Ranges

The FT-991 supports:
- **HF Bands**: 30 kHz - 56 MHz
- **VHF Band**: 118 - 164 MHz
- **UHF Band**: 420 - 470 MHz

## License

LGPL-2.1-or-later (compatible with Hamlib)

## References

- [Yaesu FT-991 Manual](https://www.manualslib.com/products/Yaesu-Ft-991-7622693.html)
- [Hamlib FT-991 Implementation](https://github.com/Hamlib/Hamlib/tree/master/rigs/yaesu)
- [CAT Protocol Documentation](https://www.yaesu.com/)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Roadmap

- [ ] Complete IF response parsing
- [ ] Memory channel management
- [ ] Extended functions (NB, NR, AGC, etc.)
- [ ] Roofing filter control
- [ ] Full split mode support
- [ ] RIT/XIT control
- [ ] CTCSS/DCS encoding
- [ ] Examples for common use cases
- [ ] Integration tests with virtual rig
