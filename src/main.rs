use ebyte_e32::{
    parameters::{air_baudrate::AirBaudRate, baudrate::BaudRate, Persistence},
    Ebyte,
};
use embedded_hal::prelude::*;
use linux_embedded_hal::Delay;
use nb::block;
use rppal::{
    gpio::Gpio,
    uart::{Parity, Uart},
};
use std::io::{self, Write};

fn main() {
    let serial = Uart::with_path("/dev/ttyAMA0", 9600, Parity::None, 8, 1).unwrap();

    let gpio = Gpio::new().unwrap();
    let aux = gpio.get(18).unwrap().into_input();
    let m0 = gpio.get(23).unwrap().into_output();
    let m1 = gpio.get(24).unwrap().into_output();

    let mut ebyte = Ebyte::new(serial, aux, m0, m1, Delay).unwrap();

    let model_data = ebyte.model_data().unwrap();
    println!("Model data: {model_data:#?}");

    let mut params = ebyte.parameters().unwrap();
    println!("Parameters unchanged: {params:#?}");

    params.air_rate = AirBaudRate::Bps300;
    params.uart_rate = BaudRate::Bps9600;
    params.channel = 23;
    ebyte
        .set_parameters(&params, Persistence::Temporary)
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
