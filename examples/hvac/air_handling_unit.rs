//! A made-up air handling unit: a supply fan, an outside-air damper
//! and a heating coil.
//!
//! The components are ind-io devices built from mocks: a `Fan`, a `Curtain`
//! used for its position capability as the damper, and a `Heater` as the
//! coil. The example demonstrates how application code can coordinate their
//! capabilities; it does not model real HVAC equipment or validate a real
//! control sequence.
//!
//! ind-io devices do not know about one another, and the crate imposes
//! no startup or shutdown order. This application opens the damper before
//! starting the fan, enables heating only when the fan is on and its speed
//! is not `Speed::Off`, and shuts down in the reverse order.
//!
//! The airflow check is an application-defined precondition, not a safety
//! interlock. It checks the fan's reported state; it does not measure airflow
//! or continuously enforce the heating precondition.
//!
//! The coil's mode and target are set before it is switched on. The mock
//! accepts settings while a device is off. That is a property of the mock,
//! and a real backend may refuse it.
//!
//! Run it with `cargo run --example hvac_air_handling_unit`.

use std::fmt;

use ind_io::mock::{MockClimate, MockPosition, MockPower, MockSpeed};
use ind_io::traits::{ClimateControl, PositionControl, PowerControl, SpeedControl};
use ind_io::{ClimateMode, Curtain, DeviceError, Fan, Heater, Speed};

const SUPPLY_TEMPERATURE_CELSIUS: f32 = 30.0;

type SupplyFan = Fan<MockPower, MockSpeed>;
type OutsideAirDamper = Curtain<MockPower, MockPosition>;
type HeatingCoil = Heater<MockPower, MockClimate>;

#[derive(Debug)]
enum UnitError {
    Device(DeviceError),
    NoAirflow,
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Device(error) => write!(f, "device operation failed: {error}"),
            Self::NoAirflow => write!(f, "heating requires the supply fan to be running"),
        }
    }
}

impl From<DeviceError> for UnitError {
    fn from(error: DeviceError) -> Self {
        Self::Device(error)
    }
}

/// Reports whether the fan's configured state indicates airflow.
///
/// This checks power and speed state only. It does not measure airflow.
fn has_airflow(fan: &(impl PowerControl + SpeedControl)) -> bool {
    fan.is_on() && fan.speed() != Speed::Off
}

/// Opens the outside-air damper before starting the supply fan.
fn start_up(damper: &mut OutsideAirDamper, fan: &mut SupplyFan) -> Result<(), UnitError> {
    damper.turn_on()?;
    damper.set_position(100)?;

    fan.turn_on()?;
    fan.set_speed(Speed::Medium)?;

    Ok(())
}

/// Enables the heating coil only when the fan's state indicates airflow.
fn enable_heating(fan: &SupplyFan, coil: &mut HeatingCoil) -> Result<(), UnitError> {
    if !has_airflow(fan) {
        return Err(UnitError::NoAirflow);
    }

    coil.set_mode(ClimateMode::Heat)?;
    coil.set_temperature(SUPPLY_TEMPERATURE_CELSIUS)?;
    coil.turn_on()?;

    Ok(())
}

/// Shuts down the heating coil, fan and damper in that order.
fn shut_down(
    coil: &mut HeatingCoil,
    fan: &mut SupplyFan,
    damper: &mut OutsideAirDamper,
) -> Result<(), UnitError> {
    coil.turn_off()?;
    fan.turn_off()?;
    damper.set_position(0)?;
    damper.turn_off()?;

    Ok(())
}

fn print_refusal(result: Result<(), UnitError>) {
    if let Err(error) = result {
        println!("Heating refused: {error}");
    }
}

fn print_state(fan: &SupplyFan, damper: &OutsideAirDamper, coil: &HeatingCoil) {
    println!("Fan: on={}, speed={:?}", fan.is_on(), fan.speed());
    println!(
        "Outside-air damper: on={}, position={}%",
        damper.is_on(),
        damper.position()
    );
    println!(
        "Heating coil: on={}, mode={:?}, target={:.1} C",
        coil.is_on(),
        coil.mode(),
        coil.temperature(),
    );
}

fn main() -> Result<(), UnitError> {
    let mut fan = SupplyFan::new(MockPower::new(), MockSpeed::new());
    let mut damper = OutsideAirDamper::new(MockPower::new(), MockPosition::new());
    let mut coil = HeatingCoil::new(MockPower::new(), MockClimate::new());

    println!("1. Heating request while the unit is idle");
    print_refusal(enable_heating(&fan, &mut coil));
    assert!(!coil.is_on());

    println!("\n2. Start the air handling unit");
    start_up(&mut damper, &mut fan)?;

    assert!(damper.is_on());
    assert_eq!(damper.position(), 100);
    assert!(fan.is_on());
    assert_eq!(fan.speed(), Speed::Medium);

    print_state(&fan, &damper, &coil);

    println!("\n3. Refuse heating when the fan is stopped or its speed is Off");
    fan.turn_off()?;

    // The fan keeps its speed while stopped, so only its power state differs.
    assert_eq!(fan.speed(), Speed::Medium);
    print_refusal(enable_heating(&fan, &mut coil));
    assert!(!coil.is_on());

    fan.turn_on()?;
    fan.set_speed(Speed::Off)?;

    print_refusal(enable_heating(&fan, &mut coil));
    assert!(!coil.is_on());

    println!("\n4. Restore fan speed and enable heating");
    fan.set_speed(Speed::Medium)?;
    enable_heating(&fan, &mut coil)?;

    assert!(coil.is_on());
    assert_eq!(coil.mode(), ClimateMode::Heat);
    assert_eq!(coil.temperature(), SUPPLY_TEMPERATURE_CELSIUS);

    print_state(&fan, &damper, &coil);

    println!("\n5. Shut down the air handling unit");
    shut_down(&mut coil, &mut fan, &mut damper)?;

    assert!(!coil.is_on());
    assert!(!fan.is_on());
    assert!(!damper.is_on());
    assert_eq!(damper.position(), 0);

    // Power state and configured values are independent.
    assert_eq!(fan.speed(), Speed::Medium);
    assert_eq!(coil.temperature(), SUPPLY_TEMPERATURE_CELSIUS);

    print_state(&fan, &damper, &coil);

    Ok(())
}
