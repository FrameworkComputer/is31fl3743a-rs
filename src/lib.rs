#![no_std]
#![doc = include_str!("../README.md")]
/// Preconfigured devices
pub mod devices;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::Write;

/// A struct to integrate with a new IS31FL3741 powered device.
pub struct IS31FL3741<I2C> {
    /// The i2c bus that is used to interact with the device. See implementation below for the
    /// trait methods required.
    pub i2c: I2C,
    /// The 7-bit i2c slave address of the device. By default on most devices this is `0x74`.
    pub address: u8,
    /// Width of the LED matrix
    pub width: u8,
    /// Height of the LED matrix
    pub height: u8,
    /// Method to convert an x,y coordinate pair to a binary address that can be accessed using the
    /// bus. second value is the page the LED is on
    pub calc_pixel: fn(x: u8, y: u8) -> (u8, u8),
}

impl<I2C, I2cError> IS31FL3741<I2C>
where
    I2C: Write<Error = I2cError>,
{
    /// Fill the display with a single brightness. The brightness should range from 0 to 255.
    pub fn fill(&mut self, brightness: u8) -> Result<(), I2cError> {
        self.bank(Page::Pwm1)?;
        let mut buf = [brightness; 0xB5];
        buf[0] = 0x00; // set the initial address
        self.write(&buf)?;
        self.bank(Page::Pwm2)?;
        self.write(&buf[..=0xAB])?;
        Ok(())
    }

    /// Setup the display. Should be called before interacting with the device to ensure proper
    /// functionality. Delay is something that your device's HAL should provide which allows for
    /// the process to sleep for a certain amount of time (in this case 10 MS to perform a reset).
    ///
    /// When you run this function the following steps will occur:
    /// 1. The chip will be told that it's being "reset".
    /// 2. The chip will be put in shutdown mode
    /// 3. The chip will be configured to use the maximum voltage
    /// 4. The chip will be taken out of shutdown mode
    pub fn setup<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
        self.reset(delay)?;
        self.shutdown(true)?;
        delay.delay_ms(10);
        // maximum current limiting
        self.write_register(Page::Config, addresses::CURRENT_REGISTER, 0xFF)?;
        self.shutdown(false)?;
        Ok(())
    }
    /// Set the brightness at a specific x,y coordinate. Just like the [fill method](Self::fill)
    /// the brightness should range from 0 to 255. If the coordinate is out of range then the
    /// function will return an error of [InvalidLocation](Error::InvalidLocation).
    pub fn pixel(&mut self, x: u8, y: u8, brightness: u8) -> Result<(), Error<I2cError>> {
        if x > self.width {
            return Err(Error::InvalidLocation(x));
        }
        if y > self.height {
            return Err(Error::InvalidLocation(y));
        }
        let (pixel, frame) = (self.calc_pixel)(x, y);
        let bank = if frame == 0 { Page::Pwm1 } else { Page::Pwm2 };
        self.write_register(bank, pixel, brightness)?;
        Ok(())
    }

    /// Change the slave address to a new 7-bit address. Should be configured before calling
    /// [setup](Self::setup) method.
    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    /// Send a reset message to the slave device. Delay is something that your device's HAL should
    /// provide which allows for the process to sleep for a certain amount of time (in this case 10
    /// MS to perform a reset).
    pub fn reset<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), I2cError> {
        self.write_register(Page::Config, addresses::RESET_REGISTER, addresses::RESET)?;
        delay.delay_ms(10);
        Ok(())
    }

    /// Set the current available to each LED. 0 is none, 255 is the maximum available
    pub fn set_scaling(&mut self, scale: u8) -> Result<(), I2cError> {
        self.bank(Page::Scale1)?;
        let mut buf = [scale; 0xB5];
        buf[0] = 0x00; // set the initial address
        self.write(&buf)?;
        self.bank(Page::Scale2)?;
        self.write(&buf[..=0xAB])?;
        Ok(())
    }

    /// Put the device into software shutdown mode
    pub fn shutdown(&mut self, yes: bool) -> Result<(), I2cError> {
        self.write_register(
            Page::Config,
            addresses::CONFIG_REGISTER,
            if yes { 0 } else { 1 },
        )?;
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<(), I2cError> {
        self.i2c.write(self.address, buf)
    }

    fn write_register(&mut self, bank: Page, register: u8, value: u8) -> Result<(), I2cError> {
        self.bank(bank)?;
        self.write(&[register, value])?;
        Ok(())
    }

    fn bank(&mut self, bank: Page) -> Result<(), I2cError> {
        self.unlock()?;
        self.write(&[addresses::PAGE_SELECT_REGISTER, bank as u8])?;
        Ok(())
    }

    fn unlock(&mut self) -> Result<(), I2cError> {
        self.i2c.write(
            self.address,
            &[
                addresses::CONFIG_LOCK_REGISTER,
                addresses::CONFIG_WRITE_ENABLE,
            ],
        )
    }
}

/// See the [data sheet](https://lumissil.com/assets/pdf/core/IS31FL3741A_DS.pdf)
/// for more information on registers.
pub mod addresses {
    pub const CONFIG_REGISTER: u8 = 0x00;
    pub const CURRENT_REGISTER: u8 = 0x01;
    pub const PULL_UP_REGISTER: u8 = 0x02;
    pub const RESET_REGISTER: u8 = 0x3F;
    pub const SHUTDOWN: u8 = 0x0A;

    pub const PAGE_SELECT_REGISTER: u8 = 0xFD;
    pub const CONFIG_LOCK_REGISTER: u8 = 0xFE;

    pub const CONFIG_WRITE_ENABLE: u8 = 0b1100_0101;
    pub const RESET: u8 = 0xAE;
}

#[derive(Clone, Copy, Debug)]
pub enum Error<I2cError> {
    I2cError(I2cError),
    InvalidLocation(u8),
    InvalidFrame(u8),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}

enum Page {
    Pwm1 = 0x00,
    Pwm2 = 0x01,
    Scale1 = 0x02,
    Scale2 = 0x03,
    Config = 0x04,
}
