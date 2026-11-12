//! Compose your own device from ind-io devices.
//!
//! The capability traits are sealed, so a type defined outside the crate
//! cannot implement them, and it cannot be passed to code bounded by them.
//! It can still own ind-io devices, call their capabilities, and pass those
//! devices to code bounded by the traits. Here, a custom ceiling fan owns a
//! `Fan` and `Light`. Each component retains its own power state and
//! configuration. The custom `night_mode` method coordinates both parts. It
//! is application logic, and it is not atomic. Run it with
//! `cargo run --example advanced_custom_device`.

use ind_io::mock::{MockLight, MockPower, MockSpeed};
use ind_io::traits::{LightControl, PowerControl, SpeedControl};
use ind_io::{DeviceError, Fan, Light, Speed};

/// A custom device composed of a fan and a light.
struct CeilingFan {
    fan: Fan<MockPower, MockSpeed>,
    light: Light<MockPower, MockLight>,
}

impl CeilingFan {
    fn new() -> Self {
        Self {
            fan: Fan::new(MockPower::new(), MockSpeed::new()),
            light: Light::new(MockPower::new(), MockLight::new()),
        }
    }

    /// Runs the fan at low speed and the light at 20 percent brightness.
    ///
    /// These are application-specific rules, not built-in device behavior.
    /// The operations run in order; if one fails, earlier operations are
    /// not automatically rolled back.
    fn night_mode(&mut self) -> Result<(), DeviceError> {
        self.fan.turn_on()?;
        self.fan.set_speed(Speed::Low)?;

        self.light.turn_on()?;
        self.light.set_brightness(20)?;

        Ok(())
    }

    /// Turns off both components without resetting their configurations.
    fn all_off(&mut self) -> Result<(), DeviceError> {
        self.fan.turn_off()?;
        self.light.turn_off()?;

        Ok(())
    }
}

/// Works with any device or component that implements `PowerControl`.
fn print_power(label: &str, part: &impl PowerControl) {
    println!("{label}: on={}", part.is_on());
}

fn main() -> Result<(), DeviceError> {
    let mut ceiling_fan = CeilingFan::new();

    // Apply the custom night mode.
    ceiling_fan.night_mode()?;

    print_power("fan", &ceiling_fan.fan);
    print_power("light", &ceiling_fan.light);

    assert!(ceiling_fan.fan.is_on());
    assert_eq!(ceiling_fan.fan.speed(), Speed::Low);
    assert!(ceiling_fan.light.is_on());
    assert_eq!(ceiling_fan.light.brightness(), 20);

    // Switching off the light does not affect the fan.
    // The light also retains its configured brightness.
    ceiling_fan.light.turn_off()?;

    print_power("fan", &ceiling_fan.fan);
    print_power("light", &ceiling_fan.light);

    assert!(ceiling_fan.fan.is_on());
    assert!(!ceiling_fan.light.is_on());
    assert_eq!(ceiling_fan.light.brightness(), 20);

    // Turn off both components while preserving their configurations.
    ceiling_fan.all_off()?;

    print_power("fan", &ceiling_fan.fan);
    print_power("light", &ceiling_fan.light);

    assert!(!ceiling_fan.fan.is_on());
    assert!(!ceiling_fan.light.is_on());
    assert_eq!(ceiling_fan.fan.speed(), Speed::Low);
    assert_eq!(ceiling_fan.light.brightness(), 20);

    Ok(())
}
