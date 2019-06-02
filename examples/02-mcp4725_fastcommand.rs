//! MPC4725 for the stm32f103. It uses the I2C 1 interface on pins pb8 and pb9 to control the DAC

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use hal::i2c::{BlockingI2c, Mode};
use hal::pac;
use hal::prelude::*;

use mcp4725::*;
#[allow(unused_imports)]
use panic_semihosting;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // Configure the pins for I2C1
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // Configure I2C
    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Standard { frequency: 100000 },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    // Configure the MCP4725 DAC
    let mut dac = MCP4725::create(i2c);

    // Create the dac command and set it to the right address
    let mut dac_cmd = Command::default().address(0b010);

    // Slowly decrease the output of the DAC to its minimum value, then start over
    let mut dac_cmd = FastCommand::default().set_address(0b010);
    let mut value: u16 = 0x0fff;

    loop {
        dac_cmd = dac_cmd.set_data(value);
        dac.send_fast(&dac_cmd);
        value -= 1;
        value &= 0x0fff;
    }
}
