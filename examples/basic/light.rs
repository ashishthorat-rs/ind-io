//! A light combines two capabilities: power and brightness.
//!
//! Brightness is a percentage from 0 to 100. A brightness of 0 is a level,
//! not an off state, and turning the light off keeps its brightness. A value
//! above 100 is rejected with `DeviceError::InvalidInput` and changes
//! nothing. Run it with `cargo run --example basic_light`.

use ind_io::mock::{MockLight, MockPower};
use ind_io::traits::{LightControl, PowerControl};
use ind_io::{DeviceError, Light};

fn main() -> Result<(), DeviceError> {
    let mut light = Light::new(MockPower::new(), MockLight::new());

    light.turn_on()?;
    light.set_brightness(75)?;
    println!("on: {}, brightness: {}%", light.is_on(), light.brightness());
    assert!(light.is_on());
    assert_eq!(light.brightness(), 75);

    // A brightness of 0 is a level. The light stays on.
    light.set_brightness(0)?;
    println!("on: {}, brightness: {}%", light.is_on(), light.brightness());
    assert!(light.is_on());
    assert_eq!(light.brightness(), 0);

    light.set_brightness(40)?;
    println!("on: {}, brightness: {}%", light.is_on(), light.brightness());
    assert!(light.is_on());
    assert_eq!(light.brightness(), 40);

    // A brightness above 100 is rejected, and the brightness does not change.
    let result = light.set_brightness(150);
    println!(
        "setting 150% gave {result:?}, brightness is still {}%",
        light.brightness()
    );
    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(light.brightness(), 40);

    // Turning the light off keeps the brightness.
    light.turn_off()?;
    println!("on: {}, brightness: {}%", light.is_on(), light.brightness());
    assert!(!light.is_on());
    assert_eq!(light.brightness(), 40);

    Ok(())
}
