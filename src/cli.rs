use clap::Parser;
use ebyte_e32_parameters::{
    air_baudrate::AirBaudRate,
    baudrate::BaudRate,
    option::{
        fec_mode::ForwardErrorCorrectionMode, io_drive_mode::IoDriveMode,
        transmission_power::TransmissionPower, wakeup_time::WakeupTime, TransmissionMode,
    },
    uart_parity::Parity,
    Persistence,
};

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct App {
    #[clap(short, long, required = true)]
    address: u16,

    #[clap(short, long, required = true)]
    channel: u8,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    persistence: Persistence,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    uart_parity: Parity,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    uart_rate: BaudRate,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    air_rate: AirBaudRate,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    transmission_mode: TransmissionMode,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    io_drive_mode: IoDriveMode,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    wakeup_time: WakeupTime,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    fec: ForwardErrorCorrectionMode,

    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    transmission_power: TransmissionPower,
}

impl From<App> for ebyte_e32_parameters::Parameters {
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
