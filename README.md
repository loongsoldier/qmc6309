# QMC6309 Driver

[![crates.io](https://img.shields.io/crates/v/qmc6309-driver.svg)](https://crates.io/crates/qmc6309-driver)
[![docs.rs](https://docs.rs/qmc6309-driver/badge.svg)](https://docs.rs/qmc6309-driver)

A platform-agnostic, `#![no_std]` Rust driver for the QMC6309 3-axis magnetic sensor over I²C.

## Features

- Three driver types matched to the chip's operating modes:
  - **`Qmc6309`** — Normal mode, continuous measurement
  - **`Qmc6309Shot`** — Single mode, one-shot sampling
  - **`Qmc6309Diag`** — Continuous mode, self-test diagnostics
- Sync (`Blocking`) and async (`Async`) via embassy-style Mode generics
- Physical unit conversion: raw ADC → Gauss
- Optional `defmt` support

## Quick Start

```rust
use qmc6309_driver::prelude::*;

let i2c = hal::I2c::new(...);
let interface = Qmc6309I2cInterface::new(i2c);
let mut sensor = Qmc6309::new(interface, Config::default())?;
let mag = sensor.read_gauss()?;
// mag.x, mag.y, mag.z in Gauss
```

## Cargo Features

| Feature | Enables |
|---------|---------|
| `async` | Async methods + `embassy-time` for non-blocking waits |
| `defmt-log` | `defmt::Format` on generated types |
| `full` | `async` + `defmt-log` |

## Development

```bash
# Re-generate register code from DSL
just generate

# Build all feature combinations
just build

# Run tests
just test
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
