# stm32f1xx-rust-i2c-scanner
I2C bus scanner written for `stm32f1xx` microcontrollers in rust.

Based on:
* [stm32f1xx-hal documentation](https://github.com/stm32-rs/stm32f1xx-hal)
* [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/)
* [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html)

## Setup

You need to install arm target support for the Rust compiler:
```
rustup target install thumbv7m-none-eabi
```

In order to use your terminal to debug and receive messages from the mcu, you will need a debug probe like `stlinkv2`.  
You will also need to install `GDB` and `OpenOCD` as explaine [here](https://docs.rust-embedded.org/book/intro/install.html).  

for mac:
```
$ # GDB
$ brew install armmbed/formulae/arm-none-eabi-gcc

$ # OpenOCD
$ brew install openocd
```

## Build and run

Build for your mcu target, in the case of `blue pill`:
```
cargo build
```
and flash the code to your mcu:
```
cargo flash --chip stm32f103C8
```

Connect your debug probe and run `openocd` on your terminal, selecting the corresponding interface for your probe.  

In the case of `stlink` running with `stlink.cfg` will select for the correct version automatically:
```
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
```
This will run `openocd` and wait for a `gdb` process to connect. You should see a message like:
```
...
Info : Listening on port 3333 for gdb connections
```

Now you can run your code in your mcu using `openocd` and `gdb` to interface between the mcu and your terminal.

On a separated terminal, run
```
cargo run target/thumbv7m-none-eabi/debug/rust-i2c-scanner
```
an `continue` so your code runs. You should see a message like:
```
...
Single stepping until exit from function Reset,
--Type <RET> for more, q to quit, c to continue without paging--c
which has no line number information.
halted: PC: 0x080056a6
cortex_m_rt::DefaultPreInit () at src/lib.rs:1058
1058	pub unsafe extern "C" fn DefaultPreInit() {}
(gdb) c
Continuing.
```

You should see the messages sent by the mcu on the terminal running `openocd`:
```
...
Info : flash size = 128kbytes
Warn : Prefer GDB command "target extended-remote 3333" instead of "target remote 3333"
semihosting is enabled

target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08000130 msp: 0x20005000, semihosting
Info : Padding image section 0 at 0x08006684 with 12 bytes
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08000130 msp: 0x20005000, semihosting
Info : halted: PC: 0x080056a6
Starting I2C Scan
Found device on address 0x3C
Found device on address 0x68
```
In my case I have two devices connected to I2C1:
```
Found device on address 0x3C
Found device on address 0x68
```
