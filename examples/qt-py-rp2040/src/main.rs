#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _;

use adafruit_qt_py_rp2040::entry;
use adafruit_qt_py_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        i2c::I2C,
        pac,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use fugit::RateExtU32;
use is31fl3743a::devices::UnknownDevice;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
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
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Using STEMMA QT connector
    let sda = pins.sda1.into_mode(); // gpio22
    let scl = pins.scl1.into_mode(); // gpio23

    //let sda = pins.sda.into_mode(); // gpio24
    //let scl = pins.scl.into_mode(); // gpio25

    let i2c = I2C::i2c1(
        pac.I2C1,
        sda,
        scl,
        400.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    let mut matrix = UnknownDevice::configure(i2c);
    matrix
        .setup(&mut delay)
        .expect("failed to setup rgb controller");

    matrix.set_scaling(0xFF).expect("failed to set scaling");

    loop {
        matrix.device.fill(0xFF).expect("couldn't turn on");
        delay.delay_ms(100u32);
        matrix.device.fill(0x00).expect("couldn't turn off");

        // Light up each LED, one by one
        for y in 0..matrix.device.height {
            for x in 0..matrix.device.width {
                matrix
                    .device
                    .pixel(x, y, 0xFF)
                    .expect("couldn't turn on");
                delay.delay_ms(100);
                matrix.device.pixel(x, y, 0).expect("couldn't turn off");
            }
        }
        delay.delay_ms(100u32);
    }
}
