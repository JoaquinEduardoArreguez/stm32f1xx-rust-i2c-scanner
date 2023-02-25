// Rust I2C Scanner
// This assumes hal32f1xx I2C1 is used with standard mapping
// SCL -> PB6
// SDA -> PB7
// This will check addresses 0 to 127 as I2C addresses are tipically 7 bits long

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use panic_semihosting as _;
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.sysclk(8.MHz()).freeze(&mut flash.acr);

    // STARTING!
    hprintln!("Starting I2C Scan");

    // Acquire GPIOB, PB6 and PB7 for I2C1
    let mut gpiob = dp.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    // Config I2C1
    let mut i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio16to9,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    // I2C addresses are tipically 7 bits long, 0..127
    for address in 0..=127 {
        match i2c.write(address, &[1]) {
            Ok(_) => {
                hprint!("Found device on address {:#04X?}\n", address);
            }
            Err(_) => {}
        }
    }
    loop {}
}
