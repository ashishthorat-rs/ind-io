//! An air purifier combines two capabilities: power and speed.
//!
//! The example runs an auto mode over made-up air-quality readings. ind-io
//! provides the capability to set and read the speed; choosing it is
//! application logic. The readings and thresholds are illustrative, not
//! sensor data or a recommended control standard. The auto mode raises the
//! speed immediately and steps down only after two consecutive lower
//! readings. `Speed::Off` is a speed level, not a power state, so the
//! purifier stays on. Run it with
//! `cargo run --example advanced_air_purifier`.

use std::cmp::Ordering;

use ind_io::mock::{MockPower, MockSpeed};
use ind_io::traits::{PowerControl, SpeedControl};
use ind_io::{AirPurifier, DeviceError, Speed};

/// Chooses a fan speed for an air-quality index (higher is worse).
fn speed_for(aqi: u16) -> Speed {
    match aqi {
        0..=50 => Speed::Off,
        51..=100 => Speed::Low,
        101..=150 => Speed::Medium,
        151..=200 => Speed::High,
        _ => Speed::Turbo,
    }
}

/// Orders speeds from quietest to loudest.
fn rank(speed: Speed) -> u8 {
    match speed {
        Speed::Off => 0,
        Speed::Low => 1,
        Speed::Medium => 2,
        Speed::High => 3,
        Speed::Turbo => 4,
        _ => 5, // Rank any future Speed variant above the currently known variants.
    }
}

/// Increases speed immediately but waits for two consecutive lower readings
/// before stepping down, preventing rapid speed changes from fluctuating data.
struct AutoMode {
    lower_readings: u8,
}

impl AutoMode {
    fn new() -> Self {
        Self { lower_readings: 0 }
    }

    fn update<D: SpeedControl>(&mut self, device: &mut D, aqi: u16) -> Result<(), DeviceError> {
        let wanted = speed_for(aqi);

        match rank(wanted).cmp(&rank(device.speed())) {
            Ordering::Greater => {
                device.set_speed(wanted)?;
                self.lower_readings = 0;
            }
            Ordering::Less => {
                self.lower_readings = self.lower_readings.saturating_add(1);

                if self.lower_readings >= 2 {
                    device.set_speed(wanted)?;
                    self.lower_readings = 0;
                }
            }
            Ordering::Equal => {
                self.lower_readings = 0;
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), DeviceError> {
    let mut purifier = AirPurifier::new(MockPower::new(), MockSpeed::new());

    purifier.turn_on()?;

    let mut auto = AutoMode::new();

    // Illustrative readings: clean air, a pollution spike, then recovery.
    let readings = [30, 45, 120, 180, 260, 190, 140, 90, 40, 35];
    let mut speeds = Vec::new();

    for aqi in readings {
        auto.update(&mut purifier, aqi)?;

        let speed = purifier.speed();
        println!("AQI {aqi:3} -> {speed:?}");
        speeds.push(speed);
    }

    assert_eq!(
        speeds,
        [
            Speed::Off,
            Speed::Off,
            Speed::Medium, // Increases immediately.
            Speed::High,
            Speed::Turbo,
            Speed::Turbo,  // First lower reading: hold.
            Speed::Medium, // Second lower reading: step down.
            Speed::Medium, // First lower reading: hold.
            Speed::Off,    // Second lower reading: step down.
            Speed::Off,
        ]
    );

    // Speed::Off is a speed level, not a power state.
    // The purifier remains powered on throughout the sequence.
    assert!(purifier.is_on());

    Ok(())
}
