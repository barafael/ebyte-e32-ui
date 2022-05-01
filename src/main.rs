use crate::cli::App;
use clap::StructOpt;
use ebyte_e32::{parameters::Parameters, Ebyte};
use embedded_hal::prelude::*;
use klask::Settings;
use linux_embedded_hal::Delay;
use nb::block;
use rppal::{
    gpio::Gpio,
    uart::{Parity, Uart},
};
use std::io::{self, Write};

mod cli;

fn main() {
    let args = App::parse();
    if args.gui {
        klask::run_derived::<App, _>(Settings::default(), process);
    } else {
        process(args);
    }
}

fn process(args: App) {
    let serial = Uart::with_path("/dev/ttyAMA0", 9600, Parity::None, 8, 1).unwrap();

    let gpio = Gpio::new().unwrap();
    let aux = gpio.get(18).unwrap().into_input();
    let m0 = gpio.get(23).unwrap().into_output();
    let m1 = gpio.get(24).unwrap().into_output();

    let mut ebyte = Ebyte::new(serial, aux, m0, m1, Delay).unwrap();

    let model_data = ebyte.model_data().unwrap();
    println!("Model data: {model_data:#?}");

    let params = ebyte.parameters().unwrap();
    println!("Parameters before: {params:#?}");

    println!("Updating parameters (persistence: {:?})", args.persistence);
    ebyte
        .set_parameters(&Parameters::from(&args), args.persistence)
        .unwrap();
    let params = ebyte.parameters().unwrap();
    println!("Parameters after customization: {params:#?}");

    loop {
        match block!(ebyte.read()) {
            Err(e) => println!("ebyte error: {e:?}"),
            Ok(b) => {
                block!(ebyte.write(b)).unwrap();
                print!("{}", b as char);
                io::stdout().flush().unwrap();
            }
        }
    }
}
