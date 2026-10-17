use ind_io::mock::{MockClimate, MockPower};
use ind_io::traits::{ClimateControl, PowerControl};
use ind_io::{ClimateMode, DeviceError, Heater, SwingMode};

fn new_heater() -> Heater<MockPower, MockClimate> {
    Heater::new(MockPower::new(), MockClimate::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

fn heat_to<C: ClimateControl>(device: &mut C, celsius: f32) -> Result<(), DeviceError> {
    device.set_temperature(celsius)?;
    device.set_mode(ClimateMode::Heat)
}

#[test]
fn starts_off() {
    let heater = new_heater();
    assert!(!heater.is_on());
}

#[test]
fn turn_on_and_turn_off_change_power_state() {
    let mut heater = new_heater();
    heater.turn_on().unwrap();
    assert!(heater.is_on());
    heater.turn_off().unwrap();
    assert!(!heater.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut heater = new_heater();
    heater.toggle().unwrap();
    assert!(heater.is_on());
    heater.toggle().unwrap();
    assert!(!heater.is_on());
}

#[test]
fn reset_leaves_the_device_on_with_climate_unchanged() {
    let mut heater = new_heater();

    heater.set_temperature(24.0).unwrap();
    heater.set_mode(ClimateMode::Heat).unwrap();
    heater.set_swing(SwingMode::Vertical).unwrap();
    heater.reset().unwrap();

    assert!(heater.is_on());
    assert_eq!(heater.temperature(), 24.0);
    assert_eq!(heater.mode(), ClimateMode::Heat);
    assert_eq!(heater.swing(), SwingMode::Vertical);
}

#[test]
fn temperature_set_through_the_device_is_read_back() {
    let mut heater = new_heater();
    heater.set_temperature(26.5).unwrap();
    assert_eq!(heater.temperature(), 26.5);
}

#[test]
fn mode_set_through_the_device_is_read_back() {
    let mut heater = new_heater();
    heater.set_mode(ClimateMode::Heat).unwrap();
    assert_eq!(heater.mode(), ClimateMode::Heat);
}

#[test]
fn swing_set_through_the_device_is_read_back() {
    let mut heater = new_heater();
    heater.set_swing(SwingMode::Vertical).unwrap();
    assert_eq!(heater.swing(), SwingMode::Vertical);
}

#[test]
fn setting_climate_while_off_does_not_turn_it_on() {
    let mut heater = new_heater();
    heater.set_temperature(19.0).unwrap();
    heater.set_mode(ClimateMode::Heat).unwrap();
    heater.set_swing(SwingMode::Vertical).unwrap();
    assert!(!heater.is_on());
}

#[test]
fn setting_climate_while_on_does_not_turn_it_on() {
    let mut heater = new_heater();

    heater.turn_on().unwrap();
    heater.set_temperature(23.0).unwrap();
    heater.set_mode(ClimateMode::Heat).unwrap();
    heater.set_swing(SwingMode::Horizontal).unwrap();

    assert!(heater.is_on());
}

#[test]
fn turning_on_keeps_the_climate_configured_while_off() {
    let mut heater = new_heater();

    heater.set_temperature(19.0).unwrap();
    heater.set_mode(ClimateMode::Heat).unwrap();
    heater.set_swing(SwingMode::Vertical).unwrap();
    heater.turn_on().unwrap();

    assert_eq!(heater.temperature(), 19.0);
    assert_eq!(heater.mode(), ClimateMode::Heat);
    assert_eq!(heater.swing(), SwingMode::Vertical);
}

#[test]
fn turning_off_keeps_the_configured_climate() {
    let mut heater = new_heater();

    heater.turn_on().unwrap();
    heater.set_temperature(24.0).unwrap();
    heater.set_mode(ClimateMode::Heat).unwrap();
    heater.set_swing(SwingMode::Horizontal).unwrap();
    heater.turn_off().unwrap();

    assert_eq!(heater.temperature(), 24.0);
    assert_eq!(heater.mode(), ClimateMode::Heat);
    assert_eq!(heater.swing(), SwingMode::Horizontal);
}

#[test]
fn invalid_temperature_is_reported_and_changes_nothing() {
    let mut heater = new_heater();
    heater.turn_on().unwrap();
    heater.set_temperature(21.0).unwrap();
    let mode = heater.mode();
    let swing = heater.swing();

    let result = heater.set_temperature(f32::NAN);

    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(heater.temperature(), 21.0);
    assert_eq!(heater.mode(), mode);
    assert_eq!(heater.swing(), swing);
    assert!(heater.is_on());
}

#[test]
fn power_still_works_after_invalid_temperature() {
    let mut heater = new_heater();

    let result = heater.set_temperature(f32::NAN);
    assert_eq!(result, Err(DeviceError::InvalidInput));

    heater.turn_on().unwrap();
    assert!(heater.is_on());

    heater.turn_off().unwrap();
    assert!(!heater.is_on());
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut heater = new_heater();

    power_on_any(&mut heater).unwrap();
    heat_to(&mut heater, 22.5).unwrap();

    assert!(heater.is_on());
    assert_eq!(heater.temperature(), 22.5);
    assert_eq!(heater.mode(), ClimateMode::Heat);
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut heater = new_heater();

    let power: &mut dyn PowerControl = &mut heater;
    power.toggle().unwrap();
    assert!(power.is_on());

    let climate: &mut dyn ClimateControl = &mut heater;
    climate.set_mode(ClimateMode::Heat).unwrap();
    assert_eq!(climate.mode(), ClimateMode::Heat);
}
