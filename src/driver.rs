//! QMC6309 human-ergonomic driver abstraction
//!
//! Three driver types correspond to the chip's three operating modes.
//! Default sync/async mode is selected automatically by the `async` feature:
//!
//! | Type | Chip Mode | Core API |
//! |------|-----------|----------|
//! | [`Qmc6309`] | Normal | `read_gauss()` — continuous measurement |
//! | [`Qmc6309Shot`] | Single | `sample()` — one-shot then sleep |
//! | [`Qmc6309Diag`] | Continuous | `run_self_test()` — self-test diagnostics |
//!
//! # Default Mode
//!
//! - Without `async` feature → `Qmc6309<I>` = `Qmc6309<I, Blocking>` (sync)
//! - With `async` feature → `Qmc6309<I>` = `Qmc6309<I, Async>` (async)
//!
//! ```ignore
//! // Default sync
//! let mut sensor = Qmc6309::new(interface, Config::default())?;
//! let mag = sensor.read_gauss()?;
//!
//! // Explicit async
//! let mut sensor = Qmc6309::<_, Async>::new(interface, Config::default())?;
//! ```

use core::marker::PhantomData;

use crate::reg::Qmc6309Reg;
use crate::reg::{Mode, Odr, Osr1, Osr2, Range, SetResetMode};
use device_driver::RegisterInterface;

// ==================== Mode Generics ====================

/// Sync/async mode marker
pub trait SensorMode: sealed::Sealed {}

/// Sync (blocking) mode — default
pub struct Blocking;
impl SensorMode for Blocking {}

/// Async (non-blocking) mode — requires `features = ["async"]`
pub struct Async;
impl SensorMode for Async {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Blocking {}
    impl Sealed for super::Async {}
}

/// Default mode selected automatically by feature
#[cfg(not(feature = "async"))]
pub type DefaultMode = Blocking;
#[cfg(feature = "async")]
pub type DefaultMode = Async;

// ==================== Error Type ====================

/// Driver-level error
#[derive(Debug)]
pub enum Error<BUS> {
    /// Bus error (I²C NACK, timeout, etc.)
    Bus(BUS),
    /// Chip ID mismatch (expected 0x90)
    InvalidChipId(u8),
}

#[cfg(feature = "defmt")]
impl<BUS: defmt::Format> defmt::Format for Error<BUS> {
    fn format(&self, f: defmt::Formatter) {
        match self {
            Error::Bus(e) => defmt::write!(f, "Bus({})", e),
            Error::InvalidChipId(id) => defmt::write!(f, "InvalidChipId({})", id),
        }
    }
}

impl<BUS> From<BUS> for Error<BUS> {
    fn from(e: BUS) -> Self {
        Error::Bus(e)
    }
}

// ==================== Configuration ====================

/// 持续测量Configuration（Normal / Continuous 模式共用）
/// Continuous measurement config (shared by Normal / Continuous modes)
///
/// `Config::default()` for recommended values: 200 Hz, ±8 G, max oversampling, set/reset both on.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    pub osr2: Osr2,
    pub osr1: Osr1,
    pub odr: Odr,
    pub range: Range,
    pub set_reset_mode: SetResetMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            osr2: Osr2::Depth16,
            osr1: Osr1::Ratio8,
            odr: Odr::Hz200,
            range: Range::G8,
            set_reset_mode: SetResetMode::SetAndResetOn,
        }
    }
}

/// One-shot config (Single mode only, no ODR needed)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ShotConfig {
    pub osr2: Osr2,
    pub osr1: Osr1,
    pub range: Range,
    pub set_reset_mode: SetResetMode,
}

impl Default for ShotConfig {
    fn default() -> Self {
        Self {
            osr2: Osr2::Depth16,
            osr1: Osr1::Ratio8,
            range: Range::G8,
            set_reset_mode: SetResetMode::SetAndResetOn,
        }
    }
}

// ==================== Data Containers ====================

/// 3-axis magnetic field in Gauss
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MagneticDataF32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Self-test data
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SelfTestData {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

// ==================== Range-to-Gauss Conversion ====================

const fn range_to_gauss(r: Range) -> f32 {
    match r {
        Range::G32 => 32.0,
        Range::G16 => 16.0,
        Range::G8 => 8.0,
        Range::CatchAll(_) => 32.0,
    }
}

// ————————————————————————————————————————————————
//  Qmc6309 — Qmc6309 — Normal mode: continuous measurement
// ————————————————————————————————————————————————

/// Standard continuous measurement driver (chip Normal mode)
///
/// 构造时自动：软复位 → 验证芯片 ID → 写入Configuration。
/// Continuously updates at ODR rate. `read_gauss()` reads the latest value at any time.
pub struct Qmc6309<I, M: SensorMode = DefaultMode> {
    device: Qmc6309Reg<I>,
    range: Range,
    _mode: PhantomData<M>,
}

impl<I> Qmc6309<I, Blocking>
where
    I: RegisterInterface<AddressType = u8>,
{
    pub fn new(interface: I, config: Config) -> Result<Self, (Error<I::Error>, I)> {
        let mut device = Qmc6309Reg::new(interface);
        if let Err(e) = soft_reset(&mut device) {
            return Err((e, device.interface));
        }
        if let Err(e) = verify_device(&mut device) {
            return Err((e, device.interface));
        }
        if let Err(e) = write_config(&mut device, &config, Mode::Normal) {
            return Err((e, device.interface));
        }
        Ok(Self {
            device,
            range: config.range,
            _mode: PhantomData,
        })
    }

    pub fn chip_id(&mut self) -> Result<u8, Error<I::Error>> {
        Ok(self.device.chip_id().read()?.chip_id())
    }

    /// Release the sensor, returning the underlying interface
    pub fn release(self) -> I {
        self.device.interface
    }

    pub fn read_gauss(&mut self) -> Result<MagneticDataF32, Error<I::Error>> {
        let (x, y, z) = read_axes(&mut self.device)?;
        let scale = range_to_gauss(self.range) / 32768.0;
        Ok(MagneticDataF32 {
            x: x as f32 * scale,
            y: y as f32 * scale,
            z: z as f32 * scale,
        })
    }
}

#[cfg(feature = "async")]
impl<I> Qmc6309<I, Async>
where
    I: device_driver::AsyncRegisterInterface<AddressType = u8>,
{
    pub async fn new(
        interface: I,
        config: Config,
    ) -> Result<
        Self,
        (
            Error<<I as device_driver::AsyncRegisterInterface>::Error>,
            I,
        ),
    > {
        let mut device = Qmc6309Reg::new(interface);
        if let Err(e) = soft_reset_async(&mut device).await {
            return Err((e, device.interface));
        }
        if let Err(e) = verify_device_async(&mut device).await {
            return Err((e, device.interface));
        }
        if let Err(e) = write_config_async(&mut device, &config, Mode::Normal).await {
            return Err((e, device.interface));
        }
        Ok(Self {
            device,
            range: config.range,
            _mode: PhantomData,
        })
    }

    pub async fn chip_id(
        &mut self,
    ) -> Result<u8, Error<<I as device_driver::AsyncRegisterInterface>::Error>> {
        Ok(self.device.chip_id().read_async().await?.chip_id())
    }

    pub fn release(self) -> I {
        self.device.interface
    }

    pub async fn read_gauss(
        &mut self,
    ) -> Result<MagneticDataF32, Error<<I as device_driver::AsyncRegisterInterface>::Error>> {
        let (x, y, z) = read_axes_async(&mut self.device).await?;
        let scale = range_to_gauss(self.range) / 32768.0;
        Ok(MagneticDataF32 {
            x: x as f32 * scale,
            y: y as f32 * scale,
            z: z as f32 * scale,
        })
    }
}

// ————————————————————————————————————————————————
//  Qmc6309Shot — Qmc6309Shot — Single mode: one-shot sampling
// ————————————————————————————————————————————————

/// One-shot sampling driver (chip Single mode)
///
/// `sample()` Triggers one measurement → waits DRDY → reads data → chip auto-returns to Suspend.
/// Ideal for low-frequency, low-power scenarios.
pub struct Qmc6309Shot<I, M: SensorMode = DefaultMode> {
    device: Qmc6309Reg<I>,
    config: ShotConfig,
    _mode: PhantomData<M>,
}

impl<I> Qmc6309Shot<I, Blocking>
where
    I: RegisterInterface<AddressType = u8>,
{
    pub fn new(interface: I, config: ShotConfig) -> Result<Self, (Error<I::Error>, I)> {
        let mut device = Qmc6309Reg::new(interface);
        if let Err(e) = soft_reset(&mut device) {
            return Err((e, device.interface));
        }
        if let Err(e) = verify_device(&mut device) {
            return Err((e, device.interface));
        }
        // Do not set Single mode here — triggered per sample() call
        Ok(Self {
            device,
            config,
            _mode: PhantomData,
        })
    }

    pub fn chip_id(&mut self) -> Result<u8, Error<I::Error>> {
        Ok(self.device.chip_id().read()?.chip_id())
    }

    pub fn release(self) -> I {
        self.device.interface
    }

    /// 触发单次测量并返回 Gauss 值
    ///
    /// Internally: write Single mode → wait DRDY → read → convert to Gauss.
    /// Chip auto-returns to Suspend after completion.
    pub fn sample(&mut self) -> Result<MagneticDataF32, Error<I::Error>> {
        write_shot_config(&mut self.device, &self.config)?;
        while !self.device.status_1().read()?.drdy() {
            core::hint::spin_loop();
        }
        let (x, y, z) = read_axes(&mut self.device)?;
        let scale = range_to_gauss(self.config.range) / 32768.0;
        Ok(MagneticDataF32 {
            x: x as f32 * scale,
            y: y as f32 * scale,
            z: z as f32 * scale,
        })
    }
}

#[cfg(feature = "async")]
impl<I> Qmc6309Shot<I, Async>
where
    I: device_driver::AsyncRegisterInterface<AddressType = u8>,
{
    pub async fn new(
        interface: I,
        config: ShotConfig,
    ) -> Result<
        Self,
        (
            Error<<I as device_driver::AsyncRegisterInterface>::Error>,
            I,
        ),
    > {
        let mut device = Qmc6309Reg::new(interface);
        if let Err(e) = soft_reset_async(&mut device).await {
            return Err((e, device.interface));
        }
        if let Err(e) = verify_device_async(&mut device).await {
            return Err((e, device.interface));
        }
        Ok(Self {
            device,
            config,
            _mode: PhantomData,
        })
    }

    pub async fn chip_id(
        &mut self,
    ) -> Result<u8, Error<<I as device_driver::AsyncRegisterInterface>::Error>> {
        Ok(self.device.chip_id().read_async().await?.chip_id())
    }

    pub fn release(self) -> I {
        self.device.interface
    }

    pub async fn sample(
        &mut self,
    ) -> Result<MagneticDataF32, Error<<I as device_driver::AsyncRegisterInterface>::Error>> {
        write_shot_config_async(&mut self.device, &self.config).await?;
        while !self.device.status_1().read_async().await?.drdy() {
            embassy_time::Timer::after_millis(1).await;
        }
        let (x, y, z) = read_axes_async(&mut self.device).await?;
        let scale = range_to_gauss(self.config.range) / 32768.0;
        Ok(MagneticDataF32 {
            x: x as f32 * scale,
            y: y as f32 * scale,
            z: z as f32 * scale,
        })
    }
}

// ————————————————————————————————————————————————
//  Qmc6309Diag — Continuous 模式：自测诊断
// ————————————————————————————————————————————————

/// Self-test diagnostic driver (chip Continuous mode)
///
/// Used only for self-test. Use [`Qmc6309`] for continuous measurement.
pub struct Qmc6309Diag<I, M: SensorMode = DefaultMode> {
    device: Qmc6309Reg<I>,
    _mode: PhantomData<M>,
}

impl<I> Qmc6309Diag<I, Blocking>
where
    I: RegisterInterface<AddressType = u8>,
{
    pub fn new(interface: I, config: Config) -> Result<Self, (Error<I::Error>, I)> {
        let mut device = Qmc6309Reg::new(interface);
        if let Err(e) = soft_reset(&mut device) {
            return Err((e, device.interface));
        }
        if let Err(e) = verify_device(&mut device) {
            return Err((e, device.interface));
        }
        if let Err(e) = write_config(&mut device, &config, Mode::Continuous) {
            return Err((e, device.interface));
        }
        Ok(Self {
            device,
            _mode: PhantomData,
        })
    }

    pub fn chip_id(&mut self) -> Result<u8, Error<I::Error>> {
        Ok(self.device.chip_id().read()?.chip_id())
    }

    pub fn release(self) -> I {
        self.device.interface
    }

    /// Full self-test flow: enable → wait → read
    pub fn run_self_test(&mut self) -> Result<SelfTestData, Error<I::Error>> {
        self.device
            .control()
            .reg_3()
            .modify(|r| r.set_self_test(true))?;
        while !self.device.status_1().read()?.st_rdy() {
            core::hint::spin_loop();
        }
        let x = self.device.self_test().st_x().read()?.value();
        let y = self.device.self_test().st_y().read()?.value();
        let z = self.device.self_test().st_z().read()?.value();
        Ok(SelfTestData { x, y, z })
    }
}

#[cfg(feature = "async")]
impl<I> Qmc6309Diag<I, Async>
where
    I: device_driver::AsyncRegisterInterface<AddressType = u8>,
{
    pub async fn new(
        interface: I,
        config: Config,
    ) -> Result<
        Self,
        (
            Error<<I as device_driver::AsyncRegisterInterface>::Error>,
            I,
        ),
    > {
        let mut device = Qmc6309Reg::new(interface);
        if let Err(e) = soft_reset_async(&mut device).await {
            return Err((e, device.interface));
        }
        if let Err(e) = verify_device_async(&mut device).await {
            return Err((e, device.interface));
        }
        if let Err(e) = write_config_async(&mut device, &config, Mode::Continuous).await {
            return Err((e, device.interface));
        }
        Ok(Self {
            device,
            _mode: PhantomData,
        })
    }

    pub async fn chip_id(
        &mut self,
    ) -> Result<u8, Error<<I as device_driver::AsyncRegisterInterface>::Error>> {
        Ok(self.device.chip_id().read_async().await?.chip_id())
    }

    pub fn release(self) -> I {
        self.device.interface
    }

    pub async fn run_self_test(
        &mut self,
    ) -> Result<SelfTestData, Error<<I as device_driver::AsyncRegisterInterface>::Error>> {
        self.device
            .control()
            .reg_3()
            .modify_async(|r| r.set_self_test(true))
            .await?;
        while !self.device.status_1().read_async().await?.st_rdy() {
            embassy_time::Timer::after_millis(1).await;
        }
        let x = self.device.self_test().st_x().read_async().await?.value();
        let y = self.device.self_test().st_y().read_async().await?.value();
        let z = self.device.self_test().st_z().read_async().await?.value();
        Ok(SelfTestData { x, y, z })
    }
}

// ==================== Internal helpers ====================

fn read_axes<I: RegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
) -> Result<(i16, i16, i16), Error<I::Error>> {
    let x = device.data().data_x().read()?.value();
    let y = device.data().data_y().read()?.value();
    let z = device.data().data_z().read()?.value();
    Ok((x, y, z))
}

#[cfg(feature = "async")]
async fn read_axes_async<I: device_driver::AsyncRegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
) -> Result<(i16, i16, i16), Error<I::Error>> {
    let x = device.data().data_x().read_async().await?.value();
    let y = device.data().data_y().read_async().await?.value();
    let z = device.data().data_z().read_async().await?.value();
    Ok((x, y, z))
}

fn soft_reset<I: RegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
) -> Result<(), Error<I::Error>> {
    device.control().reg_2().write(|r| r.set_soft_rst(true))?;
    device.control().reg_2().write(|r| r.set_soft_rst(false))?;
    Ok(())
}

#[cfg(feature = "async")]
async fn soft_reset_async<I: device_driver::AsyncRegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
) -> Result<(), Error<I::Error>> {
    device
        .control()
        .reg_2()
        .write_async(|r| r.set_soft_rst(true))
        .await?;
    device
        .control()
        .reg_2()
        .write_async(|r| r.set_soft_rst(false))
        .await?;
    Ok(())
}

fn verify_device<I: RegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
) -> Result<(), Error<I::Error>> {
    let id = device.chip_id().read()?.chip_id();
    if id != 0x90 {
        return Err(Error::InvalidChipId(id));
    }
    Ok(())
}

#[cfg(feature = "async")]
async fn verify_device_async<I: device_driver::AsyncRegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
) -> Result<(), Error<I::Error>> {
    let id = device.chip_id().read_async().await?.chip_id();
    if id != 0x90 {
        return Err(Error::InvalidChipId(id));
    }
    Ok(())
}

fn write_config<I: RegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
    cfg: &Config,
    mode: Mode,
) -> Result<(), Error<I::Error>> {
    device.control().reg_1().modify(|r| {
        r.set_mode(mode);
        r.set_osr_2(cfg.osr2);
        r.set_osr_1(cfg.osr1);
    })?;
    device.control().reg_2().modify(|r| {
        r.set_odr(cfg.odr);
        r.set_rng(cfg.range);
        r.set_set_reset_mode(cfg.set_reset_mode);
    })?;
    Ok(())
}

#[cfg(feature = "async")]
async fn write_config_async<I: device_driver::AsyncRegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
    cfg: &Config,
    mode: Mode,
) -> Result<(), Error<I::Error>> {
    device
        .control()
        .reg_1()
        .modify_async(|r| {
            r.set_mode(mode);
            r.set_osr_2(cfg.osr2);
            r.set_osr_1(cfg.osr1);
        })
        .await?;
    device
        .control()
        .reg_2()
        .modify_async(|r| {
            r.set_odr(cfg.odr);
            r.set_rng(cfg.range);
            r.set_set_reset_mode(cfg.set_reset_mode);
        })
        .await?;
    Ok(())
}

fn write_shot_config<I: RegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
    cfg: &ShotConfig,
) -> Result<(), Error<I::Error>> {
    device.control().reg_1().modify(|r| {
        r.set_mode(Mode::Single);
        r.set_osr_2(cfg.osr2);
        r.set_osr_1(cfg.osr1);
    })?;
    device.control().reg_2().modify(|r| {
        r.set_rng(cfg.range);
        r.set_set_reset_mode(cfg.set_reset_mode);
    })?;
    Ok(())
}

#[cfg(feature = "async")]
async fn write_shot_config_async<I: device_driver::AsyncRegisterInterface<AddressType = u8>>(
    device: &mut Qmc6309Reg<I>,
    cfg: &ShotConfig,
) -> Result<(), Error<I::Error>> {
    device
        .control()
        .reg_1()
        .modify_async(|r| {
            r.set_mode(Mode::Single);
            r.set_osr_2(cfg.osr2);
            r.set_osr_1(cfg.osr1);
        })
        .await?;
    device
        .control()
        .reg_2()
        .modify_async(|r| {
            r.set_rng(cfg.range);
            r.set_set_reset_mode(cfg.set_reset_mode);
        })
        .await?;
    Ok(())
}

// ==================== Tests ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reg::Range;

    #[test]
    fn test_range_to_gauss() {
        assert_eq!(range_to_gauss(Range::G32), 32.0);
        assert_eq!(range_to_gauss(Range::G16), 16.0);
        assert_eq!(range_to_gauss(Range::G8), 8.0);
        assert_eq!(range_to_gauss(Range::CatchAll(3)), 32.0);
    }

    #[test]
    fn test_config_defaults() {
        let c = Config::default();
        assert!(matches!(c.osr2, Osr2::Depth16));
        assert!(matches!(c.odr, Odr::Hz200));
        assert!(matches!(c.range, Range::G8));

        let sc = ShotConfig::default();
        assert!(matches!(sc.range, Range::G8));
    }

    #[test]
    fn test_gauss_conversion() {
        let scale: f32 = 8.0 / 32768.0;
        assert!((16384.0 * scale - 4.0).abs() < 0.01);
        assert!((-16384.0 * scale + 4.0).abs() < 0.01);
    }
}
