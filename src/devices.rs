// #[cfg_attr(docsrs, doc(cfg(feature = "adafruit_rgb_13x9")))]
#[allow(unused_imports)]
use crate::{Error, IS31FL3741};
#[allow(unused_imports)]
use core::convert::TryFrom;
#[allow(unused_imports)]
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::Read;
#[allow(unused_imports)]
use embedded_hal::blocking::i2c::Write;

#[cfg(feature = "adafruit_rgb_13x9")]
pub struct AdafruitRGB13x9<I2C> {
    pub device: IS31FL3741<I2C>,
}

#[cfg(feature = "embedded_graphics")]
use embedded_graphics_core::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};

#[cfg(all(feature = "adafruit_rgb_13x9", feature = "embedded_graphics"))]
impl<I2C, I2cError> Dimensions for AdafruitRGB13x9<I2C>
where
    I2C: Write<Error = I2cError>,
    I2C: Read<Error = I2cError>,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Size::new(13, 9))
    }
}

#[cfg(all(feature = "adafruit_rgb_13x9", feature = "embedded_graphics"))]
impl<I2C, I2cError> DrawTarget for AdafruitRGB13x9<I2C>
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

#[cfg(feature = "adafruit_rgb_13x9")]
impl<I2C, I2cError> AdafruitRGB13x9<I2C>
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

    pub fn configure(i2c: I2C) -> AdafruitRGB13x9<I2C> {
        AdafruitRGB13x9 {
            device: IS31FL3741 {
                i2c,
                address: 0x30,
                width: 13 * 9,
                height: 3,
                calc_pixel: |x: u8, y: u8| -> (u8, u8) {
                    let lookup: [[u16; 3]; 13 * 9] = [
                        [240, 241, 242],
                        [245, 243, 244],
                        [246, 247, 248],
                        [251, 249, 250],
                        [252, 253, 254],
                        [257, 255, 256],
                        [258, 259, 260],
                        [263, 261, 262],
                        [264, 265, 266],
                        [269, 267, 268],
                        [342, 343, 344],
                        [347, 345, 346],
                        [350, 348, 349],
                        [150, 151, 152],
                        [155, 153, 154],
                        [156, 157, 158],
                        [161, 159, 160],
                        [162, 163, 164],
                        [167, 165, 166],
                        [168, 169, 170],
                        [173, 171, 172],
                        [174, 175, 176],
                        [179, 177, 178],
                        [315, 316, 317],
                        [320, 318, 319],
                        [323, 321, 322],
                        [120, 121, 122],
                        [125, 123, 124],
                        [126, 127, 128],
                        [131, 129, 130],
                        [132, 133, 134],
                        [137, 135, 136],
                        [138, 139, 140],
                        [143, 141, 142],
                        [144, 145, 146],
                        [149, 147, 148],
                        [306, 307, 308],
                        [311, 309, 310],
                        [314, 312, 313],
                        [90, 91, 92],
                        [95, 93, 94],
                        [96, 97, 98],
                        [101, 99, 100],
                        [102, 103, 104],
                        [107, 105, 106],
                        [108, 109, 110],
                        [113, 111, 112],
                        [114, 115, 116],
                        [119, 117, 118],
                        [297, 298, 299],
                        [302, 300, 301],
                        [305, 303, 304],
                        [60, 61, 62],
                        [65, 63, 64],
                        [66, 67, 68],
                        [71, 69, 70],
                        [72, 73, 74],
                        [77, 75, 76],
                        [78, 79, 80],
                        [83, 81, 82],
                        [84, 85, 86],
                        [89, 87, 88],
                        [288, 289, 290],
                        [293, 291, 292],
                        [296, 294, 295],
                        [30, 31, 32],
                        [35, 33, 34],
                        [36, 37, 38],
                        [41, 39, 40],
                        [42, 43, 44],
                        [47, 45, 46],
                        [48, 49, 50],
                        [53, 51, 52],
                        [54, 55, 56],
                        [59, 57, 58],
                        [279, 280, 281],
                        [284, 282, 283],
                        [287, 285, 286],
                        [0, 1, 2],
                        [5, 3, 4],
                        [6, 7, 8],
                        [11, 9, 10],
                        [12, 13, 14],
                        [17, 15, 16],
                        [18, 19, 20],
                        [23, 21, 22],
                        [24, 25, 26],
                        [29, 27, 28],
                        [270, 271, 272],
                        [275, 273, 274],
                        [278, 276, 277],
                        [210, 211, 212],
                        [215, 213, 214],
                        [216, 217, 218],
                        [221, 219, 220],
                        [222, 223, 224],
                        [227, 225, 226],
                        [228, 229, 230],
                        [233, 231, 232],
                        [234, 235, 236],
                        [239, 237, 238],
                        [333, 334, 335],
                        [338, 336, 337],
                        [341, 339, 340],
                        [180, 181, 182],
                        [185, 183, 184],
                        [186, 187, 188],
                        [191, 189, 190],
                        [192, 193, 194],
                        [197, 195, 196],
                        [198, 199, 200],
                        [203, 201, 202],
                        [204, 205, 206],
                        [209, 207, 208],
                        [324, 325, 326],
                        [329, 327, 328],
                        [332, 330, 331],
                    ];
                    let addr = lookup[x as usize][y as usize];
                    if addr < 180 {
                        (addr as u8, 0)
                    } else {
                        ((addr - 180) as u8, 1)
                    }
                },
            },
        }
    }

    pub fn pixel_rgb(&mut self, x: u8, y: u8, r: u8, g: u8, b: u8) -> Result<(), Error<I2cError>> {
        let x = x + y * 13;
        self.device.pixel(x, 2, r)?;
        self.device.pixel(x, 1, g)?;
        self.device.pixel(x, 0, b)?;
        Ok(())
    }

    pub fn setup<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
        self.device.setup(delay)
    }

    pub fn fill_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<(), Error<I2cError>> {
        for x in 0..13 {
            for y in 0..9 {
                self.pixel_rgb(x, y, r, g, b)?;
            }
        }
        Ok(())
    }
}
