# ebyte-e32-ui
Ebyte E32 Command Line Interface + minimal GUI

# Screenshots

![image](https://user-images.githubusercontent.com/6966738/167198228-d15e67e7-de91-4b65-a96f-f3ecb1c98f81.png)

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
