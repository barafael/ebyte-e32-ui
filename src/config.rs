use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    serial_path: PathBuf,
    baudrate: u32,
    parity: Parity,
    data_bits: u8,
    stop_bits: u8,
    aux_pin: u32,
    m0_pin: u32,
    m1_pin: u32,
}

/// Same as `[rppal::uart::Parity]`,
/// copied only for serde purposes.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
enum Parity {
    /// No parity bit.
    None,
    /// Even parity.
    Even,
    /// Odd parity.
    Odd,
    /// Sets parity bit to `1`.
    Mark,
    /// Sets parity bit to `0`.
    Space,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_example_config() {
        let config = std::fs::read_to_string("Config.toml").unwrap();
        assert!(toml::from_str::<Config>(&config).is_ok());
    }

    #[test]
    fn show_config() {
        let config = Config {
            serial_path: "/dev/ttyAMA0".into(),
            parity: Parity::None,
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            aux_pin: 0,
            m0_pin: 1,
            m1_pin: 2,
        };
        println!("{}", toml::to_string(&config).unwrap());
    }
}
