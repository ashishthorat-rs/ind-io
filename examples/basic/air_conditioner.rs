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

    let result = ac.set_temperature(f32::NAN);
    println!("setting NaN gave {result:?}");
    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(ac.temperature(), 24.0);

    ac.turn_off()?;
    print_state("after turn_off", &ac);

    assert!(!ac.is_on());
    assert_eq!(ac.mode(), ClimateMode::Cool);
    assert_eq!(ac.temperature(), 24.0);
    assert_eq!(ac.speed(), Speed::Medium);
    assert_eq!(ac.swing(), SwingMode::Vertical);

    ac.reset()?;
    print_state("after reset", &ac);

    assert!(ac.is_on());
    assert_eq!(ac.mode(), ClimateMode::Cool);
    assert_eq!(ac.temperature(), 24.0);
    assert_eq!(ac.speed(), Speed::Medium);
    assert_eq!(ac.swing(), SwingMode::Vertical);

    Ok(())
}

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
