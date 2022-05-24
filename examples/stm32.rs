#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _;

// use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use is31fl3741::devices::AdafruitRGB13x9;

use stm32g0xx_hal::{
    prelude::*,
    rcc::{Config, Prescaler},
    stm32,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.freeze(Config::hsi(Prescaler::NotDivided));
    let mut delay = dp.TIM15.delay(&mut rcc);

    // let cp = cortex_m::Peripherals::take().unwrap();
    // let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split(&mut rcc);

    let sda = gpiob.pb9.into_open_drain_output_in_state(PinState::High);
    let scl = gpiob.pb8.into_open_drain_output_in_state(PinState::High);

    let i2c = dp.I2C1.i2c(sda, scl, 100.khz(), &mut rcc);

    // // https://github.com/adafruit/Adafruit_CircuitPython_IS31FL3741/blob/main/adafruit_is31fl3 741/adafruit_rgbmatrixqt.py#L53-L65

    let mut matrix = AdafruitRGB13x9::configure(i2c);
    matrix
        .setup(&mut delay)
        .expect("failed to setup rgb controller");

    matrix.set_scaling(0xFF).expect("failed to set scaling");

    loop {
        for y in 0..9 {
            for x in 0..13 {
                matrix
                    .pixel_rgb(x, y, 0x1E, 0x90, 0xFF)
                    .expect("couldn't turn on");
                delay.delay_ms(100u8);
                matrix.pixel_rgb(x, y, 0, 0, 0).expect("couldn't turn off");
            }
        }
    }
}
