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

#[cfg(feature = "embedded_graphics")]
use embedded_graphics_core::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};

#[cfg(feature = "embedded_graphics")]
impl<I2C, I2cError> Dimensions for UnknownDevice<I2C>
where
    I2C: Write<Error = I2cError>,
    I2C: Read<Error = I2cError>,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Size::new(13, 9))
    }
}

#[cfg(feature = "embedded_graphics")]
impl<I2C, I2cError> DrawTarget for UnknownDevice<I2C>
where
    I2C: Write<Error = I2cError>,
    I2C: Read<Error = I2cError>,
    I2cError:,
{
    type Color = Rgb888;
    type Error = Error<I2cError>;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (63,63)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if let Ok((x @ 0..=13, y @ 0..=9)) = coord.try_into() {
                // Calculate the index in the framebuffer.
                self.pixel_rgb(x as u8, y as u8, color.r(), color.g(), color.b())?
            }
        }

        Ok(())
    }
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
                width: 18 * 11,
                height: 1,
                calc_pixel: |x: u8, y: u8| -> u8 {
                    // Dummy value, don't use this function
                    0x00
                },
            },
        }
    }

    pub fn setup<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
        self.device.setup(delay)
    }
}
