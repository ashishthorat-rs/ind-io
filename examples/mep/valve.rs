//! A bypass transfer between two valves.
//!
//! The crate has no valve type, so the example uses a `Curtain`, built from
//! mock backends, for its power and position capabilities. A position of 0
//! is closed and 100 is fully open. The example models a main valve and a
//! bypass valve in parallel. It does not model real valves or pipework.
//!
//! The transfer moves the opening from one valve to the other in steps. The
//! valve that is opening moves first, so the combined opening never drops
//! below its starting value. The combined opening is a made-up capacity
//! measure, not a flow. The example records both positions after every
//! operation, so it can assert the order of the operations.
//!
//! A valve's position is the last position the device accepted, not a
//! measurement of the real valve. Closed does not mean de-energized: a valve
//! can be closed with its actuator powered, and switching the actuator off
//! leaves the position unchanged.
//!
//! Run it with `cargo run --example mep_valve`.

use ind_io::mock::{MockPosition, MockPower};
use ind_io::traits::{PositionControl, PowerControl};
use ind_io::{Curtain, DeviceError};

type Valve = Curtain<MockPower, MockPosition>;

/// Opening moved in each transfer step, in percent.
const STEP_PERCENT: u8 = 25;

/// Combined opening the pair must keep during a transfer, in percent.
const REQUIRED_PERCENT: u16 = 100;

/// Combined opening of two valves, in percent. A made-up capacity measure.
fn combined_percent(first: u8, second: u8) -> u16 {
    u16::from(first) + u16::from(second)
}

/// Moves the whole opening of `from` onto `to` in steps of `STEP_PERCENT`.
///
/// In each step the valve that is opening moves first. `to` must have room
/// for the opening of `from`. Otherwise the device rejects the position with
/// `DeviceError::InvalidInput` and the transfer stops part way, because the
/// operations are sequential and not atomic. Returns the positions of `from`
/// and `to` after every operation.
fn transfer(
    from: &mut impl PositionControl,
    to: &mut impl PositionControl,
) -> Result<Vec<(u8, u8)>, DeviceError> {
    let mut steps = Vec::new();

    while !from.is_closed() {
        let step = STEP_PERCENT.min(from.position());

        to.set_position(to.position() + step)?;
        steps.push((from.position(), to.position()));

        from.set_position(from.position() - step)?;
        steps.push((from.position(), to.position()));
    }

    Ok(steps)
}

fn print_steps(from_name: &str, to_name: &str, steps: &[(u8, u8)]) {
    for (number, (from, to)) in steps.iter().enumerate() {
        println!(
            "  operation {}: {from_name} {from:>3}%, {to_name} {to:>3}%, combined {}%",
            number + 1,
            combined_percent(*from, *to)
        );
    }
}

fn assert_opening_kept(steps: &[(u8, u8)]) {
    for &(from, to) in steps {
        assert!(combined_percent(from, to) >= REQUIRED_PERCENT);
    }
}

fn main() -> Result<(), DeviceError> {
    let mut main_valve = Valve::new(MockPower::new(), MockPosition::new());
    let mut bypass = Valve::new(MockPower::new(), MockPosition::new());

    // Both actuators are powered. The main valve carries the flow, and the
    // bypass is closed. A closed valve can still be powered.
    main_valve.turn_on()?;
    bypass.turn_on()?;
    main_valve.open()?;

    assert!(main_valve.is_open());
    assert!(bypass.is_closed());
    assert!(bypass.is_on());

    println!("Take the main valve out of service");
    let steps = transfer(&mut main_valve, &mut bypass)?;
    print_steps("main", "bypass", &steps);

    // The bypass opens by one step before the main valve closes by one step.
    assert_eq!(
        steps,
        [
            (100, 25),
            (75, 25),
            (75, 50),
            (50, 50),
            (50, 75),
            (25, 75),
            (25, 100),
            (0, 100),
        ]
    );
    assert_opening_kept(&steps);
    assert!(main_valve.is_closed());
    assert!(bypass.is_open());

    // Closed is not de-energized: switching the actuator off keeps the
    // position, and the bypass is not affected.
    main_valve.turn_off()?;

    println!(
        "Main valve: on={}, position={}%",
        main_valve.is_on(),
        main_valve.position()
    );
    assert!(!main_valve.is_on());
    assert!(main_valve.is_closed());
    assert!(bypass.is_open());

    println!("\nReturn the main valve to service");
    main_valve.turn_on()?;
    assert!(main_valve.is_closed());

    let steps = transfer(&mut bypass, &mut main_valve)?;
    print_steps("bypass", "main", &steps);

    assert_opening_kept(&steps);
    assert!(main_valve.is_on());
    assert!(main_valve.is_open());
    assert!(bypass.is_on());
    assert!(bypass.is_closed());

    Ok(())
}
