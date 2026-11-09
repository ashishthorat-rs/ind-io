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

    light.set_brightness(0)?;
    println!("on: {}, brightness: {}%", light.is_on(), light.brightness());
    assert!(light.is_on());
    assert_eq!(light.brightness(), 0);

    light.set_brightness(40)?;
    println!("on: {}, brightness: {}%", light.is_on(), light.brightness());
    assert!(light.is_on());
    assert_eq!(light.brightness(), 40);

    let result = light.set_brightness(150);
    println!(
        "setting 150% gave {result:?}, brightness is still {}%",
        light.brightness()
    );
    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(light.brightness(), 40);

    light.turn_off()?;
    println!("on: {}, brightness: {}%", light.is_on(), light.brightness());
    assert!(!light.is_on());
    assert_eq!(light.brightness(), 40);

    Ok(())
}
