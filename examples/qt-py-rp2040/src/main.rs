#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _;

use embedded_hal::delay::DelayNs;

use adafruit_qt_py_rp2040::entry;
use adafruit_qt_py_rp2040::{
    hal::{
        clocks::init_clocks_and_plls, fugit::RateExtU32, gpio::PullUp, i2c::I2C, pac,
        watchdog::Watchdog, Sio, Timer,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use is31fl3743a::devices::UnknownDevice;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

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

    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Using STEMMA QT connector
    let sda: adafruit_qt_py_rp2040::hal::gpio::Pin<_, _, PullUp> = pins.sda1.reconfigure(); // gpio22
    let scl: adafruit_qt_py_rp2040::hal::gpio::Pin<_, _, PullUp> = pins.scl1.reconfigure(); // gpio23

    //let sda: adafruit_qt_py_rp2040::hal::gpio::Pin<_, _, PullUp> = pins.sda.reconfigure(); // gpio24
    //let scl: adafruit_qt_py_rp2040::hal::gpio::Pin<_, _, PullUp> = pins.scl.reconfigure(); // gpio25

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
        .setup(&mut timer)
        .expect("failed to setup rgb controller");

    matrix.set_scaling(0xFF).expect("failed to set scaling");

    loop {
        matrix.device.fill(0xFF).expect("couldn't turn on");
        timer.delay_ms(100);
        matrix.device.fill(0x00).expect("couldn't turn off");
    }
}
