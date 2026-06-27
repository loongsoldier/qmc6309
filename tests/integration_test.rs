//! QMC6309 集成测试 — 初始化流程和数据读取

use embedded_hal_mock::eh1::i2c::{Mock as MockI2c, Transaction};
use qmc6309::{Blocking, Config, Error, Qmc6309, Qmc6309I2cInterface};

const ADDR: u8 = 0x1A;

/// `Qmc6309::new()` + `chip_id()` 的完整 I²C 事务序列
fn mock_new_transactions() -> Vec<Transaction> {
    vec![
        // soft_reset
        Transaction::write(ADDR, vec![0x0B, 0x80]),
        Transaction::write(ADDR, vec![0x0B, 0x00]),
        // verify_device
        Transaction::write_read(ADDR, vec![0x00], vec![0x90]),
        // write_config Reg1 (modify: read 0x0A → write)
        Transaction::write_read(ADDR, vec![0x0A], vec![0x00]),
        Transaction::write(ADDR, vec![0x0A, 0x81]),
        // write_config Reg2
        Transaction::write_read(ADDR, vec![0x0B], vec![0x00]),
        Transaction::write(ADDR, vec![0x0B, 0x48]),
    ]
}

#[test]
fn test_new_and_chip_id() {
    let mut transactions = mock_new_transactions();
    transactions.push(Transaction::write_read(ADDR, vec![0x00], vec![0x90]));

    let mock = MockI2c::new(&transactions);
    let interface = Qmc6309I2cInterface::new(mock);
    let result = Qmc6309::<_, Blocking>::new(interface, Config::default());
    let mut sensor = match result {
        Ok(s) => s,
        Err(_) => panic!("new() failed"),
    };

    assert_eq!(sensor.chip_id().unwrap(), 0x90);
    sensor.release().bus.done();
}

#[test]
fn test_bad_chip_id_is_error_and_returns_interface() {
    let transactions = vec![
        Transaction::write(ADDR, vec![0x0B, 0x80]),
        Transaction::write(ADDR, vec![0x0B, 0x00]),
        Transaction::write_read(ADDR, vec![0x00], vec![0x33]),
    ];
    let mock = MockI2c::new(&transactions);
    let interface = Qmc6309I2cInterface::new(mock);
    let result = Qmc6309::<_, Blocking>::new(interface, Config::default());

    match result {
        Err((Error::InvalidChipId(id), mut iface)) => {
            assert_eq!(id, 0x33);
            iface.bus.done();
        }
        _other => panic!("unexpected result"),
    }
}

#[test]
fn test_read_gauss() {
    let mut transactions = mock_new_transactions();
    // X @ 0x01: 0x1234 LE → i16 = 4660
    transactions.push(Transaction::write_read(ADDR, vec![0x01], vec![0x34, 0x12]));
    // Y @ 0x03: 0x0000
    transactions.push(Transaction::write_read(ADDR, vec![0x03], vec![0x00, 0x00]));
    // Z @ 0x05: 0xF000 LE → i16 = -4096
    transactions.push(Transaction::write_read(ADDR, vec![0x05], vec![0x00, 0xF0]));

    let mock = MockI2c::new(&transactions);
    let interface = Qmc6309I2cInterface::new(mock);
    let result = Qmc6309::<_, Blocking>::new(interface, Config::default());
    let mut sensor = match result {
        Ok(s) => s,
        Err(_) => panic!("new() failed"),
    };

    let mag = sensor.read_gauss().unwrap();
    // ±8G: x=4660*8/32768≈1.14, z=-4096*8/32768=-1.0
    assert!((mag.x - 1.1377).abs() < 0.01);
    assert!((mag.y - 0.0).abs() < 0.01);
    assert!((mag.z + 1.0).abs() < 0.01);
    sensor.release().bus.done();
}
