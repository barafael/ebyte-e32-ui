//! Ebyte module control.

#![doc = include_str!("../README.md")]

use crate::config::StopBits;
use crate::{cli::Mode, config::Config};
use anyhow::Context;
use anyhow::Result;
use cli::Args;
use ebyte_e32::{parameters::Parameters, Ebyte};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::*;
use embedded_hal::serial;
use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::serial_core::BaudRate;
use linux_embedded_hal::serial_core::CharSize;
use linux_embedded_hal::serial_core::FlowControl;
use linux_embedded_hal::serial_core::PortSettings;
use linux_embedded_hal::serial_core::SerialPort;
use linux_embedded_hal::serial_unix::TTYPort;
use linux_embedded_hal::CdevPin as Pin;
use linux_embedded_hal::Delay;
use nb::block;
use rustyline::{error::ReadlineError, Editor};
use std::fmt::Debug;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;

/// Configuration from `Config.toml`.
pub mod config;

/// Command line interface.
pub mod cli;

/// Load a configuration from the given file path,
/// returning an error if something goes wrong.
///
/// # Errors
/// Opening the file and parsing it may fail, returning error.
pub fn load_config(config_path: impl AsRef<Path>) -> Result<Config> {
    let path = read_to_string(&config_path).with_context(|| {
        format!(
            "Failed to open config file {}",
            config_path.as_ref().display()
        )
    })?;
    let Ok(config) = toml::from_str(&path) else {
        eprintln!(
            "Failed to parse configuration file. Here's an example:\n{}",
            toml::to_string(&Config::example()).unwrap()
        );
        anyhow::bail!("Failed to parse config");
    };
    Ok(config)
}

/// Setup the hardware, then load some parameters,
/// update them if needed, then listen, send, or read model data.
///
/// # Panics
/// Failed initialization of the module driver
/// or communicating with the module may cause a panic.
pub fn process(args: Args) -> anyhow::Result<()> {
    let config = load_config(&args.config_file).context("Failed to get config")?;
    let baud_rate = BaudRate::from_speed(config.baudrate as usize);
    let stop_bits = StopBits::try_from(config.stop_bits)
        .context("Failed to parse stop bits")?
        .into();
    let settings: PortSettings = PortSettings {
        baud_rate,
        char_size: CharSize::Bits8,
        parity: config.parity.into(),
        stop_bits,
        flow_control: FlowControl::FlowNone,
    };

    let mut serial = TTYPort::open(&config.serial_path)
        .with_context(|| format!("Failed to open TTY {}", config.serial_path.display()))?;
    serial
        .configure(&settings)
        .context("Failed to set up serial port")?;
    let serial = linux_embedded_hal::Serial(serial);

    let mut gpiochip = Chip::new(&config.gpiochip_path)
        .with_context(|| format!("Failed to open gpiochip {}", config.gpiochip_path.display()))?;

    let aux = gpiochip
        .get_line(config.aux_pin)
        .context("Failed to get AUX line")?
        .request(LineRequestFlags::INPUT, 0, "ebyte-e32-ui")
        .context("Failed to request settings for AUX pin")?;
    let aux = Pin::new(aux).context("Failed to create AUX CDEV pin")?;

    let m0 = gpiochip
        .get_line(config.m0_pin)
        .context("Failed to get M0 line")?
        .request(LineRequestFlags::OUTPUT, 0, "ebyte-e32-ui")
        .context("Failed to request settings for M0 pin")?;
    let m0 = Pin::new(m0).context("Failed to create M0 CDEV pin")?;

    let m1 = gpiochip
        .get_line(config.m1_pin)
        .context("Failed to get M1 line")?
        .request(LineRequestFlags::OUTPUT, 0, "ebyte-e32-ui")
        .context("Failed to request settings for M1 pin")?;
    let m1 = Pin::new(m1).context("Failed to create M1 CDEV pin")?;

    let mut ebyte = Ebyte::new(serial, aux, m0, m1, Delay).expect("Failed to initialize driver");

    let old_params = ebyte
        .parameters()
        .expect("Failed to read current parameters");
    println!("Loaded parameters: {old_params:#?}");

    let new_params = Parameters::from(&args);

    if new_params == old_params {
        println!("Leaving parameters unchanged");
    } else {
        println!("Updating parameters (persistence: {:?})", args.persistence);
        ebyte
            .set_parameters(&new_params, args.persistence)
            .expect("Failed to set new parameters");
        let current_params = ebyte
            .parameters()
            .expect("Failed to read current parameters");
        if current_params != new_params {
            eprintln!("Error: parameters unchanged: {current_params:#?}");
        }
    }

    match args.mode {
        Mode::Send => {
            send(ebyte);
            Ok(())
        }
        Mode::ReadModelData => {
            println!("Reading model data");
            let model_data = ebyte.model_data().expect("Failed to read model data");
            println!("{model_data:#?}");
            Ok(())
        }
        Mode::Listen => loop {
            let b = block!(ebyte.read()).expect("Failed to read");
            print!("{}", b as char);
            io::stdout().flush().expect("Failed to flush");
        },
    }
}

fn send<S, Aux, M0, M1, D>(mut ebyte: Ebyte<S, Aux, M0, M1, D, ebyte_e32::mode::Normal>)
where
    S: serial::Read<u8> + serial::Write<u8>,
    <S as serial::Write<u8>>::Error: Debug,
    Aux: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    let mut prompt = Editor::<()>::new().expect("Failed to set up prompt");
    loop {
        match prompt.readline("Enter message >> ") {
            Ok(line) => {
                if line == "exit" || line == "quit" {
                    break;
                }
                prompt.add_history_entry(&line);

                for b in line.as_bytes() {
                    block!(ebyte.write(*b)).expect("Failed to write");
                    print!("{}", *b as char);
                    io::stdout().flush().expect("Failed to flush");
                }
                block!(ebyte.write(b'\n')).expect("Failed to write");
                println!();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
}
