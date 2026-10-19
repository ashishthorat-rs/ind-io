use ind_io::mock::MockPower;
use ind_io::traits::PowerControl;
use ind_io::{DeviceError, Switch};

fn new_switch() -> Switch<MockPower> {
    Switch::new(MockPower::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

#[test]
fn starts_off() {
    let switch = new_switch();
    assert!(!switch.is_on())
}

#[test]
fn turn_on_and_turn_off_change_power_state() {
    let mut switch = new_switch();
    switch.turn_on().unwrap();
    assert!(switch.is_on());
    switch.turn_off().unwrap();
    assert!(!switch.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut switch = new_switch();
    switch.toggle().unwrap();
    assert!(switch.is_on());
    switch.toggle().unwrap();
    assert!(!switch.is_on());
}

#[test]
fn reset_leaves_the_device_on_from_either_state() {
    let mut starting_off = Switch::new(MockPower::with_state(false));
    starting_off.reset().unwrap();
    assert!(starting_off.is_on());

    let mut starting_on = Switch::new(MockPower::with_state(true));
    starting_on.reset().unwrap();
    assert!(starting_on.is_on());
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut switch = new_switch();
    power_on_any(&mut switch).unwrap();
    assert!(switch.is_on());
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut switch = new_switch();
    let power: &mut dyn PowerControl = &mut switch;
    power.toggle().unwrap();
    assert!(power.is_on());

    power.reset().unwrap();
    assert!(power.is_on());
}
