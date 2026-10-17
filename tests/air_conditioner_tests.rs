use ind_io::mock::{MockClimate, MockPower, MockSpeed};
use ind_io::traits::{ClimateControl, PowerControl, SpeedControl};
use ind_io::{AirConditioner, ClimateMode, DeviceError, Speed, SwingMode};

fn new_air_conditioner() -> AirConditioner<MockPower, MockSpeed, MockClimate> {
    AirConditioner::new(MockPower::new(), MockSpeed::new(), MockClimate::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

fn set_high<S: SpeedControl>(device: &mut S) -> Result<(), DeviceError> {
    device.set_speed(Speed::High)
}

fn cool_to<C: ClimateControl>(device: &mut C, celsius: f32) -> Result<(), DeviceError> {
    device.set_temperature(celsius)?;
    device.set_mode(ClimateMode::Cool)
}

#[derive(Debug, PartialEq)]
struct Snapshot {
    on: bool,
    speed: Speed,
    temperature: f32,
    mode: ClimateMode,
    swing: SwingMode,
}

fn snapshot<D>(device: &D) -> Snapshot
where
    D: PowerControl + SpeedControl + ClimateControl,
{
    Snapshot {
        on: device.is_on(),
        speed: device.speed(),
        temperature: device.temperature(),
        mode: device.mode(),
        swing: device.swing(),
    }
}

#[test]
fn starts_off() {
    let ac = new_air_conditioner();
    assert!(!ac.is_on());
}

#[test]
fn turn_on_and_turn_off_change_power_state() {
    let mut ac = new_air_conditioner();
    ac.turn_on().unwrap();
    assert!(ac.is_on());
    ac.turn_off().unwrap();
    assert!(!ac.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut ac = new_air_conditioner();
    ac.toggle().unwrap();
    assert!(ac.is_on());
    ac.toggle().unwrap();
    assert!(!ac.is_on());
}

#[test]
fn reset_leaves_the_device_on_with_settings_unchanged() {
    let mut ac = new_air_conditioner();
    ac.set_speed(Speed::High).unwrap();
    ac.set_temperature(20.0).unwrap();
    ac.set_mode(ClimateMode::Cool).unwrap();
    ac.set_swing(SwingMode::Both).unwrap();

    let configured = snapshot(&ac);
    ac.reset().unwrap();
    assert_eq!(
        snapshot(&ac),
        Snapshot {
            on: true,
            ..configured
        }
    );
}

#[test]
fn speed_set_through_the_device_is_read_back() {
    let mut ac = new_air_conditioner();
    ac.set_speed(Speed::High).unwrap();
    assert_eq!(ac.speed(), Speed::High);
}

#[test]
fn temperature_set_through_the_device_is_read_back() {
    let mut ac = new_air_conditioner();
    ac.set_temperature(7.5).unwrap();
    assert_eq!(ac.temperature(), 7.5);
}

#[test]
fn mode_set_through_the_device_is_read_back() {
    let mut ac = new_air_conditioner();
    ac.set_mode(ClimateMode::Cool).unwrap();
    assert_eq!(ac.mode(), ClimateMode::Cool);
}

#[test]
fn swing_set_through_the_device_is_read_back() {
    let mut ac = new_air_conditioner();
    ac.set_swing(SwingMode::Both).unwrap();
    assert_eq!(ac.swing(), SwingMode::Both);
}

#[test]
fn setting_speed_and_climate_while_off_does_not_turn_it_on() {
    let mut ac = new_air_conditioner();
    ac.set_speed(Speed::Medium).unwrap();
    ac.set_temperature(19.0).unwrap();
    ac.set_mode(ClimateMode::Heat).unwrap();
    ac.set_swing(SwingMode::Vertical).unwrap();
    assert!(!ac.is_on());
}

#[test]
fn setting_speed_and_climate_while_on_does_not_turn_it_off() {
    let mut ac = new_air_conditioner();
    ac.turn_on().unwrap();
    ac.set_speed(Speed::Low).unwrap();
    ac.set_temperature(23.0).unwrap();
    ac.set_mode(ClimateMode::Dry).unwrap();
    ac.set_swing(SwingMode::Horizontal).unwrap();
    assert!(ac.is_on());
}

#[test]
fn turning_on_keeps_speed_and_climate_configured_while_off() {
    let mut ac = new_air_conditioner();
    ac.set_speed(Speed::Medium).unwrap();
    ac.set_temperature(19.0).unwrap();
    ac.set_mode(ClimateMode::Heat).unwrap();
    ac.set_swing(SwingMode::Vertical).unwrap();
    ac.turn_on().unwrap();
    assert_eq!(ac.speed(), Speed::Medium);
    assert_eq!(ac.temperature(), 19.0);
    assert_eq!(ac.mode(), ClimateMode::Heat);
    assert_eq!(ac.swing(), SwingMode::Vertical);
}

#[test]
fn turning_off_keeps_speed_and_climate_configured() {
    let mut ac = new_air_conditioner();
    ac.turn_on().unwrap();
    ac.set_speed(Speed::High).unwrap();
    ac.set_temperature(24.0).unwrap();
    ac.set_mode(ClimateMode::Cool).unwrap();
    ac.set_swing(SwingMode::Horizontal).unwrap();
    ac.turn_off().unwrap();
    assert_eq!(ac.speed(), Speed::High);
    assert_eq!(ac.temperature(), 24.0);
    assert_eq!(ac.mode(), ClimateMode::Cool);
    assert_eq!(ac.swing(), SwingMode::Horizontal);
}

#[test]
fn speed_off_does_not_turn_the_power_off() {
    let mut ac = new_air_conditioner();
    ac.turn_on().unwrap();
    ac.set_speed(Speed::High).unwrap();
    ac.set_speed(Speed::Off).unwrap();
    assert!(ac.is_on());
    assert_eq!(ac.speed(), Speed::Off);
}

#[test]
fn speed_and_climate_do_not_affect_each_other() {
    let mut ac = new_air_conditioner();
    ac.set_temperature(18.0).unwrap();
    ac.set_mode(ClimateMode::Cool).unwrap();
    ac.set_swing(SwingMode::Horizontal).unwrap();
    ac.set_speed(Speed::Turbo).unwrap();
    assert_eq!(ac.temperature(), 18.0);
    assert_eq!(ac.mode(), ClimateMode::Cool);
    assert_eq!(ac.swing(), SwingMode::Horizontal);

    ac.set_temperature(26.0).unwrap();
    assert_eq!(ac.speed(), Speed::Turbo);
}

#[test]
fn invalid_temperature_is_reported_and_changes_nothing() {
    let mut ac = new_air_conditioner();
    ac.turn_on().unwrap();
    ac.set_speed(Speed::High).unwrap();
    ac.set_temperature(21.0).unwrap();
    let mode = ac.mode();
    let swing = ac.swing();

    let result = ac.set_temperature(f32::NAN);

    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(ac.temperature(), 21.0);
    assert_eq!(ac.mode(), mode);
    assert_eq!(ac.swing(), swing);
    assert_eq!(ac.speed(), Speed::High);
    assert!(ac.is_on());
}

#[test]
fn power_and_speed_still_work_after_invalid_temperature() {
    let mut ac = new_air_conditioner();

    let result = ac.set_temperature(f32::NAN);
    assert_eq!(result, Err(DeviceError::InvalidInput));

    ac.turn_on().unwrap();
    assert!(ac.is_on());
    ac.set_speed(Speed::Low).unwrap();
    assert_eq!(ac.speed(), Speed::Low);
    ac.turn_off().unwrap();
    assert!(!ac.is_on());
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut ac = new_air_conditioner();
    power_on_any(&mut ac).unwrap();
    set_high(&mut ac).unwrap();
    cool_to(&mut ac, 18.0).unwrap();
    assert!(ac.is_on());
    assert_eq!(ac.speed(), Speed::High);
    assert_eq!(ac.temperature(), 18.0);
    assert_eq!(ac.mode(), ClimateMode::Cool);
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut ac = new_air_conditioner();

    let power: &mut dyn PowerControl = &mut ac;
    power.toggle().unwrap();
    assert!(power.is_on());

    let speed: &mut dyn SpeedControl = &mut ac;
    speed.set_speed(Speed::Low).unwrap();
    assert_eq!(speed.speed(), Speed::Low);

    let climate: &mut dyn ClimateControl = &mut ac;
    climate.set_mode(ClimateMode::Heat).unwrap();
    assert_eq!(climate.mode(), ClimateMode::Heat);
}
