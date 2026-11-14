//! A thermostat uses a simulated room temperature to control heating
//! with hysteresis.
//!
//! The thermostat's `temperature()` is its target, not a measured room
//! temperature. ind-io does not read sensors or provide a control strategy,
//! so a small simulation stands in for sensor input and the control logic is
//! application code. The controller switches heating on below the target
//! minus a tolerance band and off above the target plus that band.
//! Within the band it leaves the power state unchanged, to avoid rapid
//! switching. In the simulation, the thermostat being on means the heating runs.
//! Run it with `cargo run --example advanced_thermostat`.

use ind_io::mock::{MockClimate, MockPower};
use ind_io::traits::{ClimateControl, PowerControl};
use ind_io::{ClimateMode, DeviceError, Thermostat};

const TARGET_CELSIUS: f32 = 21.0;
const HYSTERESIS_BAND_CELSIUS: f32 = 0.5;
const HEATING_STEP_CELSIUS: f32 = 0.8;
const COOLING_STEP_CELSIUS: f32 = 0.5;

/// Switches heating on below the lower threshold and off above the
/// upper threshold. Within the band, the current power state is retained.
///
/// The helper works with any device that provides power and climate control.
fn control_step<D>(device: &mut D, room_celsius: f32, band: f32) -> Result<(), DeviceError>
where
    D: PowerControl + ClimateControl,
{
    let target = device.temperature();

    if room_celsius < target - band && !device.is_on() {
        device.turn_on()?;
    } else if room_celsius > target + band && device.is_on() {
        device.turn_off()?;
    }

    Ok(())
}

fn main() -> Result<(), DeviceError> {
    let mut thermostat = Thermostat::new(MockPower::new(), MockClimate::new());

    thermostat.set_mode(ClimateMode::Heat)?;
    thermostat.set_temperature(TARGET_CELSIUS)?;

    // A small room model: temperature rises while heating and falls while off.
    // These increments are illustrative, not a model of real heating equipment.
    let mut room_celsius = 17.0_f32;
    let mut hottest_celsius = room_celsius;
    let mut switch_ons = 0;

    for step in 0..24 {
        let was_on = thermostat.is_on();

        control_step(&mut thermostat, room_celsius, HYSTERESIS_BAND_CELSIUS)?;

        if thermostat.is_on() && !was_on {
            switch_ons += 1;
        }

        println!(
            "step {step:2}: room {room_celsius:5.1} C, heating {}",
            thermostat.is_on()
        );

        room_celsius += if thermostat.is_on() {
            HEATING_STEP_CELSIUS
        } else {
            -COOLING_STEP_CELSIUS
        };

        hottest_celsius = hottest_celsius.max(room_celsius);
    }

    println!("heating switched on {switch_ons} times, hottest {hottest_celsius:.1} C");

    assert!(switch_ons >= 2, "the loop should have cycled");
    assert!(
        hottest_celsius <= TARGET_CELSIUS + HYSTERESIS_BAND_CELSIUS + HEATING_STEP_CELSIUS,
        "room temperature exceeded the upper threshold by more than one simulated heating step"
    );

    // The control loop reads the configured setpoint but never changes it.
    assert_eq!(thermostat.temperature(), TARGET_CELSIUS);

    Ok(())
}
