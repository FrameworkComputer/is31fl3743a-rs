//! # Framework 16 RGB Keyboard/Macropad LED Example
//!
//! Demonstrates controlling the IS31FL3743A LED controllers on the
//! Framework 16 RGB Keyboard or Macropad.
//!
//! ## RGB Keyboard (default)
//! - Two IS31FL3743A controllers at I2C addresses 0x20 and 0x23
//! - SW1-SW9 active
//! - Build with: `cargo build --release`
//!
//! ## Macropad
//! - Single IS31FL3743A controller at I2C address 0x20
//! - SW1-SW4 active
//! - Build with: `cargo build --release --features macropad --no-default-features`
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// Require exactly one of keyboard or macropad feature
#[cfg(all(feature = "keyboard", feature = "macropad"))]
compile_error!("Features 'keyboard' and 'macropad' are mutually exclusive. Choose one.");

#[cfg(not(any(feature = "keyboard", feature = "macropad")))]
compile_error!("You must enable either the 'keyboard' or 'macropad' feature. Example: cargo build --features keyboard");

// Stop compilation here if feature check failed - provide dummies to suppress other errors
#[cfg(not(any(feature = "keyboard", feature = "macropad")))]
mod dummy {
    pub use is31fl3743a::SwSetting;
    pub const SW_SETTING: SwSetting = SwSetting::Sw1Sw11;
    pub const ADDR_CTRL2: u8 = 0;
}
#[cfg(not(any(feature = "keyboard", feature = "macropad")))]
use dummy::*;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use panic_halt as _;

use framework16_keyboard::entry;
use framework16_keyboard::{
    hal::{
        clocks::init_clocks_and_plls, fugit::RateExtU32, gpio::PullUp, i2c::I2C, pac,
        watchdog::Watchdog, Sio, Timer,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use is31fl3743a::devices::UnknownDevice;
#[cfg(any(feature = "keyboard", feature = "macropad"))]
use is31fl3743a::SwSetting;

/// I2C address of the first (or only) LED controller
const ADDR_CTRL1: u8 = 0x20;

/// I2C address of the second LED controller (keyboard only)
#[cfg(feature = "keyboard")]
const ADDR_CTRL2: u8 = 0x23;

/// SW enablement setting - use keyboard value unless only macropad is enabled
#[cfg(feature = "keyboard")]
const SW_SETTING: SwSetting = SwSetting::Sw1Sw9;

#[cfg(all(feature = "macropad", not(feature = "keyboard")))]
const SW_SETTING: SwSetting = SwSetting::Sw1Sw4;

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

    // Set SDB high to enable the LED controllers
    let mut sdb = pins.sdb.into_push_pull_output();
    sdb.set_high().ok();

    // Set up I2C on GPIO26 (SDA) and GPIO27 (SCL)
    let sda: framework16_keyboard::hal::gpio::Pin<_, _, PullUp> = pins.gpio26.reconfigure();
    let scl: framework16_keyboard::hal::gpio::Pin<_, _, PullUp> = pins.gpio27.reconfigure();

    let i2c = I2C::i2c1(
        pac.I2C1,
        sda,
        scl,
        400.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    // Configure the first controller
    let mut matrix = UnknownDevice::configure(i2c);
    matrix.device.address = ADDR_CTRL1;
    matrix
        .setup(&mut timer)
        .expect("failed to setup LED controller 1");
    matrix.set_scaling(0xFF).expect("failed to set scaling");
    matrix
        .device
        .sw_enablement(SW_SETTING)
        .expect("failed to set SW enablement");

    // Configure the second controller (keyboard only)
    #[cfg(feature = "keyboard")]
    {
        matrix.device.address = ADDR_CTRL2;
        matrix
            .setup(&mut timer)
            .expect("failed to setup LED controller 2");
        matrix.set_scaling(0xFF).expect("failed to set scaling");
        matrix
            .device
            .sw_enablement(SW_SETTING)
            .expect("failed to set SW enablement");
    }

    // Main loop: cycle through colors
    loop {
        // Red (LED order is BGR, so red is at offset 2)
        set_all_color(&mut matrix, 0x00, 0x00, 0x40);
        timer.delay_ms(500);

        // Green (offset 1)
        set_all_color(&mut matrix, 0x00, 0x40, 0x00);
        timer.delay_ms(500);

        // Blue (offset 0)
        set_all_color(&mut matrix, 0x40, 0x00, 0x00);
        timer.delay_ms(500);

        // White
        set_all_color(&mut matrix, 0x20, 0x20, 0x20);
        timer.delay_ms(500);

        // Off
        set_all_color(&mut matrix, 0x00, 0x00, 0x00);
        timer.delay_ms(500);
    }
}

/// Set all LEDs on all controllers to a specific BGR color
fn set_all_color<I2C, E>(matrix: &mut UnknownDevice<I2C>, b: u8, g: u8, r: u8)
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    matrix.device.address = ADDR_CTRL1;
    fill_color(&mut matrix.device, b, g, r);

    #[cfg(feature = "keyboard")]
    {
        matrix.device.address = ADDR_CTRL2;
        fill_color(&mut matrix.device, b, g, r);
    }
}

/// Fill all LEDs with a specific BGR color
fn fill_color<I2C, E>(device: &mut is31fl3743a::IS31FL3743<I2C>, b: u8, g: u8, r: u8)
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    // Each LED uses 3 registers in BGR order
    // Total of 0xC6 (198) registers = 66 RGB LEDs max
    let mut buf = [0u8; 0xC6];
    for i in 0..66 {
        buf[i * 3] = b;
        buf[i * 3 + 1] = g;
        buf[i * 3 + 2] = r;
    }
    device.fill_matrix(&buf).ok();
}
