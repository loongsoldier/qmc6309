//! # QMC6309 Driver
//!
//! A `#![no_std]` Rust driver for the QMC6309 3-axis magnetic sensor over I²C.
//!
//! Three driver types correspond to the chip's three operating modes:
//!
//! | Type | Chip Mode | Core API |
//! |------|-----------|----------|
//! | [`Qmc6309`] | Normal | `read_gauss()` — continuous measurement |
//! | [`Qmc6309Shot`] | Single | `sample()` — one-shot then sleep |
//! | [`Qmc6309Diag`] | Continuous | `run_self_test()` — self-test diagnostics |
//!
//! ## Quick Start
//!
//! ```ignore
//! use qmc6309::{Config, Qmc6309, Qmc6309I2cInterface};
//!
//! let i2c = hal::I2c::new(...);
//! let interface = Qmc6309I2cInterface::new(i2c);
//! let mut sensor = Qmc6309::new(interface, Config::default())?;
//! let mag = sensor.read_gauss()?;
//! ```

#![no_std]
#[cfg(test)]
extern crate std;

mod driver;
mod interface;

#[allow(
    unused,
    clippy::redundant_field_names,
    clippy::identity_op,
    clippy::erasing_op,
    clippy::unnecessary_cast,
    clippy::new_without_default,
    clippy::let_and_return,
    clippy::unnecessary_fallible_conversions
)]
pub mod reg {
    include!("register.rs");
}

pub use driver::{
    Async, Blocking, Config, DefaultMode, Error, MagneticDataF32, Qmc6309, Qmc6309Diag,
    Qmc6309Shot, SelfTestData, ShotConfig,
};
pub use interface::{Qmc6309I2cInterface, Qmc6309InterfaceError};
