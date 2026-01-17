[![Crates.io](https://img.shields.io/crates/v/is31fl3743a)](https://crates.io/crates/is31fl3743a)
[![docs.rs](https://img.shields.io/docsrs/is31fl3743a)](https://docs.rs/is31fl3743a/latest/is31fl3743a/)

[![lint](https://github.com/FrameworkComputer/is31fl3743a-rs/actions/workflows/lint.yml/badge.svg)](https://github.com/FrameworkComputer/is31fl3743a-rs/actions/workflows/lint.yml)
[![build](https://github.com/FrameworkComputer/is31fl3743a-rs/actions/workflows/build.yml/badge.svg)](https://github.com/FrameworkComputer/is31fl3743a-rs/actions/workflows/build.yml)


# is31fl3743a driver

Driver for [Lumissil Microsystem's IS31FL3743A integrated circuit](https://www.lumissil.com/assets/pdf/core/IS31FL3743A_DS.pdf). Some of the major features of this library are:

1. Use of embedded HAL traits (works with any embedded device that supports the required traits). This means that this driver is platform agnostic.
2. Library features (only turn on what devices you need to save compiled binary space).
3. [Examples](./examples) for various RP2040 boards

## Install

To install this driver in your project add the following line to your `Cargo.toml`'s `dependencies` table:

```toml
is31fl3743a = "0.1.0"
```

## Examples

See the [examples](./examples) directory for complete working examples:

- **[qt-py-rp2040](./examples/qt-py-rp2040)** - Adafruit QT Py RP2040 with IS31FL3743A over STEMMA QT
- **[framework16-keyboard](./examples/framework16-keyboard)** - Framework 16 RGB Keyboard (dual IS31FL3743A controllers)

### Building Examples

```bash
# Install prerequisites
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs

# Build an example
cd examples/framework16-keyboard
cargo build --release

# Convert to UF2
elf2uf2-rs target/thumbv6m-none-eabi/release/is31fl3743a-framework16-keyboard firmware.uf2
```

## Graphics

This driver contains optional support for the [embedded-graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/) library.
Enable the `embedded_graphics` feature to use it.

## References

Contains code derived from:

- https://github.com/FrameworkComputer/is31fl3741-rs
- https://github.com/stillinbeta/is31fl3741
- https://github.com/gleich/is31fl3731

The 43A chip is the I2C variant, the 43B chip is the SPI variant of the same chip.
For the SPI variant of the device, consider using the `is31fl3741b` crate instead.
