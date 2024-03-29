# ebyte-e32-ui

Ebyte E32 Command Line Interface + minimal GUI.

Works with Ebyte-E32 LoRa modules with configurable pin assignment.
The 3 GPIOs are controlled via linux cdev, and
the serial port is controlled by a normal `/dev/ttyXXX` character device.

Uses [ebyte-e32-rs](https://github.com/barafael/ebyte-e32-rs) as a driver,
plus some traits from [embedded-hal](https://github.com/rust-embedded/embedded-hal) and their implementations from [linux-embedded-hal](https://github.com/rust-embedded/linux-embedded-hal).

For the CLI, [clap](https://github.com/clap-rs/clap) is used.

For the GUI, on top of clap, [klask](https://github.com/MichalGniadek/klask) is used.

## Example Pinout and Configuration

Configurable with `Config.toml` in the same directory as the binary
(or set the option `config`):

```toml
serial_path = "/dev/ttyAMA0"
baudrate = 9600
parity = "None"
data_bits = 8
stop_bits = 1
gpiochip_path = "/dev/gpiochip0"
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

## CLI Usage Examples

In these examples, you can substitute `ebyte-e32-cli` with `cargo run --bin ebyte-e32-cli`
if you are in the project root directory.
This directory also contains an example `Config.toml`.

* Reading model data or parameters (good first test):

    ```text
    ebyte-e32-cli {read-model-data|read-parameters}
    ```

* Listen for transmissions:

    ```text
    ebyte-e32-cli listen
    ```
* Configure for address 1, channel 16, no forward error correction, permanent persistence:

    ```text
    ebyte-e32-cli configure --address 1 --channel 16 --fec off --persistence permanent
    ```

For `send` mode, enter your messages in the prompt or pipe them in via `stdin`.

## GUI Usage Notes

The GUI is the default target, meaning you can launch it with `cargo run`.
For sending data, the data provided in the `Input` tab is used (where you can also select a file to read the input from).

## Configuration and Persistence

With the `Configuration` subcommand, the module parameters can be adjusted.
Note that your module settings have to match the other module's settings for any transmission to be successful.

With the `persistence` option of the `Configuration` subcommand, the settings can be saved (`temporary` or `permanent`).
In `permanent` mode, the settings will be persisted onto the module.

## Screenshots

You can run the GUI on your normal OS for testing.

These screenshots are slightly outdated but show the features of the GUI and CLI:

[comment]: # (https://user-images.githubusercontent.com/6966738/198895795-4d86303b-e282-4a05-8b39-d640ff9164c3.png)
[comment]: # (https://user-images.githubusercontent.com/6966738/198894630-6973925f-c167-433e-9375-8420d34d75a4.png)
[comment]: # (https://user-images.githubusercontent.com/6966738/198893498-46752f22-6d51-4489-af25-c0965a7860d1.png)
[comment]: # (https://user-images.githubusercontent.com/6966738/198893565-c187172b-1537-4fb9-a781-10f4e9425a33.png)
[comment]: # (https://user-images.githubusercontent.com/6966738/198893601-feab2e98-d819-40b2-bb85-fffc83d21f9f.png)
[comment]: # (https://user-images.githubusercontent.com/6966738/198895149-eb45ee81-d6e2-42b1-b0a5-840b524bc5f5.png)
[comment]: # (https://user-images.githubusercontent.com/6966738/198895230-c795f820-9af3-45fb-bbad-dd2ba84f4b48.png)
[comment]: # (https://user-images.githubusercontent.com/6966738/198895335-6e06b4ac-0d1c-4260-b8bc-c8e662ad4567.png)

<img src="https://user-images.githubusercontent.com/6966738/198895795-4d86303b-e282-4a05-8b39-d640ff9164c3.png" width="23%"></img> <img src="https://user-images.githubusercontent.com/6966738/198894630-6973925f-c167-433e-9375-8420d34d75a4.png" width="23%"></img> <img src="https://user-images.githubusercontent.com/6966738/198893498-46752f22-6d51-4489-af25-c0965a7860d1.png" width="23%"></img> <img src="https://user-images.githubusercontent.com/6966738/198893565-c187172b-1537-4fb9-a781-10f4e9425a33.png" width="23%"></img> <img src="https://user-images.githubusercontent.com/6966738/198893601-feab2e98-d819-40b2-bb85-fffc83d21f9f.png" width="23%"></img> <img src="https://user-images.githubusercontent.com/6966738/198895149-eb45ee81-d6e2-42b1-b0a5-840b524bc5f5.png" width="23%"></img> <img src="https://user-images.githubusercontent.com/6966738/198895230-c795f820-9af3-45fb-bbad-dd2ba84f4b48.png" width="23%"></img> <img src="https://user-images.githubusercontent.com/6966738/198895335-6e06b4ac-0d1c-4260-b8bc-c8e662ad4567.png" width="23%"></img>

## System Requirements + Portability

The underlying driver ([ebyte-e32-rs](https://github.com/barafael/ebyte-e32-rs)) is platform-agnostic
(doesn't require linux, just implementations of `embedded-hal`).

This program requires a linux character device and 3 CDEV GPIO lines.

For the character device, it should be available as `/dev/ttyABC`.
Note that (depending on your distribution)
you may have to add your user to certain groups
to be able to use the serial port, like `dialout`.

For the GPIO lines, find your boards `gpiochipXXX` in
`/sys/class/gpio/gpiochipXXX`, then
find the lines you want to connect and
enter the info in `Config.toml`.

The AUX line must support being an input line,
the M0 and M1 lines must support being output lines.

## CLI Help

Top-level help:

```text
ebyte-e32-ui 0.5.0
CLI + GUI for interacting with EByte E32 LoRa modules

USAGE:
    ebyte-e32-cli [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --config <CONFIG>    Configuration file [default: Config.toml]
    -h, --help               Print help information
    -V, --version            Print version information

SUBCOMMANDS:
    configure          Write Ebyte module parameters
    help               Print this message or the help of the given subcommand(s)
    listen             Listen for incoming data on the Ebyte module
    read-model-data    Read Ebyte module data and print to stdout
    read-parameters    Read Ebyte module parameters and print to stdout
    send               Send data from stdin over the Ebyte module
```

Configuration subcommand help (`ebyte-e32-cli help configure`):

```text
ebyte-e32-cli-configure
Write Ebyte module parameters

USAGE:
    ebyte-e32-cli configure [OPTIONS] --address <ADDRESS> --channel <CHANNEL>

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

        --wakeup-time <WAKEUP_TIME>
            Wireless Wakeup Time [default: ms250] [possible values: ms250, ms500, ms750, ms1000,
            ms1250, ms1500, ms1750, ms2000]
```

The other subcommands have no further options.

## Raspberry Pi Serial Port Setup

The serial port hardware peripheral must be enabled (but without login shell).

## Simplified Dependency Graph

![graph](graph.png)
