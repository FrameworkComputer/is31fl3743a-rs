#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _;

// use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use is31fl3741::devices::AdafruitRGB13x9;

use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
use stm32g0xx_hal::{
    prelude::*,
    rcc::{Config, Prescaler},
    stm32,
};
use tinybmp::Bmp;

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

    let bmp_data = include_bytes!("gaypride.bmp");
    let bmp = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();
    Image::new(&bmp, Point::zero()).draw(&mut matrix).unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}
