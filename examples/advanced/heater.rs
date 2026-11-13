//! Application-defined temperature limits and a simulated daily schedule.
//!
//! The example combines power and climate control. Setting a target does
//! not switch the heater on or off. The application adds an upper limit of
//! 30 degrees Celsius; this is illustrative application policy, not a
//! recommended safety standard or a hardware safety guarantee. A target
//! rejected by the application rule or by the device leaves the previous
//! target unchanged. The schedule is simulated: it runs through the day's
//! entries without waiting for real time. Run it with
//! `cargo run --example advanced_heater`.

use ind_io::mock::{MockClimate, MockPower};
use ind_io::traits::{ClimateControl, PowerControl};
use ind_io::{DeviceError, Heater};

const MAX_TARGET_CELSIUS: f32 = 30.0;

#[derive(Debug, Clone, Copy)]
enum Action {
    Target(f32),
    On,
    Off,
}

/// Applies an application-defined upper limit before setting the target.
///
/// The device still validates the temperature according to its own
/// contract, including rejecting NaN and infinity.
fn set_target<H: ClimateControl>(heater: &mut H, celsius: f32) -> Result<(), DeviceError> {
    if celsius > MAX_TARGET_CELSIUS {
        return Err(DeviceError::InvalidInput);
    }

    heater.set_temperature(celsius)
}

/// Applies a scheduled action using only the required capabilities.
fn apply<H>(heater: &mut H, action: Action) -> Result<(), DeviceError>
where
    H: PowerControl + ClimateControl,
{
    match action {
        Action::Target(celsius) => set_target(heater, celsius),
        Action::On => heater.turn_on(),
        Action::Off => heater.turn_off(),
    }
}

fn main() -> Result<(), DeviceError> {
    // Illustrative schedule: (hour, action).
    // Two target entries are deliberately invalid.
    let schedule = [
        (6, Action::Target(21.0)),
        (6, Action::On),
        (8, Action::Off),
        (17, Action::Target(22.0)),
        (17, Action::Target(35.0)),     // Above the application limit.
        (17, Action::Target(f32::NAN)), // Rejected by the device.
        (18, Action::On),
        (23, Action::Off),
    ];

    let mut heater = Heater::new(MockPower::new(), MockClimate::new());
    let mut power_by_hour = Vec::new();
    let mut refused = 0;

    for hour in 0..24u8 {
        for (_, action) in schedule.iter().filter(|(h, _)| *h == hour) {
            match apply(&mut heater, *action) {
                Ok(()) => {}
                // Invalid schedule entries are reported and skipped.
                Err(DeviceError::InvalidInput) => {
                    refused += 1;
                    println!("{hour:02}:00 refused {action:?}");
                }
                // DeviceError is non-exhaustive: stop on any other error.
                Err(other) => return Err(other),
            }
        }

        if power_by_hour.last() != Some(&heater.is_on()) {
            println!(
                "{hour:02}:00 heater {}",
                if heater.is_on() { "on" } else { "off" }
            );
        }

        power_by_hour.push(heater.is_on());
    }

    assert_eq!(refused, 2);
    assert!(power_by_hour[6]); // Explicitly switched on.
    assert!(!power_by_hour[8]); // Explicitly switched off.
    assert!(!power_by_hour[17]); // Target changes did not affect power.
    assert!(power_by_hour[18]);
    assert!(!power_by_hour[23]);

    // Both rejected targets left the previously accepted target unchanged.
    assert_eq!(heater.temperature(), 22.0);

    Ok(())
}
