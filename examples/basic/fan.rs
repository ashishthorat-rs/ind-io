use ind_io::mock::{MockPower, MockSpeed};
use ind_io::traits::{PowerControl, SpeedControl};
use ind_io::{DeviceError, Fan, Speed};

fn quiet_mode<D: PowerControl + SpeedControl>(device: &mut D) -> Result<(), DeviceError> {
    device.turn_on()?;
    device.set_speed(Speed::Low)
}

fn main() -> Result<(), DeviceError> {
    let mut fan = Fan::new(MockPower::new(), MockSpeed::new());

    fan.set_speed(Speed::Medium)?;
    println!("speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::Medium);
    assert!(!fan.is_on());

    fan.turn_on()?;
    fan.set_speed(Speed::High)?;
    println!("speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::High);
    assert!(fan.is_on());

    fan.set_speed(Speed::Off)?;
    println!("speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::Off);
    assert!(fan.is_on());

    fan.set_speed(Speed::Turbo)?;
    fan.turn_off()?;
    println!("speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::Turbo);
    assert!(!fan.is_on());

    quiet_mode(&mut fan)?;
    println!("quiet mode: speed: {:?}, on: {}", fan.speed(), fan.is_on());
    assert_eq!(fan.speed(), Speed::Low);
    assert!(fan.is_on());

    Ok(())
}
