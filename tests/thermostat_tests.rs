use ind_io::mock::{MockClimate, MockPower};
use ind_io::traits::{ClimateControl, PowerControl};
use ind_io::{ClimateMode, DeviceError, SwingMode, Thermostat};

fn new_thermostat() -> Thermostat<MockPower, MockClimate> {
    Thermostat::new(MockPower::new(), MockClimate::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

fn cool_to<C: ClimateControl>(device: &mut C, celsius: f32) -> Result<(), DeviceError> {
    device.set_temperature(celsius)?;
    device.set_mode(ClimateMode::Cool)
}

#[test]
fn starts_off() {
    let thermostat = new_thermostat();
    assert!(!thermostat.is_on());
}

#[test]
fn turn_on_and_turn_off_change_power_state() {
    let mut thermostat = new_thermostat();
    thermostat.turn_on().unwrap();
    assert!(thermostat.is_on());
    thermostat.turn_off().unwrap();
    assert!(!thermostat.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut thermostat = new_thermostat();
    thermostat.toggle().unwrap();
    assert!(thermostat.is_on());
    thermostat.toggle().unwrap();
    assert!(!thermostat.is_on());
}

#[test]
fn reset_leaves_the_device_on_with_climate_unchanged() {
    let mut thermostat = new_thermostat();

    thermostat.set_temperature(20.0).unwrap();
    thermostat.set_mode(ClimateMode::Cool).unwrap();
    thermostat.set_swing(SwingMode::Both).unwrap();
    thermostat.reset().unwrap();

    assert!(thermostat.is_on());
    assert_eq!(thermostat.temperature(), 20.0);
    assert_eq!(thermostat.mode(), ClimateMode::Cool);
    assert_eq!(thermostat.swing(), SwingMode::Both);
}

#[test]
fn temperature_set_through_the_device_is_read_back() {
    let mut thermostat = new_thermostat();
    thermostat.set_temperature(7.5).unwrap();
    assert_eq!(thermostat.temperature(), 7.5);
}

#[test]
fn mode_set_through_the_device_is_read_back() {
    let mut thermostat = new_thermostat();
    thermostat.set_mode(ClimateMode::Cool).unwrap();
    assert_eq!(thermostat.mode(), ClimateMode::Cool);
}

#[test]
fn swing_set_through_the_device_is_read_back() {
    let mut thermostat = new_thermostat();
    thermostat.set_swing(SwingMode::Both).unwrap();
    assert_eq!(thermostat.swing(), SwingMode::Both);
}

#[test]
fn setting_climate_while_off_does_not_turn_it_on() {
    let mut thermostat = new_thermostat();

    thermostat.set_temperature(19.0).unwrap();
    thermostat.set_mode(ClimateMode::Heat).unwrap();
    thermostat.set_swing(SwingMode::Vertical).unwrap();

    assert!(!thermostat.is_on());
}

#[test]
fn setting_climate_while_on_does_not_turn_it_off() {
    let mut thermostat = new_thermostat();

    thermostat.turn_on().unwrap();
    thermostat.set_temperature(23.0).unwrap();
    thermostat.set_mode(ClimateMode::Dry).unwrap();
    thermostat.set_swing(SwingMode::Horizontal).unwrap();

    assert!(thermostat.is_on());
}

#[test]
fn turning_on_keeps_the_climate_configured_while_off() {
    let mut thermostat = new_thermostat();

    thermostat.set_temperature(19.0).unwrap();
    thermostat.set_mode(ClimateMode::Heat).unwrap();
    thermostat.set_swing(SwingMode::Vertical).unwrap();
    thermostat.turn_on().unwrap();

    assert_eq!(thermostat.temperature(), 19.0);
    assert_eq!(thermostat.mode(), ClimateMode::Heat);
    assert_eq!(thermostat.swing(), SwingMode::Vertical);
}

#[test]
fn turning_off_keeps_the_configured_climate() {
    let mut thermostat = new_thermostat();

    thermostat.turn_on().unwrap();
    thermostat.set_temperature(24.0).unwrap();
    thermostat.set_mode(ClimateMode::Cool).unwrap();
    thermostat.set_swing(SwingMode::Horizontal).unwrap();
    thermostat.turn_off().unwrap();

    assert_eq!(thermostat.temperature(), 24.0);
    assert_eq!(thermostat.mode(), ClimateMode::Cool);
    assert_eq!(thermostat.swing(), SwingMode::Horizontal);
}

#[test]
fn invalid_temperature_is_reported_and_changes_nothing() {
    let mut thermostat = new_thermostat();
    thermostat.turn_on().unwrap();
    thermostat.set_temperature(21.0).unwrap();
    let mode = thermostat.mode();
    let swing = thermostat.swing();

    let result = thermostat.set_temperature(f32::NAN);
    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(thermostat.temperature(), 21.0);
    assert_eq!(thermostat.mode(), mode);
    assert_eq!(thermostat.swing(), swing);
    assert!(thermostat.is_on());
}

#[test]
fn power_still_works_after_invalid_temperature() {
    let mut thermostat = new_thermostat();

    let result = thermostat.set_temperature(f32::NAN);
    assert_eq!(result, Err(DeviceError::InvalidInput));

    thermostat.turn_on().unwrap();
    assert!(thermostat.is_on());

    thermostat.turn_off().unwrap();
    assert!(!thermostat.is_on());
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut thermostat = new_thermostat();

    power_on_any(&mut thermostat).unwrap();
    cool_to(&mut thermostat, 18.0).unwrap();

    assert!(thermostat.is_on());
    assert_eq!(thermostat.temperature(), 18.0);
    assert_eq!(thermostat.mode(), ClimateMode::Cool);
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut thermostat = new_thermostat();

    let power: &mut dyn PowerControl = &mut thermostat;
    power.toggle().unwrap();
    assert!(power.is_on());

    let climate: &mut dyn ClimateControl = &mut thermostat;
    climate.set_mode(ClimateMode::Heat).unwrap();
    assert_eq!(climate.mode(), ClimateMode::Heat);
}
