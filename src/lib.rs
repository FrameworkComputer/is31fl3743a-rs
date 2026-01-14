#![no_std]
#![doc = include_str!("../README.md")]
/// Preconfigured devices
pub mod devices;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

/// A struct to integrate with a new IS31FL3743A powered device.
pub struct IS31FL3743<I2C> {
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
    /// bus.
    pub calc_pixel: fn(x: u8, y: u8) -> u8,
}

impl<I2C, I2cError> IS31FL3743<I2C>
where
    I2C: I2c<Error = I2cError>,
{
    /// Fill all pixels of the display at once. The brightness should range from 0 to 255.
    /// brightness slice must have 0xC6 elements
    pub fn fill_matrix(&mut self, brightnesses: &[u8]) -> Result<(), I2cError> {
        // Extend by one, to add address to the beginning
        let mut buf = [0x00; 0xC7];
        buf[0] = 0x00; // set the initial address
        buf[1..=0xC6].copy_from_slice(brightnesses);
        self.bank(Page::Pwm)?;
        self.write(&buf)?;
        Ok(())
    }

    /// Read back the currently displayed matrix
    pub fn read_matrix(&mut self) -> Result<[u8; 0xC6], I2cError> {
        let mut buf = [0x00; 0xC6];
        self.bank(Page::Pwm)?;
        self.i2c.write(self.address, &[0x01])?;
        self.i2c.read(self.address, &mut buf)?;
        Ok(buf)
    }

    /// Fill the display with a single brightness. The brightness should range from 0 to 255.
    pub fn fill(&mut self, brightness: u8) -> Result<(), I2cError> {
        self.bank(Page::Pwm)?;
        let mut buf = [brightness; 0xC7];
        buf[0] = 0x00; // set the initial address
        self.write(&buf)?;
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
    pub fn setup<DEL: DelayNs>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
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
        let pixel = (self.calc_pixel)(x, y);
        self.write_register(Page::Pwm, pixel, brightness)?;
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
    /// This will result in all registers being restored to their defaults.
    pub fn reset<DEL: DelayNs>(&mut self, delay: &mut DEL) -> Result<(), I2cError> {
        self.write_register(Page::Config, addresses::RESET_REGISTER, addresses::RESET)?;
        delay.delay_ms(10);
        Ok(())
    }

    /// Reset the controller and restore all registers
    pub fn reset_restore<DEL: DelayNs>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
        // Back up registers
        let prev_config = self.read_register(Page::Config, addresses::CONFIG_REGISTER)?;
        // Assumes all scaling registers are set to the same value
        let prev_scale = self.read_register(Page::Scale, 0x01)?;
        let prev_brightness = self.read_matrix()?;

        self.setup(delay)?;

        // Restore registers
        self.write_register(Page::Config, addresses::CONFIG_REGISTER, prev_config)?;
        self.set_scaling(prev_scale)?;
        self.fill_matrix(&prev_brightness)?;
        Ok(())
    }

    /// Set the current available to each LED. 0 is none, 255 is the maximum available
    pub fn set_scaling(&mut self, scale: u8) -> Result<(), I2cError> {
        self.bank(Page::Scale)?;
        let mut buf = [scale; 0xC7];
        buf[0] = 0x00; // set the initial address
        self.write(&buf)?;
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

    /// How many SW rows to enable
    pub fn sw_enablement(&mut self, setting: SwSetting) -> Result<(), I2cError> {
        let config_register = self.read_register(Page::Config, addresses::CONFIG_REGISTER)?;

        let new_val = (config_register & 0x0F) | (setting as u8) << 4;
        self.write_register(Page::Config, addresses::CONFIG_REGISTER, new_val)?;
        Ok(())
    }

    /// Set the PWM frequency
    pub fn set_pwm_freq<DEL: DelayNs>(
        &mut self,
        delay: &mut DEL,
        pwm: PwmFreq,
    ) -> Result<(), Error<I2cError>> {
        // The default frequency, can't set it. Reset the controller and restore the registers
        if let PwmFreq::P29k = pwm {
            self.reset_restore(delay)?;
            return Ok(());
        }

        // Enter test mode
        self.write_register(Page::Config, addresses::TEST_MODE_REGISTER, 0x01)?;

        // Set PWM
        self.write(&[addresses::PWM_CONFIG_REGISTER, pwm as u8])?; // 488Hz

        // Exit test mode
        self.write_register(Page::Config, addresses::TEST_MODE_REGISTER, 0x00)?;

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

    fn read_u8(&mut self, register: u8) -> Result<u8, I2cError> {
        let mut buf = [0x00];
        self.i2c.write(self.address, &[register])?;
        self.i2c.read(self.address, &mut buf)?;
        Ok(buf[0])
    }

    fn read_register(&mut self, bank: Page, register: u8) -> Result<u8, I2cError> {
        self.bank(bank)?;
        let value = self.read_u8(register)?;
        Ok(value)
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

/// See the [data sheet](https://lumissil.com/assets/pdf/core/IS31FL3743A_DS.pdf)
/// for more information on registers.
pub mod addresses {
    // In Page 4
    pub const CONFIG_REGISTER: u8 = 0x00;
    pub const CURRENT_REGISTER: u8 = 0x01;
    pub const PULL_UP_REGISTER: u8 = 0x02;
    pub const RESET_REGISTER: u8 = 0x2F;

    pub const PAGE_SELECT_REGISTER: u8 = 0xFD;
    pub const CONFIG_LOCK_REGISTER: u8 = 0xFE;

    pub const TEST_MODE_REGISTER: u8 = 0xE0;
    pub const PWM_CONFIG_REGISTER: u8 = 0xE2;

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

#[repr(u8)]
enum Page {
    Pwm = 0x00,
    Scale = 0x01,
    Config = 0x02,
}

#[repr(u8)]
pub enum PwmFreq {
    /// 29kHz, the default. To set this, it'll reset the controller
    P29k = 0xFF,
    /// 31.25kHz
    P31k25 = 0xE0,
    /// 15.6kHz
    P15k6 = 0x20,
    /// 7.8kHz
    P7k8 = 0x40,
    /// 3.9kHz
    P3k9 = 0x60,
    /// 1.95kHz
    P1k95 = 0x80,
    /// 977Hz
    P977 = 0xA0,
    /// 488Hz
    P488 = 0xC0,
}

#[repr(u8)]
pub enum SwSetting {
    // SW1-SW11 active
    Sw1Sw11 = 0b0000,
    // SW1-SW10 active, SW11 not active
    Sw1Sw10 = 0b0001,
    // SW1-SW7 active, SW10-SW11 not active
    Sw1Sw9 = 0b0010,
    // SW1-SW8 active, SW9-SW11 not active
    Sw1Sw8 = 0b0011,
    // SW1-SW7 active, SW8-SW11 not active
    Sw1Sw7 = 0b0100,
    // SW1-SW6 active, SW7-SW11 not active
    Sw1Sw6 = 0b0101,
    // SW1-SW5 active, SW6-SW11 not active
    Sw1Sw5 = 0b0110,
    // SW1-SW4 active, SW5-SW11 not activee
    Sw1Sw4 = 0b0111,
    // SW1-SW3 active, SW4-SW11 not active
    Sw1Sw3 = 0b1000,
    // SW1-SW2 active, SW3-SW11 not active
    Sw1Sw2 = 0b1001,
    // All CSx pins only act as current sink, no scanning
    NoScan = 0b1010,
}
