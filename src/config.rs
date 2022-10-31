use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for connecting to the Ebyte module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub(crate) serial_path: PathBuf,
    pub(crate) baudrate: u32,
    pub(crate) parity: Parity,
    pub(crate) data_bits: u8,
    pub(crate) stop_bits: u8,
    pub(crate) aux_pin: u8,
    pub(crate) m0_pin: u8,
    pub(crate) m1_pin: u8,
}

impl Config {
    /// Example configuration
    #[must_use]
    pub fn example() -> Self {
        Self {
            serial_path: "dev/ttyAMA0".into(),
            baudrate: 9600,
            parity: Parity::None,
            data_bits: 8,
            stop_bits: 1,
            aux_pin: 18,
            m0_pin: 23,
            m1_pin: 24,
        }
    }
}

/// Same as `[rppal::uart::Parity]`,
/// copied only for serde purposes.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
pub(crate) enum Parity {
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

impl From<Parity> for rppal::uart::Parity {
    fn from(value: Parity) -> Self {
        match value {
            Parity::None => Self::None,
            Parity::Even => Self::Even,
            Parity::Odd => Self::Odd,
            Parity::Mark => Self::Mark,
            Parity::Space => Self::Space,
        }
    }
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
