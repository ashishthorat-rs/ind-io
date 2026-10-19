use ind_io::mock::{MockLight, MockPower};
use ind_io::traits::{LightControl, PowerControl};
use ind_io::{DeviceError, Light};

fn new_light() -> Light<MockPower, MockLight> {
    Light::new(MockPower::new(), MockLight::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

fn set_half<L: LightControl>(device: &mut L) -> Result<(), DeviceError> {
    device.set_brightness(50)
}

#[test]
fn starts_off() {
    let light = new_light();
    assert!(!light.is_on());
}

#[test]
fn turn_on_turn_off_change_power_state() {
    let mut light = new_light();
    light.turn_on().unwrap();
    assert!(light.is_on());
    light.turn_off().unwrap();
    assert!(!light.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut light = new_light();
    light.toggle().unwrap();
    assert!(light.is_on());
    light.toggle().unwrap();
    assert!(!light.is_on());
}

#[test]
fn reset_leaves_the_device_on_with_brightness_unchanged() {
    let mut light = new_light();
    light.set_brightness(70).unwrap();
    light.reset().unwrap();
    assert!(light.is_on());
    assert_eq!(light.brightness(), 70);
}

#[test]
fn brightness_set_through_the_device_is_read_back() {
    let mut light = new_light();
    light.set_brightness(75).unwrap();
    assert_eq!(light.brightness(), 75);
}

#[test]
fn setting_brightness_while_off_does_not_turn_it_on() {
    let mut light = new_light();
    light.set_brightness(50).unwrap();
    assert!(!light.is_on());
}

#[test]
fn setting_brightness_while_on_does_not_turn_it_off() {
    let mut light = new_light();
    light.turn_on().unwrap();
    light.set_brightness(30).unwrap();
    assert!(light.is_on());
}

#[test]
fn turning_on_keeps_the_brightness_configured_while_off() {
    let mut light = new_light();
    light.set_brightness(40).unwrap();
    light.turn_on().unwrap();
    assert_eq!(light.brightness(), 40);
}

#[test]
fn turning_off_keeps_the_configured_brightness() {
    let mut light = new_light();
    light.turn_on().unwrap();
    light.set_brightness(80).unwrap();
    light.turn_off().unwrap();
    assert_eq!(light.brightness(), 80);
}

#[test]
fn zero_brightness_does_not_turn_the_power_off() {
    let mut light = new_light();
    light.turn_on().unwrap();
    light.set_brightness(80).unwrap();
    light.set_brightness(0).unwrap();
    assert!(light.is_on());
    assert_eq!(light.brightness(), 0);
}

#[test]
fn invalid_brightness_is_reported_and_changes_nothing() {
    let mut light = new_light();

    light.turn_on().unwrap();
    light.set_brightness(40).unwrap();

    let result = light.set_brightness(101);

    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(light.brightness(), 40);
    assert!(light.is_on());
}

#[test]
fn power_still_works_after_invalid_brightness() {
    let mut light = new_light();

    let result = light.set_brightness(101);
    assert_eq!(result, Err(DeviceError::InvalidInput));

    light.turn_on().unwrap();
    assert!(light.is_on());

    light.turn_off().unwrap();
    assert!(!light.is_on());
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut light = new_light();

    power_on_any(&mut light).unwrap();
    set_half(&mut light).unwrap();
    assert!(light.is_on());
    assert_eq!(light.brightness(), 50);
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut light = new_light();

    let power: &mut dyn PowerControl = &mut light;
    power.toggle().unwrap();
    assert!(power.is_on());

    let brightness: &mut dyn LightControl = &mut light;
    brightness.set_brightness(30).unwrap();
    assert_eq!(brightness.brightness(), 30);
}
