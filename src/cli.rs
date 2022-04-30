use clap::Parser;
use ebyte_e32::parameters::{
    air_baudrate::AirBaudRate,
    baudrate::BaudRate,
    option::{
        fec_mode::ForwardErrorCorrectionMode, io_drive_mode::IoDriveMode,
        transmission_power::TransmissionPower, wakeup_time::WakeupTime, TransmissionMode,
    },
    uart_parity::Parity,
};

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct App {
    #[clap(short, long, required = true)]
    address: u16,

    #[clap(short, long, required = true)]
    channel: u8,

    #[clap(long, required = false)]
    uart_parity: Parity,

    #[clap(long, required = false)]
    uart_rate: BaudRate,

    #[clap(long, required = false)]
    air_rate: AirBaudRate,

    #[clap(long, required = false)]
    transmission_mode: TransmissionMode,

    #[clap(short, long, required = false)]
    io_drive_mode: IoDriveMode,

    #[clap(short, long, required = false)]
    wakeup_time: WakeupTime,

    #[clap(short, long, required = false)]
    fec: ForwardErrorCorrectionMode,

    #[clap(long, required = false)]
    transmission_power: TransmissionPower,
}

impl From<App> for ebyte_e32::parameters::Parameters {
    fn from(app: App) -> Self {
        Self {
            address: app.address,
            channel: app.channel,
            uart_parity: app.uart_parity,
            uart_rate: app.uart_rate,
            air_rate: app.air_rate,
            transmission_mode: app.transmission_mode,
            io_drive_mode: app.io_drive_mode,
            wakeup_time: app.wakeup_time,
            fec: app.fec,
            transmission_power: app.transmission_power,
        }
    }
}
