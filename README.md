[![Crates.io](https://img.shields.io/crates/v/is31fl3741)](https://crates.io/crates/is31fl3741)
[![docs.rs](https://img.shields.io/docsrs/is31fl3741)](https://docs.rs/is31fl3741/latest/is31fl3741/)

[![lint](https://github.com/stillinbeta/is31fl3741/actions/workflows/lint.yml/badge.svg)](https://github.com/gleich/is31fl3741/actions/workflows/lint.yml)
[![build](https://github.com/stillinbeta/is31fl3741/actions/workflows/build.yml/badge.svg)](https://github.com/gleich/is31fl3741/actions/workflows/build.yml)


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
 To use a preconfigured device (currently just [Adafruit IS31FL3741 13x9 PWM RGB LED Matrix](https://www.adafruit.com/product/5201)), 
 you would need to change this line to include that device:

 ```toml
 is31fl3741 = { version = "0.1.0", features = ["adafruit13x9"] }
 ```

## Graphics

This driver contains optional support for the [embedded-graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/) library.
Enable the `embedded_graphics` feature to use it. 

The `gaypride` example shows off a use of this.

## Inspiration
 This driver is ~~ripped off~~ modified from [gleich](https://github.com/gleich/)'s [is31fl3731 crate](https://github.com/gleich/is31fl3731). 
 I was originally planning on just making a PR, but the driver ended up too differet.

 That driver is a port of [adafruit's driver for the is31fl3731](https://github.com/adafruit/Adafruit_CircuitPython_IS31FL3731) in the rust programming language.
