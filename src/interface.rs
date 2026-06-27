//! I²C bus interface for QMC6309
//!
//! Implements `RegisterInterface` for standard I²C communication.

use embedded_hal::i2c::I2c;

/// QMC6309 I²C base address (7-bit)
const QMC6309_I2C_ADDR: u8 = 0x1A;

/// QMC6309 communication error
#[derive(Debug)]
pub enum Qmc6309InterfaceError<BUS> {
    /// I²C bus error
    Bus(BUS),
}

#[cfg(feature = "defmt")]
impl<BUS: defmt::Format> defmt::Format for Qmc6309InterfaceError<BUS> {
    fn format(&self, f: defmt::Formatter) {
        match self {
            Qmc6309InterfaceError::Bus(e) => defmt::write!(f, "Bus({})", e),
        }
    }
}

impl<BUS> From<BUS> for Qmc6309InterfaceError<BUS> {
    fn from(e: BUS) -> Self {
        Qmc6309InterfaceError::Bus(e)
    }
}

/// QMC6309 I²C interface wrapper
///
/// Holds the I²C bus instance and implements the communication traits
/// required by device-driver.
#[derive(Debug)]
pub struct Qmc6309I2cInterface<BUS> {
    pub bus: BUS,
}

impl<BUS> Qmc6309I2cInterface<BUS> {
    /// Create a new QMC6309 I²C interface
    pub fn new(bus: BUS) -> Self {
        Self { bus }
    }

    /// Mutable reference to the underlying I²C bus
    pub fn bus_mut(&mut self) -> &mut BUS {
        &mut self.bus
    }

    /// Release the interface, returning bus ownership
    pub fn release(self) -> BUS {
        self.bus
    }
}

use device_driver::RegisterInterface;

impl<BUS> RegisterInterface for Qmc6309I2cInterface<BUS>
where
    BUS: I2c,
{
    type AddressType = u8;
    type Error = Qmc6309InterfaceError<BUS::Error>;

    fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        // QMC6309 I²C read: START → dev_addr(W) → reg_addr → RESTART → dev_addr(R) → data → STOP
        self.bus
            .write_read(QMC6309_I2C_ADDR, &[address], data)
            .map_err(Qmc6309InterfaceError::Bus)
    }

    fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut write_buf = [0u8; 17];
        write_buf[0] = address;
        let len = data.len().min(16);
        write_buf[1..=len].copy_from_slice(&data[..len]);

        self.bus
            .write(QMC6309_I2C_ADDR, &write_buf[..=len])
            .map_err(Qmc6309InterfaceError::Bus)
    }
}

// ==================== Async (optional) ====================

#[cfg(feature = "async")]
use device_driver::AsyncRegisterInterface;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

#[cfg(feature = "async")]
impl<BUS> AsyncRegisterInterface for Qmc6309I2cInterface<BUS>
where
    BUS: AsyncI2c,
{
    type AddressType = u8;
    type Error = Qmc6309InterfaceError<BUS::Error>;

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.bus
            .write_read(QMC6309_I2C_ADDR, &[address], data)
            .await
            .map_err(Qmc6309InterfaceError::Bus)
    }

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut write_buf = [0u8; 17];
        write_buf[0] = address;
        let len = data.len().min(16);
        write_buf[1..=len].copy_from_slice(&data[..len]);

        self.bus
            .write(QMC6309_I2C_ADDR, &write_buf[..=len])
            .await
            .map_err(Qmc6309InterfaceError::Bus)
    }
}

// ==================== Tests ====================

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as MockI2c, Transaction};

    #[test]
    fn test_read_chip_id() {
        let expectations = [Transaction::write_read(
            QMC6309_I2C_ADDR,
            std::vec![0x00],
            std::vec![0x90],
        )];
        let mock_i2c = MockI2c::new(&expectations);

        let mut interface = Qmc6309I2cInterface::new(mock_i2c);
        let mut data = [0u8; 1];

        interface.read_register(0x00, 8, &mut data).unwrap();
        assert_eq!(data[0], 0x90);

        interface.bus.done();
    }

    #[test]
    fn test_multi_byte_read() {
        let expectations = [Transaction::write_read(
            QMC6309_I2C_ADDR,
            std::vec![0x01],
            std::vec![0x34, 0x12],
        )];
        let mock_i2c = MockI2c::new(&expectations);

        let mut interface = Qmc6309I2cInterface::new(mock_i2c);
        let mut data = [0u8; 2];

        interface.read_register(0x01, 16, &mut data).unwrap();
        assert_eq!(data, [0x34, 0x12]);

        interface.bus.done();
    }

    #[test]
    fn test_write_register() {
        let expectations = [Transaction::write(QMC6309_I2C_ADDR, std::vec![0x0A, 0x03])];
        let mock_i2c = MockI2c::new(&expectations);

        let mut interface = Qmc6309I2cInterface::new(mock_i2c);
        interface.write_register(0x0A, 8, &[0x03]).unwrap();

        interface.bus.done();
    }
}
