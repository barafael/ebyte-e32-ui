use embedded_hal::{serial::Read, prelude::_embedded_hal_serial_Write};

fn main() {
    let mut serial = linux_embedded_hal::Serial::open("/dev/ttyAMA0").unwrap();
    
    loop {
        let x = nb::block!(serial.read()).unwrap();
        nb::block!(serial.write(x)).unwrap();
    }
}
