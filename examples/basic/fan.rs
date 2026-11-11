//! A fan combines two capabilities: power and speed.
//!
//! `Speed::Off` is a speed level, not a power state: a fan can be on at
//! `Speed::Off`, and turning the fan off keeps its speed. The mock accepts a
//! speed while the fan is off. That is a property of the mock, and a real
//! backend may refuse it. `quiet_mode` is generic over any device with power
//! and speed control. Run it with `cargo run --example basic_fan`.

use ind_io::mock::{MockPower, MockSpeed};
use ind_io::traits::{PowerControl, SpeedControl};
use ind_io::{DeviceError, Fan, Speed};

/// Turns on any device with power and speed control and sets a low speed.
fn quiet_mode<D: PowerControl + SpeedControl>(device: &mut D) -> Result<(), DeviceError> {
    device.turn_on()?;
    device.set_speed(Speed::Low)
}

fn main() -> Result<(), DeviceError> {
    let mut fan = Fan::new(MockPower::new(), MockSpeed::new());

    // The mock accepts a speed while the fan is off. Setting it does not turn
    // the fan on.
    fan.set_speed(Speed::Medium)?;
    println!("speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::Medium);
    assert!(!fan.is_on());

    fan.turn_on()?;
    fan.set_speed(Speed::High)?;
    println!("speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::High);
    assert!(fan.is_on());

    // Speed::Off is a speed level. The fan stays on.
    fan.set_speed(Speed::Off)?;
    println!("speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::Off);
    assert!(fan.is_on());

    // Turning the fan off keeps the speed.
    fan.set_speed(Speed::Turbo)?;
    fan.turn_off()?;
    println!("speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::Turbo);
    assert!(!fan.is_on());

    // The helper works with any device that has power and speed control.
    quiet_mode(&mut fan)?;
    println!("quiet mode: speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::Low);
    assert!(fan.is_on());

    Ok(())
}
