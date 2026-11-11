//! An air conditioner combines three capabilities: power, speed and climate.
//!
//! The example configures the mode, temperature, speed and swing, then shows
//! that a rejected temperature changes nothing and that `turn_off` and `reset`
//! keep the configuration. `reset` is a power cycle: on success it ends with
//! the device on. Run it with `cargo run --example basic_air_conditioner`.

use ind_io::mock::{MockClimate, MockPower, MockSpeed};
use ind_io::traits::{ClimateControl, PowerControl, SpeedControl};
use ind_io::{AirConditioner, ClimateMode, DeviceError, Speed, SwingMode};

fn main() -> Result<(), DeviceError> {
    let mut ac = AirConditioner::new(MockPower::new(), MockSpeed::new(), MockClimate::new());

    ac.turn_on()?;
    ac.set_mode(ClimateMode::Cool)?;
    ac.set_temperature(24.0)?;
    ac.set_speed(Speed::Medium)?;
    ac.set_swing(SwingMode::Vertical)?;
    print_state("configured", &ac);

    assert!(ac.is_on());
    assert_eq!(ac.mode(), ClimateMode::Cool);
    assert_eq!(ac.temperature(), 24.0);
    assert_eq!(ac.speed(), Speed::Medium);
    assert_eq!(ac.swing(), SwingMode::Vertical);

    // A temperature that is not a number is rejected, and nothing changes.
    let result = ac.set_temperature(f32::NAN);
    println!("setting NaN gave {result:?}");
    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(ac.temperature(), 24.0);

    // Turning the device off keeps its configuration.
    ac.turn_off()?;
    print_state("after turn_off", &ac);

    assert!(!ac.is_on());
    assert_eq!(ac.mode(), ClimateMode::Cool);
    assert_eq!(ac.temperature(), 24.0);
    assert_eq!(ac.speed(), Speed::Medium);
    assert_eq!(ac.swing(), SwingMode::Vertical);

    // reset is a power cycle. It ends with the device on and the
    // configuration unchanged.
    ac.reset()?;
    print_state("after reset", &ac);

    assert!(ac.is_on());
    assert_eq!(ac.mode(), ClimateMode::Cool);
    assert_eq!(ac.temperature(), 24.0);
    assert_eq!(ac.speed(), Speed::Medium);
    assert_eq!(ac.swing(), SwingMode::Vertical);

    Ok(())
}

// Prints the state of any device with power, climate and speed control.
fn print_state(label: &str, ac: &(impl PowerControl + ClimateControl + SpeedControl)) {
    println!(
        "{label}: on={}, mode={:?}, {:.1} C, speed={:?}, swing={:?}",
        ac.is_on(),
        ac.mode(),
        ac.temperature(),
        ac.speed(),
        ac.swing()
    );
}
