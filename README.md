# ebyte-e32-ui

Ebyte E32 Command Line Interface + minimal GUI.

Works with Ebyte-E32 LoRa modules on raspberry pi with configurable pin assignment.

Uses [ebyte-e32-rs](https://github.com/barafael/ebyte-e32-rs) as a driver, plus some traits from [embedded-hal](https://github.com/rust-embedded/embedded-hal) and their implementations from [RPPAL](https://github.com/golemparts/rppal).

For the CLI, [clap](https://github.com/clap-rs/clap) is used.

For the GUI, on top of clap, [klask](https://github.com/MichalGniadek/klask) is used.

## Example Pinout

Configurable with `Config.toml` in the same directory as the binary:

```toml
serial_path = "/dev/ttyAMA0"
baudrate = 9600
parity = "None"
data_bits = 8
stop_bits = 1
aux_pin = 18
m0_pin = 23
m1_pin = 24
```

| Ebyte Pin | Raspberry Pi Pin (BCM pin number) |
|-----------|-----------------------------------|
| VCC       | 3v3                               |
| GND       | GND                               |
| AUX       | 18                                |
| M0        | 23                                |
| M1        | 24                                |
| TX        | 15 (RX)                           |
| RX        | 14 (TX)                           |

## Usage

* CLI: `cargo run --bin ebyte-e32-cli -- [OPTIONS] --address <ADDRESS> --channel <CHANNEL> {listen|send|read-model-data}`. For `send` mode, enter your messages in the prompt or pipe them in via `stdin`.
* GUI: `cargo run` or `cargo run --bin ebyte-e32-gui`. For `send` mode, the input provided in the `Input` tab is sent (there, you can also open a file to read the input from).

## Persistence

With the `persistence` argument, the settings can be saved `temporary` or `permanent`.

## Screenshots

![image](https://user-images.githubusercontent.com/6966738/167198228-d15e67e7-de91-4b65-a96f-f3ecb1c98f81.png)

You can run the GUI on your normal OS for testing.

## Portability

This program will only work on Raspberry Pi because of its dependency on `rppal`. Maybe at some point it should run on any system with `linux-embedded-hal`.

Of course, the underlying driver ([ebyte-e32-rs](https://github.com/barafael/ebyte-e32-rs)) is platform-agnostic.

## CLI Help

```text
ebyte-e32-ui 0.2.0

USAGE:
    ebyte-e32-cli [OPTIONS] --address <ADDRESS> --channel <CHANNEL> <SUBCOMMAND>

OPTIONS:
    -a, --address <ADDRESS>
            Module Address (16 Bit)

        --air-rate <AIR_RATE>
            Air Baudrate [default: bps2400] [possible values: bps300, bps1200, bps2400, bps4800,
            bps9600, bps19200]

    -c, --channel <CHANNEL>
            Channel (8 Bit)

        --fec <FEC>
            Forward Error Correction Mode [default: on] [possible values: on, off]

    -h, --help
            Print help information

        --io-drive-mode <IO_DRIVE_MODE>
            IO drive Mode for AUX pin [default: push-pull] [possible values: push-pull,
            open-collector]

        --persistence <PERSISTENCE>
            Whether settings should be saved persistently on the module [default: temporary]
            [possible values: temporary, permanent]

        --transmission-mode <TRANSMISSION_MODE>
            Transmission Mode [default: transparent] [possible values: transparent, fixed]

        --transmission-power <TRANSMISSION_POWER>
            Transmission Power [default: dbm30] [possible values: dbm30, dbm27, dbm24, dbm21]

        --uart-parity <UART_PARITY>
            UART Parity [default: none] [possible values: none, odd, even]

        --uart-rate <UART_RATE>
            UART Baudrate [default: bps9600] [possible values: bps1200, bps2400, bps4800, bps9600,
            bps19200, bps38400, bps57600, bps115200]

    -V, --version
            Print version information

        --wakeup-time <WAKEUP_TIME>
            Wireless Wakeup Time [default: ms250] [possible values: ms250, ms500, ms750, ms1000,
            ms1250, ms1500, ms1750, ms2000]

SUBCOMMANDS:
    help               Print this message or the help of the given subcommand(s)
    listen
    read-model-data
    send
```

## Raspberry Pi Setup

The serial port hardware peripheral must be enabled (but without login shell).

## Simplified Dependency Graph

![graph](graph.png)
