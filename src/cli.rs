use clap::Parser;
use ebyte_e32::parameters::{
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
    /// Module Address (16 Bit).
    #[clap(short, long, required = true)]
    pub address: u16,

    /// Channel (8 Bit).
    #[clap(short, long, required = true)]
    pub channel: u8,

    /// Whether settings should be saved persistently on the module.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub persistence: Persistence,

    /// UART Parity.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub uart_parity: Parity,

    /// UART Baudrate.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub uart_rate: BaudRate,

    /// Air Baudrate.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub air_rate: AirBaudRate,

    /// Transmission Mode.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub transmission_mode: TransmissionMode,

    /// IO drive Mode for AUX pin.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub io_drive_mode: IoDriveMode,

    /// Wireless Wakeup Time.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub wakeup_time: WakeupTime,

    /// Forward Error Correction Mode.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub fec: ForwardErrorCorrectionMode,

    /// Transmission Power.
    #[clap(arg_enum, long, required = false, ignore_case(true), default_value_t)]
    pub transmission_power: TransmissionPower,

    /// Use GUI?
    #[clap(long, takes_value(false))]
    pub gui: bool,
}

impl From<&App> for ebyte_e32::parameters::Parameters {
    fn from(app: &App) -> Self {
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
