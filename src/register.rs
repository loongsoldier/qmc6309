/// Root block of the Qmc6309Reg driver
#[derive(Debug)]
pub struct Qmc6309Reg<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u8,
}
impl<I> Qmc6309Reg<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self { interface, base_address: 0 }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.chip_id().read()?;
        callback(0 + 0 * 0, "chip_id", reg.into());
        let reg = self.status_1().read()?;
        callback(9 + 0 * 0, "status_1", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.chip_id().read_async().await?;
        callback(0 + 0 * 0, "chip_id", reg.into());
        let reg = self.status_1().read_async().await?;
        callback(9 + 0 * 0, "status_1", reg.into());
        Ok(())
    }
    ///  Chip ID register — POR value 0x90 used to verify I²C communication
    pub fn chip_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::ChipId,
        ::device_driver::RO,
    > {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::ChipId,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::ChipId::new)
    }
    pub fn data(&mut self) -> Data<'_, I> {
        let address = self.base_address + 1;
        Data::<'_, I>::new(self.interface(), address)
    }
    ///  Status Register 1 (09H)
    ///
    ///  DRDY and OVFL are cleared automatically after reading this register.
    pub fn status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Status1,
        ::device_driver::RO,
    > {
        let address = self.base_address + 9;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Status1,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::Status1::new)
    }
    pub fn control(&mut self) -> Control<'_, I> {
        let address = self.base_address + 10;
        Control::<'_, I>::new(self.interface(), address)
    }
    pub fn self_test(&mut self) -> SelfTest<'_, I> {
        let address = self.base_address + 19;
        SelfTest::<'_, I>::new(self.interface(), address)
    }
}
#[derive(Debug)]
pub struct Data<'i, I> {
    pub(crate) interface: &'i mut I,
    #[doc(hidden)]
    base_address: u8,
}
impl<'i, I> Data<'i, I> {
    /// Create a new instance of the block based on device interface
    #[doc(hidden)]
    fn new(interface: &'i mut I, base_address: u8) -> Self {
        Self {
            interface,
            base_address: base_address,
        }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.axis_data().read()?;
        callback(0 + 0 * 0, "axis_data", reg.into());
        let reg = self.data_x().read()?;
        callback(0 + 0 * 0, "data_x", reg.into());
        let reg = self.data_y().read()?;
        callback(2 + 0 * 0, "data_y", reg.into());
        let reg = self.data_z().read()?;
        callback(4 + 0 * 0, "data_z", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.axis_data().read_async().await?;
        callback(0 + 0 * 0, "axis_data", reg.into());
        let reg = self.data_x().read_async().await?;
        callback(0 + 0 * 0, "data_x", reg.into());
        let reg = self.data_y().read_async().await?;
        callback(2 + 0 * 0, "data_y", reg.into());
        let reg = self.data_z().read_async().await?;
        callback(4 + 0 * 0, "data_z", reg.into());
        Ok(())
    }
    ///  16-bit signed axis data (two's complement, saturates at ±32768)
    ///  Template register — use DataX / DataY / DataZ for actual access
    pub fn axis_data(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AxisData,
        ::device_driver::RO,
    > {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AxisData,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::AxisData::new)
    }
    ///  X-axis @ 01H (LSB) + 02H (MSB)
    pub fn data_x(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AxisData,
        ::device_driver::RO,
    > {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AxisData,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::AxisData::new)
    }
    ///  Y-axis @ 03H (LSB) + 04H (MSB)
    pub fn data_y(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AxisData,
        ::device_driver::RO,
    > {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AxisData,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::AxisData::new)
    }
    ///  Z-axis @ 05H (LSB) + 06H (MSB)
    pub fn data_z(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::AxisData,
        ::device_driver::RO,
    > {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::AxisData,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::AxisData::new)
    }
}
#[derive(Debug)]
pub struct Control<'i, I> {
    pub(crate) interface: &'i mut I,
    #[doc(hidden)]
    base_address: u8,
}
impl<'i, I> Control<'i, I> {
    /// Create a new instance of the block based on device interface
    #[doc(hidden)]
    fn new(interface: &'i mut I, base_address: u8) -> Self {
        Self {
            interface,
            base_address: base_address,
        }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.reg_1().read()?;
        callback(0 + 0 * 0, "reg_1", reg.into());
        let reg = self.reg_2().read()?;
        callback(1 + 0 * 0, "reg_2", reg.into());
        let reg = self.reg_3().read()?;
        callback(4 + 0 * 0, "reg_3", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.reg_1().read_async().await?;
        callback(0 + 0 * 0, "reg_1", reg.into());
        let reg = self.reg_2().read_async().await?;
        callback(1 + 0 * 0, "reg_2", reg.into());
        let reg = self.reg_3().read_async().await?;
        callback(4 + 0 * 0, "reg_3", reg.into());
        Ok(())
    }
    ///  Control Register 1 (0AH) — operating mode and oversampling ratio
    pub fn reg_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Reg1,
        ::device_driver::RW,
    > {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Reg1,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Reg1::new)
    }
    ///  Control Register 2 (0BH) — soft reset, data rate, range, set/reset
    pub fn reg_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Reg2,
        ::device_driver::RW,
    > {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Reg2,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Reg2::new)
    }
    ///  Control Register 3 (0EH) — self-test enable
    ///  Only valid in Continuous mode. Hardware clears automatically when done.
    pub fn reg_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Reg3,
        ::device_driver::RW,
    > {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Reg3,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Reg3::new)
    }
}
#[derive(Debug)]
pub struct SelfTest<'i, I> {
    pub(crate) interface: &'i mut I,
    #[doc(hidden)]
    base_address: u8,
}
impl<'i, I> SelfTest<'i, I> {
    /// Create a new instance of the block based on device interface
    #[doc(hidden)]
    fn new(interface: &'i mut I, base_address: u8) -> Self {
        Self {
            interface,
            base_address: base_address,
        }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.st_data().read()?;
        callback(0 + 0 * 0, "st_data", reg.into());
        let reg = self.st_x().read()?;
        callback(0 + 0 * 0, "st_x", reg.into());
        let reg = self.st_y().read()?;
        callback(1 + 0 * 0, "st_y", reg.into());
        let reg = self.st_z().read()?;
        callback(2 + 0 * 0, "st_z", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.st_data().read_async().await?;
        callback(0 + 0 * 0, "st_data", reg.into());
        let reg = self.st_x().read_async().await?;
        callback(0 + 0 * 0, "st_x", reg.into());
        let reg = self.st_y().read_async().await?;
        callback(1 + 0 * 0, "st_y", reg.into());
        let reg = self.st_z().read_async().await?;
        callback(2 + 0 * 0, "st_z", reg.into());
        Ok(())
    }
    ///  Self-test data (8-bit) template
    pub fn st_data(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::StData,
        ::device_driver::RO,
    > {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::StData,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::StData::new)
    }
    ///  X-axis self-test @ 13H
    pub fn st_x(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::StData,
        ::device_driver::RO,
    > {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::StData,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::StData::new)
    }
    ///  Y-axis self-test @ 14H
    pub fn st_y(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::StData,
        ::device_driver::RO,
    > {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::StData,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::StData::new)
    }
    ///  Z-axis self-test @ 15H
    pub fn st_z(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::StData,
        ::device_driver::RO,
    > {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::StData,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::StData::new)
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    #[allow(unused_imports)]
    use super::*;
    ///  Chip ID register — POR value 0x90 used to verify I²C communication
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChipId {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChipId {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChipId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [144] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `chip_id` field of the register.
        ///
        ///  Chip ID: fixed to 0x90 for QMC6309
        pub fn chip_id(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `chip_id` field of the register.
        ///
        ///  Chip ID: fixed to 0x90 for QMC6309
        pub fn set_chip_id(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for ChipId {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChipId> for [u8; 1] {
        fn from(val: ChipId) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChipId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChipId");
            d.field("chip_id", &self.chip_id());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChipId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChipId {{ ");
            defmt::write!(f, "chip_id: {=u8}, ", & self.chip_id());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChipId {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChipId {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChipId {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChipId {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChipId {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChipId {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChipId {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///  16-bit signed axis data (two's complement, saturates at ±32768)
    ///  Template register — use DataX / DataY / DataZ for actual access
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AxisData {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AxisData {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AxisData {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `value` field of the register.
        ///
        ///  Raw magnetic sensor output, signed 16-bit
        pub fn value(&self) -> i16 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    i16,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 16)
            };
            raw
        }
        ///Write the `value` field of the register.
        ///
        ///  Raw magnetic sensor output, signed 16-bit
        pub fn set_value(&mut self, value: i16) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    i16,
                    ::device_driver::ops::LE,
                >(raw, 0, 16, &mut self.bits)
            };
        }
    }
    impl From<[u8; 2]> for AxisData {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AxisData> for [u8; 2] {
        fn from(val: AxisData) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AxisData {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AxisData");
            d.field("value", &self.value());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AxisData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AxisData {{ ");
            defmt::write!(f, "value: {=i16}, ", & self.value());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AxisData {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AxisData {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AxisData {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AxisData {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AxisData {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AxisData {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AxisData {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///  Status Register 1 (09H)
    ///
    ///  DRDY and OVFL are cleared automatically after reading this register.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Status1 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Status1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [24] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `nvm_load_done` field of the register.
        ///
        ///  NVM data load complete
        pub fn nvm_load_done(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 4, 5)
            };
            raw > 0
        }
        ///Read the `nvm_rdy` field of the register.
        ///
        ///  NVM ready (1: non-volatile memory accessible)
        pub fn nvm_rdy(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 3, 4)
            };
            raw > 0
        }
        ///Read the `st_rdy` field of the register.
        ///
        ///  Self-test complete (1: self-test done, data readable)
        pub fn st_rdy(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 3)
            };
            raw > 0
        }
        ///Read the `ovfl` field of the register.
        ///
        ///  Data overflow (1: any axis output exceeds ±32000 LSB)
        pub fn ovfl(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 1, 2)
            };
            raw > 0
        }
        ///Read the `drdy` field of the register.
        ///
        ///  Data ready (1: new 3-axis data available, cleared on read)
        pub fn drdy(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `nvm_load_done` field of the register.
        ///
        ///  NVM data load complete
        pub fn set_nvm_load_done(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 4, 5, &mut self.bits)
            };
        }
        ///Write the `nvm_rdy` field of the register.
        ///
        ///  NVM ready (1: non-volatile memory accessible)
        pub fn set_nvm_rdy(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 3, 4, &mut self.bits)
            };
        }
        ///Write the `st_rdy` field of the register.
        ///
        ///  Self-test complete (1: self-test done, data readable)
        pub fn set_st_rdy(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 3, &mut self.bits)
            };
        }
        ///Write the `ovfl` field of the register.
        ///
        ///  Data overflow (1: any axis output exceeds ±32000 LSB)
        pub fn set_ovfl(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 1, 2, &mut self.bits)
            };
        }
        ///Write the `drdy` field of the register.
        ///
        ///  Data ready (1: new 3-axis data available, cleared on read)
        pub fn set_drdy(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Status1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Status1> for [u8; 1] {
        fn from(val: Status1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Status1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Status1");
            d.field("nvm_load_done", &self.nvm_load_done());
            d.field("nvm_rdy", &self.nvm_rdy());
            d.field("st_rdy", &self.st_rdy());
            d.field("ovfl", &self.ovfl());
            d.field("drdy", &self.drdy());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Status1 {{ ");
            defmt::write!(f, "nvm_load_done: {=bool}, ", & self.nvm_load_done());
            defmt::write!(f, "nvm_rdy: {=bool}, ", & self.nvm_rdy());
            defmt::write!(f, "st_rdy: {=bool}, ", & self.st_rdy());
            defmt::write!(f, "ovfl: {=bool}, ", & self.ovfl());
            defmt::write!(f, "drdy: {=bool}, ", & self.drdy());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for Status1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Status1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Status1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Status1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Status1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Status1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Status1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///  Control Register 1 (0AH) — operating mode and oversampling ratio
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reg1 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Reg1 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Reg1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `osr_2` field of the register.
        ///
        ///  Low-pass filter depth (higher = lower noise, higher power)
        pub fn osr_2(&self) -> super::Osr2 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 5, 8)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `osr_1` field of the register.
        ///
        ///  Oversampling ratio
        pub fn osr_1(&self) -> super::Osr1 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 3, 5)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `mode` field of the register.
        ///
        ///  Operating mode (pass through Suspend when switching)
        pub fn mode(&self) -> super::Mode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `osr_2` field of the register.
        ///
        ///  Low-pass filter depth (higher = lower noise, higher power)
        pub fn set_osr_2(&mut self, value: super::Osr2) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 5, 8, &mut self.bits)
            };
        }
        ///Write the `osr_1` field of the register.
        ///
        ///  Oversampling ratio
        pub fn set_osr_1(&mut self, value: super::Osr1) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 3, 5, &mut self.bits)
            };
        }
        ///Write the `mode` field of the register.
        ///
        ///  Operating mode (pass through Suspend when switching)
        pub fn set_mode(&mut self, value: super::Mode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Reg1 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Reg1> for [u8; 1] {
        fn from(val: Reg1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Reg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Reg1");
            d.field("osr_2", &self.osr_2());
            d.field("osr_1", &self.osr_1());
            d.field("mode", &self.mode());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Reg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Reg1 {{ ");
            defmt::write!(f, "osr_2: {}, ", & self.osr_2());
            defmt::write!(f, "osr_1: {}, ", & self.osr_1());
            defmt::write!(f, "mode: {}, ", & self.mode());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for Reg1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Reg1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Reg1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Reg1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Reg1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Reg1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Reg1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///  Control Register 2 (0BH) — soft reset, data rate, range, set/reset
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reg2 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Reg2 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Reg2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `soft_rst` field of the register.
        ///
        ///  Soft reset (set to 1 to trigger, must manually write 0 to clear)
        pub fn soft_rst(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 7, 8)
            };
            raw > 0
        }
        ///Read the `odr` field of the register.
        ///
        ///  Output data rate
        pub fn odr(&self) -> super::Odr {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 4, 7)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `rng` field of the register.
        ///
        ///  Full-scale range (Gauss). Lower range → higher sensitivity → higher resolution
        pub fn rng(&self) -> super::Range {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 2, 4)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `set_reset_mode` field of the register.
        ///
        ///  Set/reset mode — controls whether offset is updated during measurement
        pub fn set_reset_mode(&self) -> super::SetResetMode {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 2)
            };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `soft_rst` field of the register.
        ///
        ///  Soft reset (set to 1 to trigger, must manually write 0 to clear)
        pub fn set_soft_rst(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 7, 8, &mut self.bits)
            };
        }
        ///Write the `odr` field of the register.
        ///
        ///  Output data rate
        pub fn set_odr(&mut self, value: super::Odr) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 4, 7, &mut self.bits)
            };
        }
        ///Write the `rng` field of the register.
        ///
        ///  Full-scale range (Gauss). Lower range → higher sensitivity → higher resolution
        pub fn set_rng(&mut self, value: super::Range) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 2, 4, &mut self.bits)
            };
        }
        ///Write the `set_reset_mode` field of the register.
        ///
        ///  Set/reset mode — controls whether offset is updated during measurement
        pub fn set_set_reset_mode(&mut self, value: super::SetResetMode) {
            let raw = value.into();
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 2, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Reg2 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Reg2> for [u8; 1] {
        fn from(val: Reg2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Reg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Reg2");
            d.field("soft_rst", &self.soft_rst());
            d.field("odr", &self.odr());
            d.field("rng", &self.rng());
            d.field("set_reset_mode", &self.set_reset_mode());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Reg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Reg2 {{ ");
            defmt::write!(f, "soft_rst: {=bool}, ", & self.soft_rst());
            defmt::write!(f, "odr: {}, ", & self.odr());
            defmt::write!(f, "rng: {}, ", & self.rng());
            defmt::write!(f, "set_reset_mode: {}, ", & self.set_reset_mode());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for Reg2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Reg2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Reg2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Reg2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Reg2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Reg2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Reg2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///  Control Register 3 (0EH) — self-test enable
    ///  Only valid in Continuous mode. Hardware clears automatically when done.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reg3 {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Reg3 {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Reg3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `self_test` field of the register.
        ///
        ///  Self-test enable (1: start self-test)
        pub fn self_test(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Write the `self_test` field of the register.
        ///
        ///  Self-test enable (1: start self-test)
        pub fn set_self_test(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Reg3 {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Reg3> for [u8; 1] {
        fn from(val: Reg3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Reg3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Reg3");
            d.field("self_test", &self.self_test());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Reg3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Reg3 {{ ");
            defmt::write!(f, "self_test: {=bool}, ", & self.self_test());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for Reg3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Reg3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Reg3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Reg3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Reg3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Reg3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Reg3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    ///  Self-test data (8-bit) template
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StData {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for StData {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl StData {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `value` field of the register.
        ///
        ///  Self-test output value
        pub fn value(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `value` field of the register.
        ///
        ///  Self-test output value
        pub fn set_value(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for StData {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<StData> for [u8; 1] {
        fn from(val: StData) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for StData {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("StData");
            d.field("value", &self.value());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for StData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "StData {{ ");
            defmt::write!(f, "value: {=u8}, ", & self.value());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for StData {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for StData {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for StData {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for StData {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for StData {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for StData {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for StData {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        ///  Chip ID register — POR value 0x90 used to verify I²C communication
        ChipId(ChipId),
        ///  16-bit signed axis data (two's complement, saturates at ±32768)
        ///  Template register — use DataX / DataY / DataZ for actual access
        AxisData(AxisData),
        ///  Status Register 1 (09H)
        ///
        ///  DRDY and OVFL are cleared automatically after reading this register.
        Status1(Status1),
        ///  Control Register 1 (0AH) — operating mode and oversampling ratio
        Reg1(Reg1),
        ///  Control Register 2 (0BH) — soft reset, data rate, range, set/reset
        Reg2(Reg2),
        ///  Control Register 3 (0EH) — self-test enable
        ///  Only valid in Continuous mode. Hardware clears automatically when done.
        Reg3(Reg3),
        ///  Self-test data (8-bit) template
        StData(StData),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::ChipId(val) => core::fmt::Debug::fmt(val, f),
                Self::AxisData(val) => core::fmt::Debug::fmt(val, f),
                Self::Status1(val) => core::fmt::Debug::fmt(val, f),
                Self::Reg1(val) => core::fmt::Debug::fmt(val, f),
                Self::Reg2(val) => core::fmt::Debug::fmt(val, f),
                Self::Reg3(val) => core::fmt::Debug::fmt(val, f),
                Self::StData(val) => core::fmt::Debug::fmt(val, f),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::ChipId(val) => defmt::Format::format(val, f),
                Self::AxisData(val) => defmt::Format::format(val, f),
                Self::Status1(val) => defmt::Format::format(val, f),
                Self::Reg1(val) => defmt::Format::format(val, f),
                Self::Reg2(val) => defmt::Format::format(val, f),
                Self::Reg3(val) => defmt::Format::format(val, f),
                Self::StData(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<ChipId> for FieldSetValue {
        fn from(val: ChipId) -> Self {
            Self::ChipId(val)
        }
    }
    impl From<AxisData> for FieldSetValue {
        fn from(val: AxisData) -> Self {
            Self::AxisData(val)
        }
    }
    impl From<Status1> for FieldSetValue {
        fn from(val: Status1) -> Self {
            Self::Status1(val)
        }
    }
    impl From<Reg1> for FieldSetValue {
        fn from(val: Reg1) -> Self {
            Self::Reg1(val)
        }
    }
    impl From<Reg2> for FieldSetValue {
        fn from(val: Reg2) -> Self {
            Self::Reg2(val)
        }
    }
    impl From<Reg3> for FieldSetValue {
        fn from(val: Reg3) -> Self {
            Self::Reg3(val)
        }
    }
    impl From<StData> for FieldSetValue {
        fn from(val: StData) -> Self {
            Self::StData(val)
        }
    }
}
///  Low-pass filter depth (higher = lower noise, higher power)
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osr2 {
    Depth1 = 0,
    Depth2 = 1,
    Depth4 = 2,
    Depth8 = 3,
    Depth16 = 4,
    ///  Reserved values (5~7), hardware equivalent to Depth16
    CatchAll(u8) = 5,
}
impl From<u8> for Osr2 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Depth1,
            1 => Self::Depth2,
            2 => Self::Depth4,
            3 => Self::Depth8,
            4 => Self::Depth16,
            val => Self::CatchAll(val),
        }
    }
}
impl From<Osr2> for u8 {
    fn from(val: Osr2) -> Self {
        match val {
            Osr2::Depth1 => 0,
            Osr2::Depth2 => 1,
            Osr2::Depth4 => 2,
            Osr2::Depth8 => 3,
            Osr2::Depth16 => 4,
            Osr2::CatchAll(num) => num,
        }
    }
}
///  Oversampling ratio
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osr1 {
    ///  Ratio 8 (lowest noise, highest power)
    Ratio8 = 0,
    Ratio4 = 1,
    Ratio2 = 2,
    ///  Ratio 1 (highest noise, lowest power)
    Ratio1 = 3,
}
impl core::convert::TryFrom<u8> for Osr1 {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ratio8),
            1 => Ok(Self::Ratio4),
            2 => Ok(Self::Ratio2),
            3 => Ok(Self::Ratio1),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "Osr1",
                })
            }
        }
    }
}
impl From<Osr1> for u8 {
    fn from(val: Osr1) -> Self {
        match val {
            Osr1::Ratio8 => 0,
            Osr1::Ratio4 => 1,
            Osr1::Ratio2 => 2,
            Osr1::Ratio1 => 3,
        }
    }
}
///  Operating mode (pass through Suspend when switching)
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    ///  Suspend mode (POR default, lowest power)
    Suspend = 0,
    ///  Normal mode
    Normal = 1,
    ///  Single mode (one measurement then auto Suspend)
    Single = 2,
    ///  Continuous mode
    Continuous = 3,
}
impl core::convert::TryFrom<u8> for Mode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Suspend),
            1 => Ok(Self::Normal),
            2 => Ok(Self::Single),
            3 => Ok(Self::Continuous),
            val => {
                Err(::device_driver::ConversionError {
                    source: val,
                    target: "Mode",
                })
            }
        }
    }
}
impl From<Mode> for u8 {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Suspend => 0,
            Mode::Normal => 1,
            Mode::Single => 2,
            Mode::Continuous => 3,
        }
    }
}
///  Output data rate
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Odr {
    Hz1 = 0,
    Hz10 = 1,
    Hz50 = 2,
    Hz100 = 3,
    Hz200 = 4,
    ///  Reserved values (5~7), hardware equivalent to 200 Hz
    CatchAll(u8) = 5,
}
impl From<u8> for Odr {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Hz1,
            1 => Self::Hz10,
            2 => Self::Hz50,
            3 => Self::Hz100,
            4 => Self::Hz200,
            val => Self::CatchAll(val),
        }
    }
}
impl From<Odr> for u8 {
    fn from(val: Odr) -> Self {
        match val {
            Odr::Hz1 => 0,
            Odr::Hz10 => 1,
            Odr::Hz50 => 2,
            Odr::Hz100 => 3,
            Odr::Hz200 => 4,
            Odr::CatchAll(num) => num,
        }
    }
}
///  Full-scale range (Gauss). Lower range → higher sensitivity → higher resolution
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    ///  ±32 Gauss
    G32 = 0,
    ///  ±16 Gauss
    G16 = 1,
    ///  ±8 Gauss
    G8 = 2,
    ///  Reserved (3), equivalent to ±32 Gauss
    CatchAll(u8) = 3,
}
impl From<u8> for Range {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::G32,
            1 => Self::G16,
            2 => Self::G8,
            val => Self::CatchAll(val),
        }
    }
}
impl From<Range> for u8 {
    fn from(val: Range) -> Self {
        match val {
            Range::G32 => 0,
            Range::G16 => 1,
            Range::G8 => 2,
            Range::CatchAll(num) => num,
        }
    }
}
///  Set/reset mode — controls whether offset is updated during measurement
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SetResetMode {
    ///  Set and reset both on (offset updated each measurement)
    SetAndResetOn = 0,
    ///  Set only on
    SetOnlyOn = 1,
    ///  Reserved
    CatchAll(u8) = 2,
    ///  Set and reset both off (no offset update)
    Off = 3,
}
impl From<u8> for SetResetMode {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::SetAndResetOn,
            1 => Self::SetOnlyOn,
            3 => Self::Off,
            val => Self::CatchAll(val),
        }
    }
}
impl From<SetResetMode> for u8 {
    fn from(val: SetResetMode) -> Self {
        match val {
            SetResetMode::SetAndResetOn => 0,
            SetResetMode::SetOnlyOn => 1,
            SetResetMode::CatchAll(num) => num,
            SetResetMode::Off => 3,
        }
    }
}
