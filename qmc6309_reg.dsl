config {
    type RegisterAddressType = u8;
    type DefaultByteOrder = LE;
    type DefaultBitOrder = LSB0;
    type DefmtFeature = "defmt";
}

// ==================== Device Identification ====================
/// Chip ID register — POR value 0x90 used to verify I²C communication
register ChipId {
    const ADDRESS = 0x00;
    const SIZE_BITS = 8;
    type Access = RO;
    const RESET_VALUE = 0x90;

    /// Chip ID: fixed to 0x90 for QMC6309
    chip_id: uint = 0..8,
},

// ==================== 3-Axis Magnetic Data Output ====================
block Data {
    const ADDRESS_OFFSET = 0x01;

    /// 16-bit signed axis data (two's complement, saturates at ±32768)
    /// Template register — use DataX / DataY / DataZ for actual access
    register AxisData {
        const ADDRESS = 0;
        const SIZE_BITS = 16;
        type Access = RO;
        const ALLOW_ADDRESS_OVERLAP = true;

        /// Raw magnetic sensor output, signed 16-bit
        value: int = 0..16,
    },
    /// X-axis @ 01H (LSB) + 02H (MSB)
    ref DataX = register AxisData { const ADDRESS = 0; },
    /// Y-axis @ 03H (LSB) + 04H (MSB)
    ref DataY = register AxisData { const ADDRESS = 2; },
    /// Z-axis @ 05H (LSB) + 06H (MSB)
    ref DataZ = register AxisData { const ADDRESS = 4; },
},

// ==================== Status Register ====================
/// Status Register 1 (09H)
///
/// DRDY and OVFL are cleared automatically after reading this register.
register Status1 {
    const ADDRESS = 0x09;
    const SIZE_BITS = 8;
    type Access = RO;
    const RESET_VALUE = 0x18;

    /// NVM data load complete
    nvm_load_done: bool = 4,
    /// NVM ready (1: non-volatile memory accessible)
    nvm_rdy: bool = 3,
    /// Self-test complete (1: self-test done, data readable)
    st_rdy: bool = 2,
    /// Data overflow (1: any axis output exceeds ±32000 LSB)
    ovfl: bool = 1,
    /// Data ready (1: new 3-axis data available, cleared on read)
    drdy: bool = 0,
},

// ==================== Control Registers ====================
block Control {
    const ADDRESS_OFFSET = 0x0A;

    /// Control Register 1 (0AH) — operating mode and oversampling ratio
    register Reg1 {
        const ADDRESS = 0;
        const SIZE_BITS = 8;

        /// Low-pass filter depth (higher = lower noise, higher power)
        osr2: uint as enum Osr2 {
            Depth1 = 0,
            Depth2 = 1,
            Depth4 = 2,
            Depth8 = 3,
            Depth16 = 4,
            /// Reserved values (5~7), hardware equivalent to Depth16
            _CatchAll = catch_all,
        } = 5..8,
        /// Oversampling ratio
        osr1: uint as enum Osr1 {
            /// Ratio 8 (lowest noise, highest power)
            Ratio8 = 0,
            Ratio4 = 1,
            Ratio2 = 2,
            /// Ratio 1 (highest noise, lowest power)
            Ratio1 = 3,
        } = 3..5,
        /// Operating mode (pass through Suspend when switching)
        mode: uint as enum Mode {
            /// Suspend mode (POR default, lowest power)
            Suspend = 0,
            /// Normal mode
            Normal = 1,
            /// Single mode (one measurement then auto Suspend)
            Single = 2,
            /// Continuous mode
            Continuous = 3,
        } = 0..2,
    },

    /// Control Register 2 (0BH) — soft reset, data rate, range, set/reset
    register Reg2 {
        const ADDRESS = 1;
        const SIZE_BITS = 8;

        /// Soft reset (set to 1 to trigger, must manually write 0 to clear)
        soft_rst: bool = 7,
        /// Output data rate
        odr: uint as enum Odr {
            Hz1 = 0,
            Hz10 = 1,
            Hz50 = 2,
            Hz100 = 3,
            Hz200 = 4,
            /// Reserved values (5~7), hardware equivalent to 200 Hz
            _CatchAll = catch_all,
        } = 4..7,
        /// Full-scale range (Gauss). Lower range → higher sensitivity → higher resolution
        rng: uint as enum Range {
            /// ±32 Gauss
            G32 = 0,
            /// ±16 Gauss
            G16 = 1,
            /// ±8 Gauss
            G8 = 2,
            /// Reserved (3), equivalent to ±32 Gauss
            _CatchAll = catch_all,
        } = 2..4,
        /// Set/reset mode — controls whether offset is updated during measurement
        set_reset_mode: uint as enum SetResetMode {
            /// Set and reset both on (offset updated each measurement)
            SetAndResetOn = 0,
            /// Set only on
            SetOnlyOn = 1,
            /// Reserved
            _CatchAll = catch_all,
            /// Set and reset both off (no offset update)
            Off = 3,
        } = 0..2,
    },

    /// Control Register 3 (0EH) — self-test enable
    /// Only valid in Continuous mode. Hardware clears automatically when done.
    register Reg3 {
        const ADDRESS = 4;
        const SIZE_BITS = 8;

        /// Self-test enable (1: start self-test)
        self_test: bool = 0,
    },
},

// ==================== Self-Test Data Output ====================
block SelfTest {
    const ADDRESS_OFFSET = 0x13;

    /// Self-test data (8-bit) template
    register StData {
        const ADDRESS = 0;
        const SIZE_BITS = 8;
        type Access = RO;
        const ALLOW_ADDRESS_OVERLAP = true;

        /// Self-test output value
        value: uint = 0..8,
    },
    /// X-axis self-test @ 13H
    ref StX = register StData { const ADDRESS = 0; },
    /// Y-axis self-test @ 14H
    ref StY = register StData { const ADDRESS = 1; },
    /// Z-axis self-test @ 15H
    ref StZ = register StData { const ADDRESS = 2; },
},
