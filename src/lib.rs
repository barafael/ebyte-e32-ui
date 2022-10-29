use crate::{config::Config, interface::Mode};
use ebyte_e32::{parameters::Parameters, Ebyte};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::*;
use embedded_hal::serial;
use interface::App;
use linux_embedded_hal::Delay;
use nb::block;
use rppal::{gpio::Gpio, uart::Uart};
use rustyline::{error::ReadlineError, Editor};
use std::fmt::Debug;
use std::fs::read_to_string;
use std::io::{self, Write};

pub mod config;
pub mod interface;

pub fn load_default_config() -> Config {
    let config = read_to_string("Config.toml").unwrap_or_else(|e| {
        panic!(
            "Failed to open Config.toml [{e:?}]\nHere's a default: {:#?}",
            Config::default()
        )
    });
    toml::from_str(&config).expect("Failed to parse config")
}

pub fn process(config: Config, args: App) {
    let serial = Uart::with_path(
        config.serial_path,
        config.baudrate,
        config.parity.into(),
        config.data_bits,
        config.stop_bits,
    )
    .expect("Failed to set up serial port");

    let gpio = Gpio::new().unwrap();
    let aux = gpio.get(config.aux_pin).unwrap().into_input();
    let m0 = gpio.get(config.m0_pin).unwrap().into_output();
    let m1 = gpio.get(config.m1_pin).unwrap().into_output();

    let mut ebyte = Ebyte::new(serial, aux, m0, m1, Delay).unwrap();

    let old_params = ebyte.parameters().unwrap();
    println!("Loaded parameters: {old_params:#?}");

    let new_params = Parameters::from(&args);

    if new_params == old_params {
        println!("Leaving parameters unchanged");
    } else {
        println!("Updating parameters (persistence: {:?})", args.persistence);
        ebyte.set_parameters(&new_params, args.persistence).unwrap();
        let current_params = ebyte.parameters().unwrap();
        if current_params != new_params {
            eprintln!("Error: parameters unchanged: {current_params:#?}");
        }
    }

    match args.mode {
        Mode::Send => send(ebyte),
        Mode::ReadModelData => {
            println!("Reading model data");
            let model_data = ebyte.model_data().unwrap();
            println!("{model_data:#?}");
        }
        Mode::Listen => loop {
            let b = block!(ebyte.read()).unwrap();
            print!("{}", b as char);
            io::stdout().flush().unwrap();
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
                    block!(ebyte.write(*b)).unwrap();
                    print!("{}", *b as char);
                    io::stdout().flush().unwrap();
                }
                block!(ebyte.write(b'\n')).unwrap();
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
