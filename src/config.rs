use anyhow::{Context, Result};
use linux_embedded_hal::serial_core;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

/// Load a configuration from the given file path,
/// returning an error if something goes wrong.
///
/// # Errors
/// Opening the file and parsing it may fail, returning error.
pub fn load(config_path: impl AsRef<Path>) -> Result<Config> {
    let path = read_to_string(&config_path).with_context(|| {
        format!(
            "Failed to open config file {}",
            config_path.as_ref().display()
        )
    })?;
    toml::from_str(&path).map_err(|e| {
        eprintln!(
            "Failed to parse configuration file. Here's an example:\n{}",
            toml::to_string(&Config::example()).unwrap()
        );
        anyhow::anyhow!(e)
    })
}
/// Configuration for connecting to the Ebyte module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub(crate) serial_path: PathBuf,
    pub(crate) baudrate: u32,
    pub(crate) parity: Parity,
    pub(crate) data_bits: u8,
    pub(crate) stop_bits: u8,
    pub(crate) gpiochip_path: PathBuf,
    pub(crate) aux_pin: u32,
    pub(crate) m0_pin: u32,
    pub(crate) m1_pin: u32,
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
            gpiochip_path: "/dev/gpiochip0".into(),
            aux_pin: 18,
            m0_pin: 23,
            m1_pin: 24,
        }
    }
}

/// Same as [`linux_embedded_hal::serial_core::Parity`],
/// copied only for serde purposes.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
pub(crate) enum Parity {
    /// No parity bit.
    None,
    /// Even parity.
    Even,
    /// Odd parity.
    Odd,
}

impl From<Parity> for serial_core::Parity {
    fn from(value: Parity) -> Self {
        match value {
            Parity::None => Self::ParityNone,
            Parity::Even => Self::ParityEven,
            Parity::Odd => Self::ParityOdd,
        }
    }
}

/// Same as [`linux_embedded_hal::serial_core::StopBits`],
/// copied only for serde purposes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum StopBits {
    /// One stop bit.
    Stop1,

    /// Two stop bits.
    Stop2,
}

impl TryFrom<u8> for StopBits {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Stop1),
            2 => Ok(Self::Stop2),
            n => anyhow::bail!(
                "'{n}' is not a valid value for stop bits. Valid values are '1' and '2'."
            ),
        }
    }
}

impl From<StopBits> for serial_core::StopBits {
    fn from(value: StopBits) -> Self {
        match value {
            StopBits::Stop1 => Self::Stop1,
            StopBits::Stop2 => Self::Stop2,
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
            gpiochip_path: "/dev/gpiochip0".into(),
            aux_pin: 0,
            m0_pin: 1,
            m1_pin: 2,
        };
        println!("{}", toml::to_string(&config).unwrap());
    }
}
