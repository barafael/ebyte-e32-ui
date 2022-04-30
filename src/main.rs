use ebyte_e32::{
    parameters::{air_baudrate::AirBaudRate, baudrate::BaudRate, Persistence},
    Ebyte,
};
use embedded_hal::prelude::*;
use linux_embedded_hal::{Delay, Pin, Serial};
use nb::block;

fn main() {
    let serial = Serial::open("/dev/ttyAMA0").unwrap();

    let aux = Pin::new(18);
    let m0 = Pin::new(23);
    let m1 = Pin::new(24);

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
            Ok(byte) => {
                block!(ebyte.write(byte)).unwrap();
                print!("{}", byte);
            }
        }
    }
}
