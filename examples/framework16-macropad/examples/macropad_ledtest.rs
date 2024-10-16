//! LED Matrix Module
//!
//!
//!
//! Goes into bootloader mode when the host is asleep. This is to make it easy to reflash your
//! firmware - the regular bootloader mechanism using the DIP switch still works.
#![no_std]
#![no_main]
#![allow(clippy::needless_range_loop)]

use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_halt as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use bsp::entry;
use is31fl3743a::devices::Framework16Macropad;
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio, pac,
    sio::Sio,
    watchdog::Watchdog,
};
use fugit::RateExtU32;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Enable LED controller
    // SDB - Gpio29
    let mut led_enable = pins.voltage_monitor.into_push_pull_output();
    led_enable.set_high().unwrap();

    let i2c = bsp::hal::I2C::i2c1(
        pac.I2C1,
        pins.gpio26.into_mode::<gpio::FunctionI2C>(),
        pins.gpio27.into_mode::<gpio::FunctionI2C>(),
        1000.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let sleep = pins.gpio0.into_pull_up_input();

    // TODO: RGB Keyboard has two, one at 0x20 and one at 0x23
    // TODO: Macroapd has just a single one at 0x20
    let mut matrix = Framework16Macropad::configure(i2c, 0x20);
    matrix.setup(&mut delay).expect("failed to setup RGB controller");
    matrix.set_scaling(0xFF).expect("failed to set scaling");

    loop {
        for i in 9..14 {
            for color in 0..3 {
                matrix.device.pixel(i, color, 0xFF).expect("couldn't turn on");
                delay.delay_ms(1000);
            }
        }

        continue;

        // Light up each LED, one by one
        //for y in 0..matrix.device.height {
        for y in 0..6 {
            //for x in 0..matrix.device.width {
            for x in 0..4 {
                // Light up LED if system is not asleep
                if sleep.is_low().unwrap(){
                    matrix
                        .device
                        .pixel(x, y, 0xFF)
                        .expect("couldn't turn on");
                    delay.delay_ms(1000);
                    matrix
                        .device
                        .pixel(x, y+1, 0xFF)
                        .expect("couldn't turn on");
                    delay.delay_ms(1000);
                    matrix
                        .device
                        .pixel(x, y+2, 0xFF)
                        .expect("couldn't turn on");
                    delay.delay_ms(1000);
                    //matrix
                    //    .pixel_rgb(x, y, 0xFF, 0x00, 0x00)
                    //    .expect("couldn't turn on");
                    // delay.delay_ms(1000);
                    //matrix
                    //    .pixel_rgb(x, y, 0x00, 0xFF, 0x00)
                    //    .expect("couldn't turn on");
                    // delay.delay_ms(1000);
                    //matrix
                    //    .pixel_rgb(x, y, 0x00, 0x00, 0xFF)
                    //    .expect("couldn't turn on");
                    // delay.delay_ms(1000);
                } else {
                    // Turn all LEDs off
                    matrix.device.fill(0x00).expect("couldn't turn off");
                }

                delay.delay_ms(1000);
                matrix.device.fill(0x00).expect("couldn't turn off");
                // matrix
                //     .pixel_rgb(x, y, 0x00, 0x00, 0x00)
                //     .expect("couldn't turn off");
            }
        }
    }
}
