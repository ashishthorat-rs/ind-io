use ind_io::mock::MockPower;
use ind_io::traits::PowerControl;
use ind_io::{DeviceError, Switch};

fn main() -> Result<(), DeviceError> {
    let mut switch = Switch::new(MockPower::new());
    println!("on at start: {}", switch.is_on());
    assert!(!switch.is_on());

    switch.turn_on()?;
    println!("after turn_on: {}", switch.is_on());
    assert!(switch.is_on());

    switch.toggle()?;
    println!("after toggle: {}", switch.is_on());
    assert!(!switch.is_on());

    switch.reset()?;
    println!("after reset: {}", switch.is_on());
    assert!(switch.is_on());

    Ok(())
}
