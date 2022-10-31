//! Ebyte module control.

#![doc = include_str!("../README.md")]

use crate::{cli::Mode, config::Config};
use anyhow::Context;
use anyhow::Result;
use cli::App;
use ebyte_e32::{parameters::Parameters, Ebyte};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::*;
use embedded_hal::serial;
use linux_embedded_hal::Delay;
use nb::block;
use rppal::{gpio::Gpio, uart::Uart};
use rustyline::{error::ReadlineError, Editor};
use std::fmt::Debug;
use std::fs::read_to_string;
use std::io::{self, Write};

/// Configuration from `Config.toml`.
pub mod config;

/// Command line interface.
pub mod cli;

/// Load a configuration from `Config.toml`,
/// returning an error if something goes wrong.
///
/// # Errors
/// Opening "Config.toml" and parsing it may fail, returning error.
pub fn load_config() -> Result<Config> {
    let config = read_to_string("Config.toml").context("Failed to open Config.toml")?;
    toml::from_str(&config).context("Failed to parse config")
}

/// Setup the hardware, then load some parameters,
/// update them if needed, then listen, send, or read model data.
///
/// # Panics
/// Failed initialization of the module driver
/// or communicating with the module may cause a panic.
pub fn process(config: Config, args: App) -> anyhow::Result<()> {
    let serial = Uart::with_path(
        config.serial_path,
        config.baudrate,
        config.parity.into(),
        config.data_bits,
        config.stop_bits,
    )
    .context("Failed to set up serial port")?;

    let gpio = Gpio::new().context("Failed to open Gpio")?;
    let aux = gpio
        .get(config.aux_pin)
        .context("Failed to open AUX pin")?
        .into_input();
    let m0 = gpio
        .get(config.m0_pin)
        .context("Failed to open m0 pin")?
        .into_output();
    let m1 = gpio
        .get(config.m1_pin)
        .context("Failed to open m1 pin")?
        .into_output();

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
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
