# ebyte-e32-ui
Ebyte E32 Command Line Interface + minimal GUI.

Works with Ebyte-E32 LoRa modules. Uses [ebyte-e32-rs](https://github.com/barafael/ebyte-e32-rs) as a driver, plus some traits from [embedded-hal](https://github.com/rust-embedded/embedded-hal) and their implementations from [RPPAL](https://github.com/golemparts/rppal).

For the CLI, [clap](https://github.com/clap-rs/clap) is used. For the GUI, on top of clap, [klask](https://github.com/MichalGniadek/klask) is used.

# Pinout

| Ebyte Pin | Raspberry Pi Pin (BCM pin number) |
|-----------|-----------------------------------|
| VCC       | 3v3                               |
| GND       | GND                               |
| AUX       | 18                                |
| M0        | 23                                |
| M1        | 24                                |
| TX        | 15 (RX)                           |
| RX        | 14 (TX)                           |

# Usage

 * Run `cargo run --bin ebyte-e32-cli [OPTIONS] --address --channel` (options etc. see below) for the CLI. In `Send` mode, a REPL-like interface is provided.
 * For the GUI, run `cargo run --bin ebyte-e32-gui`. In `Send` mode, the input provided in the `Input` tab is sent (there, you can also open a file but I never tested that :D).

# Screenshots

![image](https://user-images.githubusercontent.com/6966738/167198228-d15e67e7-de91-4b65-a96f-f3ecb1c98f81.png)

(As can be seen here, you can run the GUI on your normal OS, just usually it wont have a serial port ang GPIOs like the PI 3)

# Portability

The UI program will only work on Raspberry Pi 3. It could be simply extended to run on other systems which have e.g. linux-embedded-hal support, though currently the RPPAL crate is used (but simple to replace with linux-embedded-hal).

Of course, the underlying driver is completely independent.

# CLI Help

```
ebyte-e32-ui 0.1.0                                                                                                         
                                                                                                                          
USAGE:                                                                                                                     
   ebyte-e32-cli [OPTIONS] --address --channel                                           
                                                                                                                          
OPTIONS:                                                                                                                   
   -a, --address                                                                                                
           Module Address (16 Bit)                                                                                        
                                                                                                                          
       --air-rate                                                                                              
           Air Baudrate [default: bps2400] [possible values: bps300, bps1200, bps2400, bps4800,                           
           bps9600, bps19200]                                                                                             
                                                                                                                          
   -c, --channel                                                                                                
           Channel (8 Bit)                                                                                                
                                                                                                                          
       --fec                                                                                                        
           Forward Error Correction Mode [default: on] [possible values: on, off]                                         
                                                                                                                          
   -h, --help                                                                                                             
           Print help information                                                                                         
                                                                                                                          
       --io-drive-mode                                                                                    
           IO drive Mode for AUX pin [default: push-pull] [possible values: push-pull, open-                              
           collector]                                                                                                     
                                                                                                                          
       --persistence                                                                                        
           Whether settings should be saved persistently on the module [default: temporary]                               
           [possible values: temporary, permanent]                                                                        
                                                                                                                          
       --transmission-mode                                                                            
           Transmission Mode [default: transparent] [possible values: transparent, fixed]                                 
                                                                                                                          
       --transmission-power                                                                          
           Transmission Power [default: dbm30] [possible values: dbm30, dbm27, dbm24, dbm21]                              
                                                                                                                          
       --uart-parity                                                                                        
           UART Parity [default: none] [possible values: none, odd, even]                                                 
                                                                                                                          
       --uart-rate                                                                                            
           UART Baudrate [default: bps9600] [possible values: bps1200, bps2400, bps4800, bps9600,                         
           bps19200, bps38400, bps57600, bps115200]                                                                       
                                                                                                                          
   -V, --version                                                                                                          
           Print version information                                                                                      
                                                                                                                          
       --wakeup-time                                                                                        
           Wireless Wakeup Time [default: ms250] [possible values: ms250, ms500, ms750, ms1000,                           
           ms1250, ms1500, ms1750, ms2000]                                                                                
                                                                                                                          
SUBCOMMANDS:                                                                                                               
   help      Print this message or the help of the given subcommand(s)                                                    
   listen                                                                                                                 
   send                                                                                                                   
```

# Raspberry Pi Serial Port Setup

 * Set serial port hardware to be enabled, but without the login shell.
 * Install something like the linaro cross compiler: 

```
aur/arm-linux-gnueabihf-gcc75-linaro-bin
```

 * Build with 

```
cargo b --target armv7-unknown-linux-musleabihf
```

# Simplified Dependency Graph

![graph](https://user-images.githubusercontent.com/6966738/167207702-d781f5c7-ec73-4785-8b22-783575306463.png)
