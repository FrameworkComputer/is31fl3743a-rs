#[allow(unused_imports)]
use crate::{Error, IS31FL3743};
#[allow(unused_imports)]
use core::convert::TryFrom;
#[allow(unused_imports)]
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::Read;
#[allow(unused_imports)]
use embedded_hal::blocking::i2c::Write;

pub struct UnknownDevice<I2C> {
    pub device: IS31FL3743<I2C>,
}

impl<I2C, I2cError> UnknownDevice<I2C>
where
    I2C: Write<Error = I2cError>,
    I2C: Read<Error = I2cError>,
{
    pub fn unwrap(self) -> I2C {
        self.device.i2c
    }

    pub fn set_scaling(&mut self, scale: u8) -> Result<(), I2cError> {
        self.device.set_scaling(scale)
    }

    pub fn configure(i2c: I2C) -> UnknownDevice<I2C> {
        UnknownDevice {
            device: IS31FL3743 {
                i2c,
                address: 0b0100000,
                // Dummy values, not used
                width: 18 * 11,
                // Dummy values, not used
                height: 1,
                calc_pixel: |_x: u8, _y: u8| -> u8 {
                    // Dummy value, don't use this function
                    unimplemented!("No Matrix support yet")
                },
            },
        }
    }

    pub fn setup<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
        self.device.setup(delay)
    }
}
