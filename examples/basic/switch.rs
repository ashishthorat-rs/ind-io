//! A switch has one capability: power.
//!
//! `toggle` and `reset` are default methods of `PowerControl`. `toggle` flips
//! the power state, and `reset` is a power cycle that ends with the device
//! on. Run it with `cargo run --example basic_switch`.

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

    // toggle flips the power state.
    switch.toggle()?;
    println!("after toggle: {}", switch.is_on());
    assert!(!switch.is_on());

    // reset is a power cycle. The switch is on afterwards.
    switch.reset()?;
    println!("after reset: {}", switch.is_on());
    assert!(switch.is_on());

    Ok(())
}
