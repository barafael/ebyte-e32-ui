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

#[derive(clap::Subcommand, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Listen,
    Send,
}

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct App {
    /// Listen for transmissions or send stdin?
    #[clap(subcommand)]
    pub mode: Mode,

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
}

impl From<&App> for ebyte_e32::parameters::Parameters {
    fn from(params: &App) -> Self {
        Self {
            address: params.address,
            channel: params.channel,
            uart_parity: params.uart_parity,
            uart_rate: params.uart_rate,
            air_rate: params.air_rate,
            transmission_mode: params.transmission_mode,
            io_drive_mode: params.io_drive_mode,
            wakeup_time: params.wakeup_time,
            fec: params.fec,
            transmission_power: params.transmission_power,
        }
    }
}
