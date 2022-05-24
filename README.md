# is31fl3741 driver

 Driver for [Lumissil Microsystem's IS31FL3741 integrated circuit](https://www.lumissil.com/assets/pdf/core/IS31FL3741_DS.pdf). Some of the major features of this library are:

 1. Use of embedded HAL traits (works with any embedded device that supports the required traits). This means that this driver is platform agnostic.
 2. Library features (only turn on what devices you need to save compiled binary space).
 3. [Examples](./examples) on how to use this driver. 
 Right now there is only an example on how to use this crate with a stm32 nucleo. 

## Install

 To install this driver in your project add the following line to your `Cargo.toml`'s `dependencies` table:

 ```toml
 is31fl3741 = "0.1.0"
 ```

 By default this version will only contain the core driver. 
 To use a preconfigured device, such as the [Adafruit CharliePlex LED Matrix Bonnet](https://www.adafruit.com/product/3467), 
 you would need to change this line to include that device:

 ```toml
 is31fl3741 = { version = "0.1.0", features = ["adafruit13x9"] }
 ```

 ## Inspiration
 This driver is ~~ripped off from~~ heavily based on [gleich](https://github.com/gleich/)'s [is31fl3731 crate](https://github.com/gleich/is31fl3731). 
 I was originally planning on just making a PR, but the driver ended up too differet.

 That driver is a port of [adafruit's driver for the is31fl3731](https://github.com/adafruit/Adafruit_CircuitPython_IS31FL3731) in the rust programming language.
