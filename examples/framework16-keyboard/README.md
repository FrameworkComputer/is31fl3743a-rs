# Framework 16 RGB Keyboard/Macropad LED Example

Demonstrates controlling the IS31FL3743A LED controllers on the Framework 16 RGB Keyboard or Macropad.
Lights up the whole module in red, green, blue, white in sequence forever.

## Hardware Variants

### RGB Keyboard (default)
- Two IS31FL3743A controllers at I2C addresses 0x20 and 0x23
- SW1-SW9 active

### Macropad
- Single IS31FL3743A controller at I2C address 0x20
- SW1-SW4 active

## Prerequisites

Install the target and elf2uf2-rs tool:

```bash
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs
```

## Building

### For RGB Keyboard (default)

```bash
cd examples/framework16-keyboard
cargo build --features keyboard
```

### For Macropad

```bash
cd examples/framework16-keyboard
cargo build --features macropad
```

The ELF binary will be at `target/thumbv6m-none-eabi/debug/is31fl3743a-framework16-keyboard`.

## Converting to UF2

```bash
elf2uf2-rs target/thumbv6m-none-eabi/debug/is31fl3743a-framework16-keyboard firmware.uf2
```

## Flashing

1. Put the keyboard/macropad into bootloader mode
  a. On keyboard hold both alt keys down while powering it on
  b. On macropad hold both keys 1 and 6 (see numpad) down while powering it on
2. A USB drive named `RPI-RP2` will appear
3. Copy `firmware.uf2` to the drive
4. The device will automatically reboot and run the firmware

## One-step build and flash

If the device is already in bootloader mode:

```bash
cargo run --features keyboard
cargo run --features macropad
```

This uses elf2uf2-rs configured in `.cargo/config.toml` to automatically convert and flash.
